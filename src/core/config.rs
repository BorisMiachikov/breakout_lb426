use bevy::prelude::Resource;
use serde::{Deserialize, Serialize};
use std::fs;

const CONFIG_PATH: &str = "config.json";

#[derive(Resource, Serialize, Deserialize, Clone)]
pub struct GameConfig {
    pub window_width: f32,
    pub window_height: f32,
    pub music_volume: f32,
    pub sfx_volume: f32,
}

impl Default for GameConfig {
    fn default() -> Self {
        Self {
            window_width: 960.0,
            window_height: 540.0,
            music_volume: 0.5,
            sfx_volume: 0.7,
        }
    }
}

impl GameConfig {
    pub fn load() -> Self {
        if let Ok(data) = fs::read_to_string(CONFIG_PATH) {
            if let Ok(config) = serde_json::from_str(&data) {
                return config;
            }
        }

        let default = Self::default();
        default.save();
        default
    }

    pub fn save(&self) {
        if let Ok(json) = serde_json::to_string_pretty(self) {
            let _ = fs::write(CONFIG_PATH, json);
        }
    }
}
