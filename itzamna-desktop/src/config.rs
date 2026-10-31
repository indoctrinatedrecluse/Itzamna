//! Itzamna Configuration & Persistence Chamber
//! Persists user preferences and recent sanctuaries into a TOML altar.

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub active_repo_path: Option<String>,
    pub background_mode: String, // "video", "procedural", "stealth"
    pub recent_repos: Vec<String>,
    pub auto_fetch_interval_secs: u32,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            active_repo_path: Some(".".to_string()),
            background_mode: "video".to_string(),
            recent_repos: vec![".".to_string()],
            auto_fetch_interval_secs: 300,
        }
    }
}

impl AppConfig {
    pub fn config_path() -> PathBuf {
        let base = dirs_or_fallback();
        base.join("Itzamna").join("config.toml")
    }

    pub fn load() -> Self {
        let path = Self::config_path();
        if let Ok(content) = fs::read_to_string(&path) {
            if let Ok(cfg) = toml::from_str::<AppConfig>(&content) {
                return cfg;
            }
        }
        Self::default()
    }

    pub fn save(&self) -> Result<(), String> {
        let path = Self::config_path();
        if let Some(parent) = path.parent() {
            let _ = fs::create_dir_all(parent);
        }
        let serialized = toml::to_string_pretty(self).map_err(|e| e.to_string())?;
        fs::write(&path, serialized).map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn add_recent(&mut self, path: String) {
        self.recent_repos.retain(|p| p != &path);
        self.recent_repos.insert(0, path.clone());
        if self.recent_repos.len() > 10 {
            self.recent_repos.truncate(10);
        }
        self.active_repo_path = Some(path);
        let _ = self.save();
    }
}

fn dirs_or_fallback() -> PathBuf {
    if let Ok(app_data) = std::env::var("APPDATA") {
        PathBuf::from(app_data)
    } else if let Ok(home) = std::env::var("USERPROFILE").or_else(|_| std::env::var("HOME")) {
        PathBuf::from(home).join(".config")
    } else {
        PathBuf::from(".")
    }
}
