use crate::state::ManagerExt;
use tauri::{AppHandle, Emitter, WebviewWindow};

use super::Prefs;

#[tauri::command]
pub async fn get_prefs(app: AppHandle) -> Prefs {
    app.lock_prefs().await.clone()
}

#[tauri::command]
pub async fn set_prefs(value: Prefs, window: WebviewWindow, app: AppHandle) {
    let mut prefs = app.lock_prefs().await;
    let _ = prefs.set(value, &app);

    let _ = app.emit("mofu://settings/invalidated", window.label());
}