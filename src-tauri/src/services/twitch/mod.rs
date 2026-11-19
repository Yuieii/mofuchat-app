use std::{collections::HashMap, sync::Arc, time::Duration};

use async_trait::async_trait;
use cached::cached;
use chrono::{DateTime, Local, TimeDelta};
use futures_util::TryStreamExt;
use keyring::Entry;
use serde::{Deserialize, Serialize};
use tauri::AppHandle;
use tokio::sync::{Mutex, RwLock};

use twitch_api::HelixClient;
use twitch_api::common::chat::Fragment;
use twitch_api::eventsub::channel::chat::message::Badge;
use twitch_api::eventsub::channel::*;
use twitch_api::eventsub::*;
use twitch_api::helix::Scope;
use twitch_api::helix::bits::GetCheermotesRequest;
use twitch_api::helix::bits::get_cheermotes::CheermoteImage;
use twitch_api::helix::chat::{BadgeSet, ChatBadge, GetChannelChatBadgesRequest, GetGlobalChatBadgesRequest};
use twitch_api::types::{EmoteAnimationSetting, EmoteUrlBuilder, EventSubId};

use twitch_api::{
    twitch_oauth2::{
        AccessToken, ClientId, ClientSecret, DeviceUserTokenBuilder, RefreshToken, TwitchToken,
        UserToken, client::Client,
    },
    types::Nickname,
};

use eyre::Result;
use reqwest_websocket::Upgrade;

use crate::comments::{CommentPlatform, CommonCommentData, UserBadge};
use crate::{
    comments::{Comment, CommentData, CommentRenderer},
    services::{ServiceEvent, adapter::ServiceAdapter},
    state::ManagerExt,
};

const CLIENT_ID: &str = env!("TWITCH_CLIENT_ID");

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserTokenModel {
    pub access_token: String,
    pub refresh_token: String,
}

impl UserTokenModel {
    pub fn new(source: &UserToken) -> Self {
        Self {
            access_token: source.access_token.secret().into(),
            refresh_token: source
                .refresh_token
                .as_ref()
                .expect("No refresh token available")
                .secret()
                .into(),
        }
    }

    async fn try_into_token<C: Client>(self, client: &C) -> Result<UserToken> {
        let token = UserToken::from_existing_or_refresh_token(
            client,
            AccessToken::new(self.access_token),
            RefreshToken::new(self.refresh_token),
            ClientId::new(CLIENT_ID.into()),
            None,
        )
        .await?;
        Ok(token)
    }
}

pub struct TwitchServiceAdapter {
    client: HelixClient<'static, reqwest::Client>,
    authenticate_lock: Mutex<Option<UserToken>>,
    websocket_session: Arc<RwLock<Option<String>>>,
    subscriptions: Arc<RwLock<HashMap<String, Vec<EventSubId>>>>,
    global_badges: Mutex<Option<(Vec<BadgeSet>, DateTime<Local>)>>,
}

