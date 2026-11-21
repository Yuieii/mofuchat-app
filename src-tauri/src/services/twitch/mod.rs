use std::fmt::Display;
use std::mem::ManuallyDrop;
use std::sync::Weak;
use std::{collections::HashMap, sync::Arc, time::Duration};

use async_trait::async_trait;
use chrono::{DateTime, Local, TimeDelta};
use futures_util::{SinkExt, TryStreamExt};
use keyring::Entry;
use serde::{Deserialize, Serialize};
use tauri::AppHandle;
use tauri::async_runtime::JoinHandle;
use tokio::sync::{Mutex, RwLock, RwLockReadGuard, RwLockWriteGuard};

use twitch_api::HelixClient;
use twitch_api::common::chat::Fragment;
use twitch_api::eventsub::channel::*;
use twitch_api::eventsub::*;
use twitch_api::helix::Scope;
use twitch_api::helix::chat::{BadgeSet, GetChannelChatBadgesRequest, GetGlobalChatBadgesRequest};
use twitch_api::twitch_oauth2::id::DeviceCodeResponse;
use twitch_api::types::{EmoteUrlBuilder, EventSubId};

use twitch_api::{
    twitch_oauth2::{
        AccessToken, ClientId, DeviceUserTokenBuilder, RefreshToken, TwitchToken, UserToken,
        client::Client,
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

#[derive(Clone, Debug)]
struct ErrString(String);

impl std::error::Error for ErrString {}

impl Display for ErrString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

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
    websocket_active: Mutex<()>,
    websocket_session: Arc<RwLock<Option<String>>>,
    subscriptions: Arc<RwLock<HashMap<String, Vec<EventSubId>>>>,
    global_badges: Mutex<Option<(Vec<BadgeSet>, DateTime<Local>)>>,
    pending_auth: Mutex<Option<DeviceCodeResponse>>,
    loop_handle: Mutex<Option<JoinHandle<()>>>,
    
    /// Broadcaster User ID -> Service ID lookup
    id_service_lookup: Arc<RwLock<HashMap<String, String>>>,

    /// Service ID -> Broadcaster User ID lookup
    service_id_lookup: Arc<RwLock<HashMap<String, String>>>,

    weak_self: Weak<Self>
}

impl TwitchServiceAdapter {
    pub fn new_shared() -> Arc<Self> {
        Arc::new_cyclic(|weak| Self {
            client: HelixClient::default(),
            authenticate_lock: Mutex::new(None),
            websocket_active: Mutex::new(()),
            websocket_session: Arc::new(RwLock::new(None)),
            subscriptions: Arc::new(RwLock::new(HashMap::new())),
            global_badges: Mutex::new(None),
            pending_auth: Mutex::new(None),
            loop_handle: Mutex::new(None),

            id_service_lookup: Arc::new(RwLock::new(HashMap::new())),
            service_id_lookup: Arc::new(RwLock::new(HashMap::new())),

            weak_self: weak.clone()
        })
    }

    async fn lock_read_websocket_session(&self) -> RwLockReadGuard<'_, Option<String>> {
        self.websocket_session.read().await
    }

    async fn lock_write_websocket_session(&self) -> RwLockWriteGuard<'_, Option<String>> {
        self.websocket_session.write().await
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
        payload: ChannelChatMessageV1Payload,
        app: &AppHandle,
    ) -> Result<()> {
        log::trace!("--- Start processing comment ---- first");
        let service_lookup = self.id_service_lookup.read().await;
        let id = service_lookup.get(payload.broadcaster_user_id.as_str())
            .expect("Service lookup failed").clone();
        drop(service_lookup);

        log::trace!("--- Start processing comment");
        let token = self.authenticate(app).await?;

        log::trace!("--- Getting chatter info: needs avatar");

        let chatter = self
            .client
            .get_user_from_id(&payload.chatter_user_id, &token)
            .await?
            .unwrap();

        let service = app
            .lock_prefs()
            .await
            .get_service_by_id(&id)
            .unwrap()
            .clone();

        log::trace!("--- Getting badges: needs icon URL");

        let global_badges = self.get_global_badges(&token).await?;

        let channel_badges = self
            .client
            .req_get(
                GetChannelChatBadgesRequest::broadcaster_id(payload.broadcaster_user_id),
                &token,
            )
            .await?
            .data;

        let badges = payload
            .badges
            .iter()
            .map(|badge| {
                channel_badges
                    .iter()
                    .find(|b| b.set_id == badge.set_id)
                    .or_else(|| global_badges.iter().find(|b| b.set_id == badge.set_id))
                    .map(|b| b.versions.iter().find(|v| v.id == badge.id))
                    .flatten()
                    .map(|b| (b.title.clone(), b.image_url_4x.clone()))
                    .map(|(title, url)| UserBadge { url, label: title })
                    .unwrap_or_else(|| UserBadge {
                        url: "".into(),
                        label: "Unknown".into(),
                    })
            })
            .collect();

        log::trace!("--- Transforming fragments into renderer");

        let reply = payload.reply;

        let renderers = payload
            .message
            .fragments
            .iter()
            .enumerate()
            .filter_map(|(i, fragment)| {
                if i == 0 && match fragment {
                    Fragment::Mention { .. } => true,
                    _ => false
                } {
                    None
                } else {
                    Some(fragment)
                }
            })
            .map(|fragment| match fragment {
                Fragment::Text { text } => CommentRenderer::Text {
                    content: text.into(),
                },
                Fragment::Emote { text, emote } => CommentRenderer::Emote {
                    name: text.into(),
                    url: EmoteUrlBuilder::new(&emote.id)
                        .animation_default()
                        .size_3x()
                        .render(),
                },
                Fragment::Mention { text, .. } => CommentRenderer::Mention {
                    content: text[1..].into(),
                },
                Fragment::Cheermote { cheermote, .. } => CommentRenderer::TwitchCheermote {
                    name: cheermote.prefix.clone(),
                    url: format!(
                        "https://d3aqoihi2n8ty8.cloudfront.net/actions/{}/dark/animated/{}/4.gif",
                        cheermote.prefix, cheermote.tier
                    ),
                    bits: cheermote.bits as u32,
                },
                _ => CommentRenderer::Text {
                    content: "?".into(),
                },
            })
            .collect();

        log::trace!("--- Pushing comment event...");

        app.push_comment(Comment {
            id: service.id.clone(),
            service: CommentPlatform::Twitch,
            color: service.color.clone(),
            data: CommentData::Twitch {
                base: CommonCommentData {
                    id: format!("tw-{}", payload.message_id.as_str()),
                    user_id: payload.chatter_user_id.to_string(),
                    timestamp: Local::now().to_utc().to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
                    name: payload.chatter_user_login.into(),
                    display_name: payload.chatter_user_name.into(),
                    profile_image: chatter.profile_image_url,
                    badges,
                    renderers,
                },
                bits: None,
            },
        }).await?;

        Ok(())
    }

    async fn process_payload(&self, event: Event, app: &AppHandle) -> Result<()> {
        match event {
            Event::ChannelChatMessageV1(Payload { message, .. }) => match message {
                Message::Notification(payload) => {
                    self.process_comment(payload, app).await?;
                }
                _ => {}
            },
            _ => {}
        }
        Ok(())
    }

    async fn process_ws_event(&self, frame: &str, app: &AppHandle) -> Result<()> {
        match Event::parse_websocket(frame)? {
            EventsubWebsocketData::Welcome { payload, .. } => {
                let mut session_id = self.lock_write_websocket_session().await;
                session_id.replace(payload.session.id.into_owned());
                drop(session_id);
            }
            EventsubWebsocketData::Reconnect { .. } => {
                let mut session_id = self.lock_write_websocket_session().await;
                // TODO: Handle reconnect
                session_id.take();
                drop(session_id);
            }
            EventsubWebsocketData::Notification { payload, .. } => {
                self.process_payload(payload, app).await?;
            }
            _ => {}
        }

        Ok(())
    }

    async fn gateway_loop(&self, app: AppHandle) {
        log::trace!("Checking gateway loop...");
        let Ok(_lock) = self.websocket_active.try_lock() else {
            log::trace!("Already has active WebSocket loop. Exiting for this one");
            return
        };
    
        log::trace!("Running gateway loop...");

        loop {
            let token = self.user_token().await;

            let Ok(token) = token else {
                tokio::time::sleep(Duration::from_secs(5)).await;
                continue;
            };

            let _token = match token {
                Some(token) => token,
                None => match self.authenticate(&app).await {
                    Ok(token) => token,
                    Err(err) => {
                        log::trace!("Failed to authenticate: {err:?}");
                        tokio::time::sleep(Duration::from_secs(5)).await;
                        continue;
                    }
                }
            };

            log::trace!("Starting WebSocket gateway");
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
                    log::warn!("Failed to upgrade to WebSocket. Retrying");
                    tokio::time::sleep(Duration::from_secs(5)).await;
                    continue;
                }
            };

            let ws = ws.into_websocket().await;
            let mut ws = match ws {
                Ok(v) => v,
                Err(err) => {
                    log::error!("Error? {:?}", err);
                    return;
                }
            };

            log::trace!("Starting WebSocket gateway loop");
            while let Ok(Some(message)) = ws.try_next().await {
                match message {
                    reqwest_websocket::Message::Text(text) => {
                        if let Err(err) = self.process_ws_event(&text, &app).await {
                            log::error!("Error while processing WebSocket message: {err:?}");
                        }
                    }

                    reqwest_websocket::Message::Ping(bytes) => {
                        if let Err(err) = ws.send(reqwest_websocket::Message::Pong(bytes)).await {
                            log::error!("Error while processing WebSocket ping: {err:?}");
                        }
                    }

                    reqwest_websocket::Message::Close { code, reason } => {
                        log::warn!("Connection closed!!! code={code:?} reason={reason:?}");
                    }

                    _ => {}
                }
            }

            let _ = app.service_tx().send(ServiceEvent::PlatformDisconnected {
                platform: CommentPlatform::Twitch
            });

            // let _ = shared.unsubscribe_eventsub(id.clone(), &app).await;
            let mut subs = self.subscriptions.write().await;
            subs.clear();

            log::warn!("Not receiving WebSocket because ended");
            let _ = self.lock_write_websocket_session().await.take();

            tokio::time::sleep(Duration::from_secs(5)).await;
        }
    }

    async fn start_gateway(&self, id: String, app: &AppHandle) -> Result<()> {
        let session = self.lock_read_websocket_session().await;
        if session.is_some() {
            self.subscribe_eventsub(id, app).await?;
            return Ok(());
        }
        drop(session);

        let app = app.clone();
        let shared = self.weak_self.upgrade()
            .expect("unwrapping weak pointer of self");

        let prefs = app.lock_prefs().await;
        let service = prefs.get_service_by_id(&id).unwrap().clone();
        let sid = service.id.clone();
        drop(prefs);

        let id_clone = id.clone();
        let app_clone = app.clone();
        let app = app.clone();
        let mut loop_handle = self.loop_handle.lock().await;
        let handle = tauri::async_runtime::spawn(async move {
            shared.gateway_loop(app).await;
        });
        *loop_handle = Some(handle);

        self.subscribe_eventsub(id_clone, &app_clone).await?;
        Ok(())
    }

    async fn unsubscribe_eventsub(&self, id: String, app: &AppHandle) -> Result<()> {
        let token = self.authenticate(app).await?;

        let mut subs = self.subscriptions.write().await;
        if let Some(subs) = subs.get(&id) {
            for sub_id in subs {
                self.client
                    .delete_eventsub_subscription(sub_id, &token)
                    .await?;
            }
            
            app.service_tx().send(ServiceEvent::ServiceDeactivated {
                id: id.clone(),
            })?;

            let mut id_lookup = self.service_id_lookup.write().await;
            let mut service_lookup = self.id_service_lookup.write().await;
            let user_id = id_lookup.get(&id).expect(&format!("Failed to lookup user ID by service ID: {id:?}")).clone();
            id_lookup.remove(&id);
            service_lookup.remove(&user_id);
        }

        subs.remove(&id);

        Ok(())
    }

    /// Creates a subscription to EventSub.
    /// 
    /// Called when a service is enabled.
    async fn subscribe_eventsub(&self, id: String, app: &AppHandle) -> Result<()> {
        let token = match self.authenticate(app).await {
            Ok(token) => token,
            Err(err) => return Err(err.into()),
        };

        let prefs = app.lock_prefs().await;
        let service = prefs.get_service_by_id(&id).unwrap().clone();
        drop(prefs);

        let login = Self::extract_login(&service.url).unwrap();
        log::trace!("Login name: {:?}", login);

        let user = self
            .client
            .get_user_from_login(&Nickname::new(login), &token)
            .await?
            .unwrap();

        log::trace!("User ID: {:?}", user.id.clone());

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

        app.service_tx()
            .send(ServiceEvent::ServiceConnected {
                id: service.id.clone(),
            })?;

        let sub_id = sub.id.clone();
        subs.entry(service.id.clone())
            .and_modify(|e| e.push(sub_id.clone()))
            .or_insert_with(|| vec![sub_id.clone()]);

        let mut service_lookup = self.id_service_lookup.write().await;
        let mut id_lookup = self.service_id_lookup.write().await;
        service_lookup.insert(user.id.as_str().into(), service.id.clone());
        id_lookup.insert(service.id.clone(), user.id.as_str().into());

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
        let pending = self.pending_auth.lock().await;
        if let Some(info) = &*pending {
            app.service_tx()
                .send(ServiceEvent::RequiresSignIn {
                    platform: CommentPlatform::Twitch,
                    code: Some(info.user_code.clone()),
                    url: info.verification_uri.clone(),
                    expires_in: info.expires_in,
                })?;
            return Err(ErrString("Pending device code auth".into()))?;
        }
        drop(pending);

        let mut lock = self.authenticate_lock.lock().await;

        if let Some(token) = &mut *lock {
            if token.is_elapsed() {
                token.refresh_token(&self.client).await?;
                
                let token = token.clone();
                let save = UserTokenModel::new(&token);
                let entry = Entry::new("com.yuieii.mofuchat", "token")?;
                entry.set_password(&serde_json::to_string(&save)?)?;
            }
            return Ok(token.clone());
        }

        log::trace!("No stored authentication!");

        let scopes = vec![
            Scope::ChannelReadRedemptions,
            Scope::ModeratorManageAnnouncements,
            Scope::UserReadChat,
        ];

        let entry = Entry::new("com.yuieii.mofuchat", "token")?;
        let password = entry.get_password();

        match password {
            Ok(t) => match serde_json::from_str::<UserTokenModel>(&t) {
                Ok(m) => match m.try_into_token(&self.client).await {
                    Ok(res) => {
                        if res.scopes() == scopes {
                            *lock = Some(res.clone());
                            return Ok(res);
                        } else {
                            log::error!("Scopes mismatch!");
                        }
                    }

                    Err(error) => {
                        log::error!("Failed to authenticate: {:?}", error);
                    }
                },

                Err(error) => {
                    log::error!(
                        "Failed to deserialize info from secure storage: {:?}",
                        error
                    );
                }
            },

            Err(error) => {
                log::error!("Failed to get credential from secure storage: {:?}", error);
            }
        }

        let mut builder = DeviceUserTokenBuilder::new(CLIENT_ID, scopes);

        let info = builder.start(&self.client).await?;
        log::trace!("Device code: {:?}", info.verification_uri);

        let mut pending = self.pending_auth.lock().await;
        *pending = Some(info.clone());
        drop(pending);

        app.service_tx()
            .send(ServiceEvent::RequiresSignIn {
                platform: CommentPlatform::Twitch,
                code: Some(info.user_code.clone()),
                url: info.verification_uri.clone(),
                expires_in: info.expires_in,
            })?;

        let result = builder
            .wait_for_code(&self.client, async |duration| {
                tokio::time::sleep(duration).await;
            })
            .await?;

        let mut pending = self.pending_auth.lock().await;
        *pending = None;
        drop(pending);

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

    async fn terminate(&self) {
        if let Some(handle) = &*self.loop_handle.lock().await {
            handle.abort();
        }
    }
}
