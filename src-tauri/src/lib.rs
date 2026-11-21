mod comments;
mod prefs;
mod services;
mod state;
mod error;

#[cfg(target_os = "macos")]
use tauri::RunEvent;
use tauri::menu::{MenuBuilder, MenuItemBuilder};
use tauri::tray::{TrayIconBuilder, TrayIconEvent};
use tauri::{App, AppHandle, Emitter, Manager, WebviewWindow, WindowEvent};
use crate::error::{CommandError, ResultExt, TauriResult};
use crate::{services::commands::toggle_channel, state::ManagerExt};

#[tauri::command]
async fn open_settings_window(app: AppHandle) -> TauriResult<()> {
    /*
       fullscreen: false,
       resizable: true,
       width: 800,
       minWidth: 800,
       height: 600,
       minHeight: 600,
       titleBarStyle: "overlay",
       hiddenTitle: true,
       center: true,
       focusable: true,
       focus: true,
       decorations: currentPlatform === "macos",
       transparent: true,
    */

    if let Some(window) = app.get_webview_window("settings") {
        window.show()?;
        window.set_focus()?;
        return Ok(());
    }

    let prefs = app.lock_prefs().await;
    let dark_mode = prefs.config.app.dark_mode;
    drop(prefs);

    let window = tauri::WebviewWindowBuilder::new(
        &app,
        "settings",
        tauri::WebviewUrl::App("/settings.html".into()),
    )
    .fullscreen(false)
    .resizable(true)
    .transparent(true)
    .inner_size(800.0, 600.0)
    .min_inner_size(800.0, 600.0)
    .title("MofuChat")
    .visible(false);

    #[cfg(target_os = "macos")]
    let window = {
        window
            .title_bar_style(tauri::TitleBarStyle::Overlay)
            .hidden_title(true)
    };

    let window = window.build()?;
    apply_window_effects(&app, &window, dark_mode).tauri()?;
    window.show()?;

    Ok(())
}

fn apply_window_effects(
    app: &AppHandle,
    window: &WebviewWindow,
    dark_mode: bool,
) -> eyre::Result<()> {
    #[cfg(target_os = "windows")]
    window.set_decorations(false)?;

    let window_ = window.clone();
    let handle = app.clone();
    
    tauri::async_runtime::spawn(async move {
        let prefs = handle.lock_prefs().await;
        let _ = prefs
            .config
            .mofu
            .apply_window_style(&handle, &window_, dark_mode);
    });

    Ok(())
}

fn setup(app: &mut App) -> Result<(), Box<dyn std::error::Error + 'static>> {
    let tray = TrayIconBuilder::new()
        .icon(app.default_window_icon().unwrap().clone())
        .build(app)?;

    tray.on_tray_icon_event(|icon, event| {
        let main_window = icon.app_handle().get_webview_window("main").unwrap();
        match event {
            #[cfg(target_os = "windows")]
            TrayIconEvent::DoubleClick {
                button, ..
            } => {
                use tauri::tray::MouseButton;
                if button == MouseButton::Left {
                    let _ = main_window.show();
                    let _ = main_window.set_focus();
                }
            }

            _ => {}
        }
    });

    tray.set_menu(Some(
        MenuBuilder::new(app)
            .item(&MenuItemBuilder::new("開啟").id("tray_open").build(app)?)
            .item(&MenuItemBuilder::new("退出").id("tray_quit").build(app)?)
            .build()?,
    ))?;

    tray.on_menu_event(|app, event| match event.id.as_ref() {
        "tray_open" => {
            let main_window = app.get_webview_window("main").unwrap();
            let _ = main_window.show();
            let _ = main_window.set_focus();
        }

        "tray_quit" => {
            let handle = app.clone();
            tauri::async_runtime::block_on(async move {
                let adapters = &handle.app_state().registry;
                adapters.terminate(&handle).await;
                handle.cleanup_before_exit();
                handle.exit(0);
            });
        }

        _ => {}
    });

    state::setup(app)?;

    // Create a custom titlebar for main window
    // On Windows this will hide decoration and render custom window controls
    // On macOS it expects a hiddenTitle: true and titleBarStyle: overlay

    let handle = app.handle().clone();
    let dark_mode = tauri::async_runtime::block_on(async move {
        let prefs = handle.lock_prefs().await;
        prefs.config.app.dark_mode
    });

    let window = app.get_webview_window("main").unwrap();
    apply_window_effects(&app.handle().clone(), &window, dark_mode)?;

    let handle = app.handle().clone();

    let mut rx = handle.service_tx().subscribe();
    let h = handle.clone();
    tauri::async_runtime::spawn(async move {
        while let Ok(event) = rx.recv().await {
            log::trace!("emit service event: {:?}", &event);
            let _ = h.emit("mofu://services/event", &event);
        }
        log::warn!("emit event out, this is bad");
    });
    
    let mut rx = handle.event_tx().subscribe();
    let h = handle.clone();
    tauri::async_runtime::spawn(async move {
        while let Ok(event) = rx.recv().await {
            log::trace!("emit app event: {:?}", &event);
            let _ = h.emit("mofu://app/event", &event);
        }
        log::warn!("emit event out, this is bad");
    });

    let h = handle.clone();
    tauri::async_runtime::spawn(async move {
        let prefs = h.lock_prefs().await;
        let ids: Vec<_> = prefs.services.iter().map(|s| s.id.clone()).collect();
        drop(prefs);

        // A hack to turn adapters on when they are enabled
        for id in ids {
            let _ = toggle_channel(h.clone(), id).await;
        }
    });

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let app = tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::new()
                // .level(tauri_plugin_log::log::LevelFilter::Info)
                .build(),
        )
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_decorum::init())
        .setup(setup)
        .on_window_event(|window, event| match event {
            WindowEvent::CloseRequested { api, .. } => {
                if window.label() == "main" {
                    window.hide().unwrap();
                    api.prevent_close();
                }
            }
            _ => {}
        })
        .invoke_handler(tauri::generate_handler![
            open_settings_window,
            prefs::commands::get_prefs,
            prefs::commands::set_prefs,
            services::commands::fetch_comments_from_memory,
            services::commands::clear_comments,
            services::commands::check_supports_url,
            services::commands::toggle_channel,
        ])
        .build(tauri::generate_context!())
        .expect("error while building tauri application");

    app.run(|app_handle, event| {
        #[cfg(target_os = "macos")]
        if let RunEvent::Reopen {
            has_visible_windows,
            ..
        } = event
        {
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
