use bevy::prelude::*;

use crate::gameplay::resources::{CampaignManifest, CurrentLevelIndex, HighScores, Score};

pub fn record_game_over_score(
    score: Res<Score>,
    current_level: Res<CurrentLevelIndex>,
    manifest: Res<CampaignManifest>,
    mut high_scores: ResMut<HighScores>,
) {
    let level_reached = (current_level.0 + 1).min(manifest.levels.len());
    high_scores.record_run(score.0, "Game Over", level_reached);
}

pub fn record_victory_score(
    score: Res<Score>,
    manifest: Res<CampaignManifest>,
    mut high_scores: ResMut<HighScores>,
) {
    high_scores.record_run(score.0, "Victory", manifest.levels.len());
}
