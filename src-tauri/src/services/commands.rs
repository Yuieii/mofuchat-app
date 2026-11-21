use std::{collections::VecDeque, fmt::Display};

use eyre::Result;
use serde::{Deserialize, Serialize};
use tauri::AppHandle;

use thiserror::Error;

use super::ServiceEvent;
use crate::{comments::Comment, error::{FutureExt, TauriResult}, state::ManagerExt};

#[derive(Error, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Error {
    #[error("Invalid service ID")]
    InvalidServiceId,
    #[error("Unsupported URL")]
    UnsupportedURL,
    #[error("Unknown error")]
    Unknown,
}

#[tauri::command]
pub async fn fetch_comments_from_memory(app: AppHandle) -> VecDeque<Comment> {
    let comments = app.lock_comments().await;
    log::trace!("Returning {} comments", comments.len());
    comments.clone()
}

#[tauri::command]
pub async fn clear_comments(app: AppHandle) -> TauriResult<()> {
    app.clear_comments().tauri().await?;
    Ok(())
}

#[tauri::command]
pub fn check_supports_url(app: AppHandle, url: String) -> bool {
    app.app_state()
        .registry
        .resolve(&url)
        .map(|_| true)
        .unwrap_or(false)
}

#[tauri::command]
pub async fn toggle_channel(app: AppHandle, service_id: String) -> TauriResult<()> {
    let mut tasks = app.lock_active_tasks().await;
    if let Some(old_handle) = tasks.remove(&service_id) {
        old_handle.abort();
    }
    drop(tasks);

    let prefs = app.lock_prefs().await.clone();
    let Some(service) = prefs
        .services
        .iter()
        .find(|service| service.id == service_id)
    else {
        return Err(Error::InvalidServiceId.into());
    };

    let service = service.clone();
    let state = app.app_state();

    let adapter = state
        .registry
        .resolve(&service.url)
        .ok_or_else(|| Error::UnsupportedURL)?;

    if !service.enabled {
        adapter
            .dispose(service_id.clone(), app.clone())
            .await
            .or_else(|_| Err(Error::Unknown))?;
        
        return Ok(());
    }

    let cloned = app.clone();
    let tx = app.service_tx().clone();
    let cid = service_id.clone();

    let handle = tokio::spawn(async move {
        log::trace!("Running adapter for service: {service:?}");
        if let Err(err) = adapter.run(cid.clone(), service.url.clone(), cloned).await {
            let _ = tx.send(ServiceEvent::ServiceDisconnected { id: cid });
            log::error!("Error! {:?}", err);
        }
    });

    let mut tasks = app.lock_active_tasks().await;
    tasks.insert(service_id, handle);
    Ok(())
}
