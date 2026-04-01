use bevy::prelude::Resource;
use serde::{Deserialize, Serialize};
use std::fs;

const HIGH_SCORES_PATH: &str = "high_scores.json";
const MAX_HIGH_SCORES: usize = 10;

#[derive(Serialize, Deserialize, Clone)]
pub struct HighScoreEntry {
    pub score: u32,
    pub result: String,
    pub level_reached: usize,
}

#[derive(Resource, Serialize, Deserialize, Default)]
pub struct HighScores {
    pub entries: Vec<HighScoreEntry>,
}

impl HighScores {
    pub fn load() -> Self {
        if let Ok(data) = fs::read_to_string(HIGH_SCORES_PATH) {
            if let Ok(scores) = serde_json::from_str(&data) {
                return scores;
            }
        }

        let default = Self::default();
        default.save();
        default
    }

    pub fn save(&self) {
        if let Ok(json) = serde_json::to_string_pretty(self) {
            let _ = fs::write(HIGH_SCORES_PATH, json);
        }
    }

    pub fn record_run(&mut self, score: u32, result: impl Into<String>, level_reached: usize) {
        if score == 0 {
            return;
        }

        self.entries.push(HighScoreEntry {
            score,
            result: result.into(),
            level_reached,
        });

        self.entries.sort_by(|a, b| {
            b.score
                .cmp(&a.score)
                .then_with(|| b.level_reached.cmp(&a.level_reached))
        });
        self.entries.truncate(MAX_HIGH_SCORES);
        self.save();
    }
}
