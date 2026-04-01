use bevy::prelude::*;

use crate::app::states::GameState;
use crate::gameplay::resources::{Lives, Score};
use crate::gameplay::spawn::GameEntity;

#[derive(Component)]
pub struct GameOverText;

pub fn setup_game_over(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Text::new("GAME OVER\nPress SPACE to restart"),
        TextFont {
            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
            font_size: 48.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Percent(40.0),
            left: Val::Percent(25.0),
            ..default()
        },
        GameOverText,
    ));
}

pub fn cleanup_game_over(
    mut commands: Commands,
    query: Query<Entity, With<GameOverText>>,
) {
    for e in query.iter() {
        commands.entity(e).despawn();
    }
}

pub fn restart_game(
    keys: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    game_entities: Query<Entity, With<GameEntity>>,
    mut lives: ResMut<Lives>,
    mut score: ResMut<Score>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keys.just_pressed(KeyCode::Space) {
        for entity in game_entities.iter() {
            commands.entity(entity).despawn();
        }

        lives.0 = 3;
        score.0 = 0;
        next_state.set(GameState::Playing);
    }
}
