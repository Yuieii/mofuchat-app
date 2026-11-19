use crate::state::ManagerExt;
use eyre::Result;
use tauri::{AppHandle, command};

use super::Prefs;

#[tauri::command]
pub async fn get_prefs(app: AppHandle) -> Prefs {
    app.lock_prefs().await.clone()
}

#[tauri::command]
pub async fn set_prefs(value: Prefs, app: AppHandle) {
    let mut prefs = app.lock_prefs().await;
    let _ = prefs.set(value, &app);
}
