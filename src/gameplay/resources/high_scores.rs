use bevy::prelude::Resource;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

const HIGH_SCORES_PATH: &str = "high_scores.json";
const MAX_HIGH_SCORES: usize = 10;

#[derive(Serialize, Deserialize, Clone)]
pub struct HighScoreEntry {
    #[serde(default = "default_player_label")]
    pub player_label: String,
    pub score: u32,
    pub result: String,
    pub level_reached: usize,
    #[serde(default = "generate_run_id")]
    pub run_id: u64,
    #[serde(default = "current_timestamp_secs")]
    pub recorded_at_unix_secs: u64,
}

#[derive(Resource, Serialize, Deserialize, Default)]
pub struct HighScores {
    pub entries: Vec<HighScoreEntry>,
}

#[derive(Resource, Default)]
pub struct LatestRecordedRun(pub Option<u64>);

impl HighScores {
    pub fn load() -> Self {
        Self::load_from_path(HIGH_SCORES_PATH)
    }

    pub fn save(&self) {
        self.save_to_path(HIGH_SCORES_PATH);
    }

    pub fn load_from_path(path: impl AsRef<Path>) -> Self {
        let path = path.as_ref();

        if let Ok(data) = fs::read_to_string(path) {
            if let Ok(mut scores) = serde_json::from_str::<Self>(&data) {
                scores.normalize();
                return scores;
            }
        }

        let default = Self::default();
        default.save_to_path(path);
        default
    }

    pub fn save_to_path(&self, path: impl AsRef<Path>) {
        if let Ok(json) = serde_json::to_string_pretty(self) {
            let _ = fs::write(path, json);
        }
    }

    pub fn record_run(
        &mut self,
        player_label: impl Into<String>,
        score: u32,
        result: impl Into<String>,
        level_reached: usize,
    ) -> Option<u64> {
        self.record_run_inner(player_label.into(), score, result.into(), level_reached, true)
    }

    fn record_run_inner(
        &mut self,
        player_label: String,
        score: u32,
        result: String,
        level_reached: usize,
        persist: bool,
    ) -> Option<u64> {
        if score == 0 {
            return None;
        }

        let run_id = generate_run_id();
        self.entries.push(HighScoreEntry {
            player_label,
            score,
            result,
            level_reached,
            run_id,
            recorded_at_unix_secs: current_timestamp_secs(),
        });

        self.entries.sort_by(|a, b| {
            b.score
                .cmp(&a.score)
                .then_with(|| b.level_reached.cmp(&a.level_reached))
                .then_with(|| b.recorded_at_unix_secs.cmp(&a.recorded_at_unix_secs))
        });
        self.entries.truncate(MAX_HIGH_SCORES);
        if persist {
            self.save();
        }
        Some(run_id)
    }

    fn normalize(&mut self) {
        for entry in &mut self.entries {
            if entry.player_label.is_empty() {
                entry.player_label = default_player_label();
            }

            if entry.run_id == 0 {
                entry.run_id = generate_run_id();
            }

            if entry.recorded_at_unix_secs == 0 {
                entry.recorded_at_unix_secs = current_timestamp_secs();
            }
        }

        self.entries.sort_by(|a, b| {
            b.score
                .cmp(&a.score)
                .then_with(|| b.level_reached.cmp(&a.level_reached))
                .then_with(|| b.recorded_at_unix_secs.cmp(&a.recorded_at_unix_secs))
        });
        self.entries.truncate(MAX_HIGH_SCORES);
    }
}

fn default_player_label() -> String {
    "Player".to_string()
}

fn current_timestamp_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs())
        .unwrap_or(0)
}

fn generate_run_id() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_nanos() as u64)
        .unwrap_or_else(|_| current_timestamp_secs())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn load_from_path_migrates_legacy_entries() {
        let path = std::env::temp_dir().join(format!(
            "breakout_high_scores_{}_legacy.json",
            current_timestamp_secs()
        ));

        fs::write(
            &path,
            r#"{
  "entries": [
    {
      "score": 500,
      "result": "Game Over",
      "level_reached": 1
    }
  ]
}"#,
        )
        .unwrap();

        let scores = HighScores::load_from_path(&path);
        let entry = &scores.entries[0];

        assert_eq!(entry.player_label, "Player");
        assert_eq!(entry.score, 500);
        assert_ne!(entry.run_id, 0);
        assert_ne!(entry.recorded_at_unix_secs, 0);

        let _ = fs::remove_file(path);
    }

    #[test]
    fn record_run_sorts_and_limits_entries() {
        let mut scores = HighScores::default();

        for score in 1..=12 {
            scores.record_run_inner("Player".to_string(), score, "Game Over".to_string(), 1, false);
        }

        assert_eq!(scores.entries.len(), MAX_HIGH_SCORES);
        assert_eq!(scores.entries[0].score, 12);
        assert_eq!(scores.entries[MAX_HIGH_SCORES - 1].score, 3);
    }
}
