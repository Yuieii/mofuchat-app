use std::sync::Arc;

use async_trait::async_trait;
use eyre::Result;
use futures_util::future::join_all;
use tauri::AppHandle;
use tokio::sync::broadcast;

use crate::{
    prefs::Service,
    services::{ServiceEvent, adapter, twitch},
    state::ManagerExt,
};

#[async_trait]
pub trait ServiceAdapter: Send + Sync {
    fn can_handle(&self, service_url: &str) -> bool;
    async fn run(&self, service_id: String, url: String, app: AppHandle) -> Result<()>;
    async fn dispose(&self, service_id: String, app: AppHandle) -> Result<()>;
    async fn terminate(&self);
}

pub struct AdapterRegistry {
    adapters: Vec<Arc<dyn ServiceAdapter>>,
}

impl AdapterRegistry {
    pub fn new() -> Self {
        Self {
            adapters: vec![
                twitch::TwitchServiceAdapter::new(),
                // Arc::new(yo)
            ],
        }
    }

    pub fn resolve(&self, url: &str) -> Option<Arc<dyn ServiceAdapter>> {
        self.adapters
            .iter()
            .find(|adapter| adapter.can_handle(url))
            .cloned()
    }

    pub async fn terminate(&self, app: &AppHandle) {
        log::trace!("Obtaining active tasks...");
        let tasks = app.lock_active_tasks().await;
        log::trace!("Terminating active tasks...");
        for task in tasks.values() {
            task.abort();
        }
        drop(tasks);

        log::trace!("Terminating active tasks complete");

        let futures: Vec<_> = self
            .adapters
            .iter()
            .map(|adapter| adapter.terminate())
            .collect();

        log::trace!("Terminating adapters");
        join_all(futures).await;
        log::trace!("All adapter terminated");
    }
}
