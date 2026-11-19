use std::fs;

use eyre::Result;
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager, path::BaseDirectory};

pub mod commands;

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
        Self { services: vec![] }
    }
}
