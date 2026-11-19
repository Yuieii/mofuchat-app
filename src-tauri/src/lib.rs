mod state;
mod prefs;
mod comments;
mod services;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

use std::sync::Mutex;

use tauri::{App, Emitter, Manager, RunEvent, WindowEvent};

use crate::{prefs::Prefs, services::commands::toggle_channel, state::{AppState, ManagerExt}};

fn setup(app: &mut App) -> Result<(), Box<dyn std::error::Error + 'static>> {
    // Create a custom titlebar for main window
    // On Windows this will hide decoration and render custom window controls
    // On macOS it expects a hiddenTitle: true and titleBarStyle: overlay
    let main_window = app.get_webview_window("main").unwrap();
    // main_window.create_overlay_titlebar().unwrap();

    #[cfg(target_os = "macos")]
    let window_ = main_window.clone();

    state::setup(app)?;

    #[cfg(target_os = "macos")]
    main_window.with_webview(move |webview| {
        //window_.set_traffic_lights_inset(16.0, 20.0).unwrap();

        // Make window transparent without privateApi
        // main_window.make_transparent().unwrap();

        // use objc2_web_kit::WKWebView;
        use window_vibrancy::*;

        // let webview: &WKWebView = unsafe {&*webview.inner().cast()};

        apply_vibrancy(&window_, NSVisualEffectMaterial::HudWindow,  None, None)
            .expect("Unsupported platform! 'apply_vibrancy' is only supported on macOS");

        // let options = LiquidGlassOptions::new(NSGlassEffectViewStyle::Sidebar)
        //     .radius(26.0)
        //     .interactive(true)
        //     .content_view(webview);

        // apply_liquid_glass(&window_, options)
        //     .expect("Unsupported platform! 'apply_liquid_glass' is only supported on macOS 26+");
    })?;

    let handle = app.handle().clone();
    let mut rx = handle.app_state().event_tx.subscribe();

    let h = handle.clone();
    tauri::async_runtime::spawn(async move {
        while let Ok(event) = rx.recv().await {
            println!("emit event pre: {:?}", &event);
            let _ = h.emit("mofuchat-event", &event);
            println!("emit event post: {:?}", &event);
        }
        println!("emit event out, this is bad");
    });

    let h = handle.clone();
    tauri::async_runtime::spawn(async move { 
        let prefs = h.lock_prefs().await;
        let ids: Vec<_> = prefs.services.iter().map(|s| s.id.clone()).collect();
        drop(prefs);

        for id in ids {
            let _ = toggle_channel(h.clone(), id).await;
        }
    });

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let app = tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_decorum::init())
        .setup(setup)
        .on_window_event(|window, event| match event {
            WindowEvent::CloseRequested { api, .. } => {
                window.hide().unwrap();
                api.prevent_close();
            },
            _ => {}
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            prefs::commands::get_prefs,
            prefs::commands::set_prefs,
            services::commands::toggle_channel,
        ])
        .build(tauri::generate_context!())
        .expect("error while building tauri application");

    app.run(|app_handle, event| {
        if let RunEvent::Reopen { has_visible_windows, .. } = event {
            if !has_visible_windows {
                if let Some(window) = app_handle.get_webview_window("main") {
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }
        }
    });
    // .expect("error while running tauri application");
}
