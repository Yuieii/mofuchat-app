use std::sync::Arc;

use async_trait::async_trait;
use eyre::Result;
use tauri::AppHandle;
use tokio::sync::broadcast;

use crate::{
    prefs::Service,
    services::{ServiceEvent, twitch},
};

#[async_trait]
pub trait ServiceAdapter: Send + Sync {
    fn can_handle(&self, service_url: &str) -> bool;
    async fn run(&self, service_id: String, url: String, app: AppHandle) -> Result<()>;
    async fn dispose(&self, service_id: String, app: AppHandle) -> Result<()>;
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
}