impl TwitchServiceAdapter {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            client: HelixClient::default(),
            authenticate_lock: Mutex::new(None),
            websocket_session: Arc::new(RwLock::new(None)),
            subscriptions: Arc::new(RwLock::new(HashMap::new())),
            global_badges: Mutex::new(None),
        })
    }

    fn extract_login(url: &str) -> Option<String> {
        url.trim()
            .trim_end_matches('/')
            .split("twitch.tv/")
            .nth(1)?
            .split(&['/', '?'][..])
            .next()
            .filter(|s| !s.is_empty())
            .map(|s| s.to_lowercase())
    }

    async fn get_global_badges(&self, token: &UserToken) -> Result<Vec<BadgeSet>> {
        let mut lock = self.global_badges.lock().await;

        if let Some((badges, time)) = &*lock {
            if Local::now() - time <= TimeDelta::minutes(30) {
                return Ok(badges.clone());
            }
        }

        let badges = self
            .client
            .req_get(GetGlobalChatBadgesRequest::new(), token)
            .await?
            .data;

        *lock = Some((badges.clone(), Local::now()));
        Ok(badges)
    }

    async fn process_comment(
        &self,
        id: &str,
        payload: ChannelChatMessageV1Payload,
        app: &AppHandle,
    ) -> Result<()> {
        let token = self.authenticate(app).await?;
        let chatter = self
            .client
            .get_user_from_id(&payload.chatter_user_id, &token)
            .await?
            .unwrap();

        let service = app
            .lock_prefs()
            .await
            .get_service_by_id(id)
            .unwrap()
            .clone();

        let global_badges = self.get_global_badges(&token).await?;

        let channel_badges = self
            .client
            .req_get(GetChannelChatBadgesRequest::broadcaster_id(payload.broadcaster_user_id), &token)
            .await?
            .data;

        let badges = payload.badges
            .iter()
            .map(|badge| {
                channel_badges
                    .iter()
                    .find(|b| b.set_id == badge.set_id)
                    .or_else(|| {
                        global_badges.iter().find(|b| b.set_id == badge.set_id)
                    })
                    .map(|b| b.versions.first())
                    .flatten()
                    .map(|b| (b.title.clone(), b.image_url_4x.clone()))
                    .map(|(title, url)| {
                        UserBadge {
                            url,
                            label: title
                        }
                    })
                    .unwrap_or_else(|| {
                        UserBadge {
                            url: "".into(),
                            label: "Unknown".into()
                        }
                    })
            })
            .collect();

        let renderers = payload.message.fragments
            .iter()
            .map(|fragment| {
                match fragment {
                    Fragment::Text { text } => {
                        CommentRenderer::Text { content: text.into() }
                    }
                    Fragment::Emote { text, emote } => {
                        CommentRenderer::Emote { 
                            name: text.into(), 
                            url: EmoteUrlBuilder::new(&emote.id).animation_default().size_3x().render()
                        }
                    }
                    Fragment::Mention { text, mention } => {
                        CommentRenderer::Mention { content: text[1..].into() }
                    }
                    Fragment::Cheermote { text, cheermote } => {
                        CommentRenderer::TwitchCheermote { 
                            name: cheermote.prefix.clone(), 
                            url: format!("https://d3aqoihi2n8ty8.cloudfront.net/actions/{}/dark/animated/{}/4.gif", cheermote.prefix, cheermote.tier), 
                            bits: cheermote.bits as u32 
                        }
                    }
                    _ => {
                        CommentRenderer::Text { content: "?".into() }
                    }
                }
            })
            .collect();

        app.app_state()
            .event_tx
            .send(ServiceEvent::CommentCreated(Comment {
                id: service.id.clone(),
                service: CommentPlatform::Twitch,
                color: service.color.clone(),
                data: CommentData::Twitch {
                    base: CommonCommentData {
                        id: format!("tw-{}", payload.message_id.as_str()),
                        timestamp: Local::now().to_utc().timestamp() as u64,
                        name: payload.chatter_user_login.into(),
                        display_name: payload.chatter_user_name.into(),
                        profile_image: chatter.profile_image_url,
                        badges,
                        renderers,
                    },
                    bits: None,
                },
            }))?;

        Ok(())
    }

    async fn process_payload(&self, id: &str, event: Event, app: &AppHandle) -> Result<()> {
        match event {
            Event::ChannelChatMessageV1(Payload { message, .. }) => match message {
                Message::Notification(payload) => {
                    self.process_comment(id, payload, app).await?;
                }
                _ => {}
            },
            _ => {}
        }
        Ok(())
    }

    async fn process_ws_event(&self, id: &str, frame: &str, app: &AppHandle) -> Result<()> {
        match Event::parse_websocket(frame)? {
            EventsubWebsocketData::Welcome { metadata, payload } => {
                let mut session_id = self.websocket_session.write().await;
                session_id.replace(payload.session.id.into_owned());
                drop(session_id);
                self.subscribe_eventsub(id.into(), app).await?;
            }
            EventsubWebsocketData::Reconnect { metadata, payload } => {
                let mut session_id = self.websocket_session.write().await;
                session_id.take();
                drop(session_id);
            }
            EventsubWebsocketData::Notification {
                metadata,
                payload: event,
            } => {
                self.process_payload(id, event, app).await?;
            }
            _ => {}
        }

        Ok(())
    }

    async fn start_gateway(&self, id: String, app: &AppHandle) -> Result<()> {
        if self.websocket_session.read().await.is_some() {
            self.subscribe_eventsub(id, app).await?;
            return Ok(());
        }

        let app = app.clone();

        let mut shared: Arc<Self> = unsafe { Arc::from_raw(self as *const _) };

        let prefs = app.lock_prefs().await;
        let service = prefs.get_service_by_id(&id).unwrap().clone();
        let sid = service.id.clone();
        drop(prefs);

        let tx = app.app_state().event_tx.clone();

        tauri::async_runtime::spawn(async move {
            loop {
                println!("Starting WebSocket gateway");
                let ws = reqwest::Client::builder()
                    .http1_only()
                    .build()
                    .expect("Failed to create http client")
                    .get("wss://eventsub.wss.twitch.tv/ws")
                    .upgrade()
                    .send()
                    .await;

                let ws = match ws {
                    Ok(v) => v,
                    Err(_) => {
                        tokio::time::sleep(Duration::from_secs(5)).await;
                        continue;
                    }
                };

                let mut ws = ws.into_websocket().await;
                let mut ws = match ws {
                    Ok(v) => v,
                    Err(err) => {
                        println!("Error? {:?}", err);
                        return;
                    }
                };

                println!("Starting WebSocket gateway loop");
                while let Ok(Some(message)) = ws.try_next().await {
                    if let reqwest_websocket::Message::Text(text) = message {
                        if let Err(_) = shared.process_ws_event(&id.clone(), &text, &app).await {
                            // ...
                        }
                    }
                }

                let _ = tx.send(ServiceEvent::ServiceDisconnected { id: sid.clone() });

                let _ = shared.unsubscribe_eventsub(id.clone(), &app).await;

                println!("Not receiving WebSocket because ended");
                let _ = shared.websocket_session.write().await.take();
            }
        });

        Ok(())
    }

    async fn unsubscribe_eventsub(&self, id: String, app: &AppHandle) -> Result<()> {
        let token = self.authenticate(app).await?;

        let prefs = app.lock_prefs().await;
        let service = prefs.get_service_by_id(&id).unwrap();
        let id = service.id.clone();
        drop(prefs);

        let mut subs = self.subscriptions.write().await;
        if let Some(subs) = subs.get(&id) {
            for sub_id in subs {
                self.client
                    .delete_eventsub_subscription(sub_id, &token)
                    .await?;
            }
        }
        subs.remove(&id);

        Ok(())
    }

    async fn subscribe_eventsub(&self, id: String, app: &AppHandle) -> Result<()> {
        let token = match self.authenticate(app).await {
            Ok(token) => token,
            Err(err) => return Err(err.into()),
        };

        let prefs = app.lock_prefs().await;
        let service = prefs.get_service_by_id(&id).unwrap().clone();
        drop(prefs);

        let login = Self::extract_login(&service.url).unwrap();
        println!("Login name: {:?}", login);

        let user = self
            .client
            .get_user_from_login(&Nickname::new(login), &token)
            .await?
            .unwrap();
        println!("User ID: {:?}", user.id.clone());

        while let None = &*self.websocket_session.read().await {
            tokio::time::sleep(Duration::from_millis(100)).await;
        }

        let mut subs = self.subscriptions.write().await;

        let sub = self
            .client
            .create_eventsub_subscription(
                ChannelChatMessageV1::new(user.id.clone(), token.user_id.clone()),
                Transport::websocket(self.websocket_session.read().await.as_ref().unwrap()),
                &token,
            )
            .await?;
        app.app_state()
            .event_tx
            .send(ServiceEvent::ServiceConnected {
                id: service.id.clone(),
            })?;

        let sub_id = sub.id.clone();
        subs.entry(service.id.clone())
            .and_modify(|e| e.push(sub_id.clone()))
            .or_insert_with(|| vec![sub_id.clone()]);

        Ok(())
    }

    async fn user_token(&self) -> Result<Option<UserToken>> {
        let lock = self.authenticate_lock.lock().await;

        if let Some(token) = &*lock {
            return Ok(Some(token.clone()));
        }

        Ok(None)
    }

    async fn authenticate(&self, app: &AppHandle) -> Result<UserToken> {
        let lock = self.authenticate_lock.lock().await;

        if let Some(token) = &*lock {
            return Ok(token.clone());
        }

        let scopes = vec![
            Scope::ChannelReadRedemptions,
            Scope::ModeratorManageAnnouncements,
            Scope::UserReadChat,
        ];

        let entry = Entry::new("com.yuieii.mofuchat", "token")?;
        let password = entry.get_password();

        if let Ok(t) = password {
            let model = serde_json::from_str::<UserTokenModel>(&t);
            if let Ok(m) = model {
                if let Ok(res) = m.try_into_token(&self.client).await {
                    if res.scopes() == scopes {
                        return Ok(res);
                    }
                }
            }
        }

        let mut builder = DeviceUserTokenBuilder::new(CLIENT_ID, scopes);

        let info = builder.start(&self.client).await?;
        println!("Device code: {:?}", info.verification_uri);

        let result = builder
            .wait_for_code(&self.client, async |duration| {
                tokio::time::sleep(duration).await;
            })
            .await?;

        let save = UserTokenModel::new(&result);
        entry.set_password(&serde_json::to_string(&save)?)?;

        Ok(result)
    }
}

#[async_trait]
impl ServiceAdapter for TwitchServiceAdapter {
    fn can_handle(&self, service_url: &str) -> bool {
        Self::extract_login(&service_url).is_some()
    }

    async fn run(&self, channel_id: String, url: String, app: AppHandle) -> Result<()> {
        self.start_gateway(channel_id.clone(), &app).await?;
        Ok(())
    }

    async fn dispose(&self, service_id: String, app: AppHandle) -> Result<()> {
        self.unsubscribe_eventsub(service_id, &app).await?;
        Ok(())
    }
}
