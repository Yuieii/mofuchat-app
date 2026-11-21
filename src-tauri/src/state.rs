use std::collections::{HashMap, VecDeque};

use async_trait::async_trait;
use eyre::Result;
use tauri::{App, Manager, Runtime};
use tokio::{
    sync::{Mutex, MutexGuard, broadcast},
    task::JoinHandle,
};

use crate::{
    comments::Comment, prefs::Prefs, services::{AppEvent, ServiceEvent, adapter::AdapterRegistry},
};

const COMMENT_LIMIT: usize = 1000;

pub struct AppState {
    pub event_tx: broadcast::Sender<AppEvent>,
    pub service_tx: broadcast::Sender<ServiceEvent>,
    pub active_tasks: Mutex<HashMap<String, JoinHandle<()>>>,
    pub comments: Mutex<VecDeque<Comment>>,
    pub prefs: Mutex<Prefs>,
    pub registry: AdapterRegistry,
}

pub fn setup(app: &mut App) -> Result<()> {
    let (event_tx, _) = broadcast::channel(512);
    let (service_tx, _) = broadcast::channel(512);

    let handle = app.handle();
    let prefs = Prefs::try_from_local(handle)
        .or_else(|_| Prefs::try_from_onecomme(handle))
        .unwrap_or_else(|err| {
            println!("Error while reading OneComme config: {:?}", err);
            Prefs::default()
        });

    let state = AppState {
        service_tx,
        event_tx,
        prefs: Mutex::new(prefs),
        active_tasks: Mutex::new(HashMap::new()),
        registry: AdapterRegistry::new(),
        comments: Mutex::new(VecDeque::with_capacity(1000)),
    };

    app.manage(state);

    Ok(())
}

impl AppState {
    pub async fn lock_prefs(&self) -> MutexGuard<'_, Prefs> {
        self.prefs.lock().await
    }

    pub async fn lock_active_tasks(&self) -> MutexGuard<'_, HashMap<String, JoinHandle<()>>> {
        self.active_tasks.lock().await
    }

    pub fn event_tx(&self) -> &broadcast::Sender<AppEvent> {
        &self.event_tx
    }

    pub fn service_tx(&self) -> &broadcast::Sender<ServiceEvent> {
        &self.service_tx
    }
    
    pub async fn lock_comments(&self) -> MutexGuard<'_, VecDeque<Comment>> {
        self.comments.lock().await
    }

    pub async fn clear_comments(&self) -> Result<()> {
        let mut comments = self.lock_comments().await;
        comments.clear();

        self.event_tx().send(AppEvent::CommentCleared)?;
        // self.service_tx().send(ServiceEvent::CommentCleared)?;
        Ok(())
    }

    pub async fn push_comment(&self, comment: Comment) -> Result<()> {
        let mut comments = self.lock_comments().await;
        if (comments.len() == COMMENT_LIMIT) {
            comments.pop_front();
        }
        comments.push_back(comment.clone());

        self.service_tx().send(ServiceEvent::CommentCreated(comment))?;
        Ok(())
    }
}

#[async_trait]
pub trait ManagerExt<R> {
    fn app_state(&self) -> &AppState;

    async fn lock_active_tasks(&self) -> MutexGuard<'_, HashMap<String, JoinHandle<()>>> {
        self.app_state().lock_active_tasks().await
    }

    async fn lock_prefs(&self) -> MutexGuard<'_, Prefs> {
        self.app_state().lock_prefs().await
    }

    fn event_tx(&self) -> &broadcast::Sender<AppEvent> {
        self.app_state().event_tx()
    }

    fn service_tx(&self) -> &broadcast::Sender<ServiceEvent> {
        self.app_state().service_tx()
    }

    async fn lock_comments(&self) -> MutexGuard<'_, VecDeque<Comment>> {
        self.app_state().lock_comments().await
    }

    async fn clear_comments(&self) -> Result<()> {
        self.app_state().clear_comments().await
    }

    async fn push_comment(&self, comment: Comment) -> Result<()> {
        self.app_state().push_comment(comment).await
    }
}

impl<T, R> ManagerExt<R> for T
where
    T: Manager<R>,
    R: Runtime,
{
    fn app_state(&self) -> &AppState {
        self.state::<AppState>().inner()
    }
}
