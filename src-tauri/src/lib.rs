mod comments;
mod prefs;
mod services;
mod state;

use tauri::{App, AppHandle, Emitter, Manager, WebviewWindow, WindowEvent};
use tauri::menu::{MenuBuilder, MenuItemBuilder};
use tauri::tray::{TrayIconBuilder, TrayIconEvent};

use crate::prefs::VisualStyle;
use crate::{
    services::commands::toggle_channel,
    state::ManagerExt,
};

#[tauri::command]
async fn open_settings_window(app: AppHandle) {
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
        let _ = window.show();
        let _ = window.set_focus();
        return;
    }

    let prefs = app.lock_prefs().await;
    let dark_mode = prefs.config.app.dark_mode;
    let visual_style = prefs.config.mofu.window_style.clone();
    
    let window = tauri::WebviewWindowBuilder::new(&app, "settings", tauri::WebviewUrl::App("/settings.html".into()))
        .fullscreen(false)
        .resizable(true)
        .transparent(true)
        .inner_size(800.0, 600.0)
        .min_inner_size(800.0, 600.0)
        .title("MofuChat")
        .visible(false)
        .build()
        .unwrap();

    apply_window_effects(&window, visual_style, dark_mode).unwrap();
    window.show().unwrap();
}

fn apply_window_effects(window: &WebviewWindow, visual_style: VisualStyle, dark_mode: bool) -> eyre::Result<()> {
    #[cfg(target_os = "windows")]
    window.set_decorations(false)?;

    let window_ = window.clone();

    window.with_webview(move |webview| {
        //window_.set_traffic_lights_inset(16.0, 20.0).unwrap();

        // Make window transparent without privateApi
        // window.make_transparent().unwrap();

        use window_vibrancy::*;

        match visual_style {
            #[cfg(target_os = "windows")]    
            VisualStyle::Mica => {
                let _ = apply_mica(&window_, Some(dark_mode));
            }

            #[cfg(target_os = "windows")]
            VisualStyle::Acrylic => {
                let _ = apply_acrylic(&window_, None);    
            }    

            #[cfg(target_os = "macos")]
            VisualStyle::Vibrancy => {
                apply_vibrancy(&window_, NSVisualEffectMaterial::HudWindow, None, None)
                    .expect("Unsupported platform! 'apply_vibrancy' is only supported on macOS");    
            }

            _ => {}
        };

        // #[cfg(target_os = "macos")]
        {
            // use objc2_web_kit::WKWebView;
            // let webview: &WKWebView = unsafe {&*webview.inner().cast()};
            // let options = LiquidGlassOptions::new(NSGlassEffectViewStyle::Sidebar)
            //     .radius(26.0)
            //     .interactive(true)
            //     .content_view(webview);

            // apply_liquid_glass(&window_, options)
            //     .expect("Unsupported platform! 'apply_liquid_glass' is only supported on macOS 26+");
        }
    })?;

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
            TrayIconEvent::DoubleClick { id, position, rect, button } => {
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
            .build()?
    ))?;

    tray.on_menu_event(|app, event| {
        match event.id.as_ref() {
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
        }
    });

    state::setup(app)?;

    // Create a custom titlebar for main window
    // On Windows this will hide decoration and render custom window controls
    // On macOS it expects a hiddenTitle: true and titleBarStyle: overlay

    let handle = app.handle().clone();
    let (visual_style, dark_mode) = tauri::async_runtime::block_on(async move {
        let prefs = handle.lock_prefs().await;
        (prefs.config.mofu.window_style.clone(), prefs.config.app.dark_mode)
    });

    let window = app.get_webview_window("main").unwrap();
    apply_window_effects(&window, visual_style, dark_mode)?;

    let handle = app.handle().clone();
    let mut rx = handle.app_state().event_tx.subscribe();

    let h = handle.clone();
    tauri::async_runtime::spawn(async move {
        while let Ok(event) = rx.recv().await {
            println!("emit event pre: {:?}", &event);
            let _ = h.emit("mofu://services/event", &event);
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
