use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Default)]
pub struct AppConfig {
    #[serde(default)]
    pub enabled_sinks: Option<Vec<String>>,
    #[serde(default)]
    pub volumes: HashMap<String, f32>,
}

impl AppConfig {
    pub fn is_sink_enabled(&self, name: &str) -> bool {
        match &self.enabled_sinks {
            None => true,
            Some(list) => list.iter().any(|n| n == name),
        }
    }

    pub fn set_sink_enabled(&mut self, name: &str, enabled: bool) {
        let list = self.enabled_sinks.get_or_insert_with(Vec::new);
        if enabled {
            if !list.iter().any(|n| n == name) {
                list.push(name.to_string());
            }
        } else {
            list.retain(|n| n != name);
        }
    }
}

fn config_path() -> Option<PathBuf> {
    let base = std::env::var_os("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .or_else(|| std::env::var_os("HOME").map(|h| PathBuf::from(h).join(".config")))?;
    Some(base.join("lotus-audio").join("config.json"))
}

pub fn load() -> AppConfig {
    let Some(path) = config_path() else {
        return AppConfig::default();
    };
    match std::fs::read_to_string(&path) {
        Ok(data) => serde_json::from_str(&data).unwrap_or_default(),
        Err(_) => AppConfig::default(),
    }
}

pub fn save(cfg: &AppConfig) {
    let Some(path) = config_path() else {
        return;
    };
    if let Some(dir) = path.parent() {
        if let Err(e) = std::fs::create_dir_all(dir) {
            eprintln!("lotus-audio: could not create config dir {}: {e}", dir.display());
            return;
        }
    }
    match serde_json::to_string_pretty(cfg) {
        Ok(data) => {
            if let Err(e) = std::fs::write(&path, data) {
                eprintln!("lotus-audio: could not save config {}: {e}", path.display());
            }
        }
        Err(e) => eprintln!("lotus-audio: could not serialize config: {e}"),
    }
}
