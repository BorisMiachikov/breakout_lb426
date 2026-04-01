use bevy::prelude::*;

use crate::app::states::GameState;
use crate::gameplay::components::brick::Brick;
use crate::gameplay::resources::{CampaignManifest, CurrentLevelIndex};

pub fn check_level_complete(
    bricks: Query<Entity, With<Brick>>,
    current_level: Res<CurrentLevelIndex>,
    manifest: Res<CampaignManifest>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if !bricks.is_empty() {
        return;
    }

    if current_level.0 + 1 < manifest.levels.len() {
        next_state.set(GameState::LevelComplete);
    } else {
        next_state.set(GameState::Victory);
    }
}

pub fn advance_to_next_level(
    manifest: Res<CampaignManifest>,
    mut current_level: ResMut<CurrentLevelIndex>,
) {
    if current_level.0 + 1 < manifest.levels.len() {
        current_level.0 += 1;
    }
}

pub fn reset_campaign_progress(mut current_level: ResMut<CurrentLevelIndex>) {
    current_level.0 = 0;
}
