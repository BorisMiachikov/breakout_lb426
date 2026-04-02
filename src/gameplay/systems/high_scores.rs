use bevy::prelude::*;

use crate::gameplay::resources::{
    CampaignManifest, CurrentLevelIndex, HighScores, LatestRecordedRun, Score,
};

pub fn record_game_over_score(
    score: Res<Score>,
    current_level: Res<CurrentLevelIndex>,
    manifest: Res<CampaignManifest>,
    mut high_scores: ResMut<HighScores>,
    mut latest_recorded_run: ResMut<LatestRecordedRun>,
) {
    let level_reached = (current_level.0 + 1).min(manifest.levels.len());
    latest_recorded_run.0 = high_scores.record_run("Player", score.0, "Game Over", level_reached);
}

pub fn record_victory_score(
    score: Res<Score>,
    manifest: Res<CampaignManifest>,
    mut high_scores: ResMut<HighScores>,
    mut latest_recorded_run: ResMut<LatestRecordedRun>,
) {
    latest_recorded_run.0 =
        high_scores.record_run("Player", score.0, "Victory", manifest.levels.len());
}

pub fn clear_latest_recorded_run(mut latest_recorded_run: ResMut<LatestRecordedRun>) {
    latest_recorded_run.0 = None;
}
