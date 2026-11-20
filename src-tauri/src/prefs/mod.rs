use std::{default, fs};

use eyre::Result;
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager, path::BaseDirectory};
use window_vibrancy::clear_mica;

pub mod commands;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(default, rename_all = "camelCase")]
pub struct SpeechConfig {
    volume: f32,
    rate: f32,
    pitch: f32,
}

impl SpeechConfig {
    pub fn replace(&mut self, other: Self, app: &AppHandle) {
        self.volume = other.volume;
        self.rate = other.rate;
        self.pitch = other.pitch;
    }
}

impl Default for SpeechConfig {
    fn default() -> Self {
        SpeechConfig { volume: 0.5, rate: 1.0, pitch: 1.0 }
    }
}

#[cfg(target_os = "windows")]
#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum WindowsVisualStyle {
    Normal,
    #[default]
    Mica,
    Acrylic,
}

#[cfg(target_os = "macos")]
#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum MacOsVisualStyle {
    Normal,
    #[default]
    Vibrancy,
    Glass,
}

#[cfg(target_os = "windows")]
pub type VisualStyle = WindowsVisualStyle;

#[cfg(target_os = "macos")]
pub type VisualStyle = MacOsVisualStyle;

#[derive(Default, Clone, Debug)]
pub struct MofuReplaceResult {
    pub mica_dirty: bool
}

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
#[serde(default, rename_all = "camelCase")]
pub struct MofuConfig {
    pub window_style: VisualStyle
}

impl MofuConfig {
    pub fn replace(&mut self, other: Self, app: &AppHandle) -> MofuReplaceResult {
        use window_vibrancy::*;

        let original = self.window_style.clone();
        self.window_style = other.window_style;

        let mut mica_dirty = false;

        if self.window_style != original {
            for window in app.webview_windows().values() {
                match original {
                    #[cfg(target_os = "windows")]
                    VisualStyle::Mica => {
                        let _ = clear_mica(window);
                    }

                    #[cfg(target_os = "windows")]
                    VisualStyle::Acrylic => {
                        let _ = clear_acrylic(window);
                    }

                    #[cfg(target_os = "macos")]
                    VisualStyle::Vibrancy => {
                        let _ = clear_vibrancy(window);
                    }

                    #[cfg(target_os = "macos")]
                    VisualStyle::Glass => {
                        // TODO
                    }

                    _ => {}
                }

                match self.window_style {
                    #[cfg(target_os = "windows")]
                    VisualStyle::Mica => {
                        // let _ = apply_mica(window, Some(false));
                        mica_dirty = true;
                    }

                    #[cfg(target_os = "windows")]
                    VisualStyle::Acrylic => {
                        let _ = apply_acrylic(window, None);
                    }

                    #[cfg(target_os = "macos")]
                    VisualStyle::Vibrancy => {
                        let _ = apply_vibrancy(&window_, NSVisualEffectMaterial::HudWindow, None, None);
                    }

                    #[cfg(target_os = "macos")]
                    VisualStyle::Glass => {
                        // TODO
                    }

                    _ => {}
                }
            }
        }

        MofuReplaceResult { 
            mica_dirty
        }
    }
}

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
#[serde(default, rename_all = "camelCase")]
pub struct AppConfig {
    pub dark_mode: bool
}

impl AppConfig {
    pub fn replace(&mut self, other: Self, app: &AppHandle) {
        self.dark_mode = other.dark_mode;
    }
}

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
#[serde(default, rename_all = "camelCase")]
pub struct Config {
    pub speech: SpeechConfig,
    pub app: AppConfig,
    pub mofu: MofuConfig,
}

impl Config {
    pub fn replace(&mut self, other: Self, app: &AppHandle) {
        let dark_dirty = self.app.dark_mode != other.app.dark_mode;

        self.speech.replace(other.speech, app);
        self.app.replace(other.app, app);

        let result = self.mofu.replace(other.mofu, app);
        
        #[cfg(target_os = "windows")]
        {
            if result.mica_dirty || (self.mofu.window_style == VisualStyle::Mica && dark_dirty) {
                self.apply_new_mica(app);
            }
        }
    }

    #[cfg(target_os = "windows")]
    fn apply_new_mica(&self, app: &AppHandle) {
        for window in app.webview_windows().values() {
            let _ = window_vibrancy::apply_mica(window, Some(self.app.dark_mode));
        }
    }
}

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
#[serde(default, rename_all = "camelCase")]
pub struct RgbColor {
    r: u8,
    g: u8,
    b: u8,
}

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
#[serde(default, rename_all = "camelCase")]
pub struct Service {
    pub id: String,
    pub name: String,
    pub url: String,
    pub color: RgbColor,
    pub enabled: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(default, rename_all = "camelCase")]
pub struct Prefs {
    pub config: Config,
    pub services: Vec<Service>,
}

impl Prefs {
    pub fn try_from_onecomme(app: &AppHandle) -> Result<Self> {
        let mut path = app.path().data_dir()?;
        path.push("onecomme");
        path.push("config.json");

        println!("OneComme config.json path: {:?}", &path);

        let str = fs::read_to_string(path)?;
        let prefs = serde_json::from_str(&str)?;
        Ok(prefs)
    }

    fn save(&self, app: &AppHandle) -> Result<()> {
        let str = serde_json::to_string_pretty(self)?;
        let mut path = app.path().app_data_dir()?;

        let dir = path.clone();
        fs::create_dir_all(dir)?;

        path.push("config.json");

        println!("Saving config to {:?}", &path);
        fs::write(path, str)?;
        Ok(())
    }

    fn set(&mut self, value: Self, app: &AppHandle) -> Result<()> {
        self.config.replace(value.config, app);
        self.services = value.services;
        self.save(app)
    }

    pub fn get_service_by_id(&self, id: &str) -> Option<&Service> {
        self.services.iter().find(|s| s.id == id)
    }

    pub fn get_service_by_url(&self, url: &str) -> Option<&Service> {
        self.services.iter().find(|s| s.url == url)
    }
}

impl Default for Prefs {
    fn default() -> Self {
        Self { 
            config: Default::default(),
            services: vec![] 
        }
    }
}
