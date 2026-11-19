use std::fmt::Display;

use eyre::Result;
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, command};

use super::ServiceEvent;
use crate::state::ManagerExt;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Error {
    InvalidServiceId,
    UnsupportedURL,
    Unknown,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::InvalidServiceId => f.write_str("Invalid service ID"),
            Error::UnsupportedURL => f.write_str("Unsupported URL"),
            Error::Unknown => f.write_str("Unknown"),
        }
    }
}

impl std::error::Error for Error {}

#[tauri::command]
pub async fn toggle_channel(app: AppHandle, service_id: String) -> Result<(), Error> {
    let mut tasks = app.lock_active_tasks().await;

    if let Some(old_handle) = tasks.remove(&service_id) {
        old_handle.abort();
    }

    let prefs = app.lock_prefs().await.clone();
    let Some(service) = prefs
        .services
        .iter()
        .find(|service| service.id == service_id)
    else {
        println!("Err: InvalidServiceId");
        return Err(Error::InvalidServiceId.into());
    };

    let service = service.clone();
    let state = app.app_state();

    let adapter = state
        .registry
        .resolve(&service.url)
        .ok_or_else(|| Error::UnsupportedURL)?;

    if !service.enabled {
        let _ = state.event_tx.send(ServiceEvent::ServiceDeactivated {
            id: service_id.clone(),
        });
        adapter
            .dispose(service_id.clone(), app.clone())
            .await
            .or_else(|_| Err(Error::Unknown))?;
        return Ok(());
    }

    let cloned = app.clone();
    let tx = state.event_tx.clone();
    let cid = service_id.clone();

    let handle = tokio::spawn(async move {
        println!("Run adapter");
        if let Err(err) = adapter.run(cid.clone(), service.url.clone(), cloned).await {
            let _ = tx.send(ServiceEvent::ServiceDisconnected { id: cid });
            println!("Error! {:?}", err);
        }
    });

    tasks.insert(service_id, handle);
    Ok(())
}
