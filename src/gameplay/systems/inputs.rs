use bevy::prelude::*;
use bevy::input::keyboard::KeyCode;
use bevy::window::PrimaryWindow;

use crate::app::states::GameState;
use crate::core::camera::{playfield_left, playfield_right, VIRTUAL_WIDTH};
use crate::gameplay::components::paddle::Paddle;
use crate::gameplay::components::collider::Collider;
use crate::gameplay::resources::{CampaignManifest, CurrentLevelIndex};
use crate::gameplay::spawn::{spawn_game_entities, CurrentLevelPath, GameEntity};

pub fn paddle_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Paddle>,
) {
    for mut paddle in query.iter_mut() {
        let mut direction = 0.0;

        if keyboard.pressed(KeyCode::KeyA) || keyboard.pressed(KeyCode::ArrowLeft) {
            direction -= 1.0;
        }

        if keyboard.pressed(KeyCode::KeyD) || keyboard.pressed(KeyCode::ArrowRight) {
            direction += 1.0;
        }

        paddle.direction = direction;
    }
}

pub fn game_pause(
    keys: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        next_state.set(GameState::Paused);
    }
}

pub fn paddle_mouse_control(
    windows: Query<&Window, With<PrimaryWindow>>,
    mut paddle_q: Query<(&mut Transform, &Collider, &mut Paddle)>,
) {
    let Ok(window) = windows.single() else {
        return;
    };

    let Some(cursor_position) = window.cursor_position() else {
        return;
    };

    let virtual_x = (cursor_position.x / window.width()) * VIRTUAL_WIDTH - VIRTUAL_WIDTH / 2.0;

    for (mut transform, collider, mut paddle) in paddle_q.iter_mut() {
        let half_width = collider.size.x / 2.0;
        let min_x = playfield_left() + half_width;
        let max_x = playfield_right() - half_width;

        transform.translation.x = virtual_x.clamp(min_x, max_x);
        paddle.direction = 0.0;
    }
}

pub fn debug_next_level(
    keys: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    game_entities: Query<Entity, With<GameEntity>>,
    mut current_level_path: ResMut<CurrentLevelPath>,
    mut current_level_index: ResMut<CurrentLevelIndex>,
    manifest: Res<CampaignManifest>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if !keys.just_pressed(KeyCode::NumpadMultiply) {
        return;
    }

    for entity in game_entities.iter() {
        commands.entity(entity).despawn();
    }

    if current_level_index.0 + 1 >= manifest.levels.len() {
        next_state.set(GameState::Victory);
        return;
    }

    current_level_index.0 += 1;
    spawn_game_entities(
        &mut commands,
        &asset_server,
        &mut current_level_path,
        current_level_index.0,
        &manifest,
    );
}
