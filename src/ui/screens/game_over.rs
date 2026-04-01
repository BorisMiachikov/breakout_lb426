use bevy::prelude::*;

use crate::app::states::GameState;
use crate::gameplay::resources::{CurrentLevelIndex, Lives, Score};
use crate::gameplay::spawn::GameEntity;
use crate::ui::screens::style::*;

#[derive(Component)]
pub struct GameOverUI;

pub fn setup_game_over(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    score: Res<Score>,
) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");

    commands
        .spawn((screen_root_node(), screen_overlay_color(), GameOverUI))
        .with_children(|parent| {
            parent
                .spawn((screen_panel_node(), panel_color()))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("GAME OVER"),
                        TextFont {
                            font: font.clone(),
                            font_size: TITLE_SIZE,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));

                    parent.spawn((
                        Text::new(format!("Final Score: {}", score.0)),
                        TextFont {
                            font: font.clone(),
                            font_size: ITEM_SIZE - 4.0,
                            ..default()
                        },
                        TextColor(accent_text()),
                    ));

                    parent.spawn((
                        Text::new("Press Space to restart"),
                        TextFont {
                            font,
                            font_size: SUBTITLE_SIZE,
                            ..default()
                        },
                        TextColor(muted_text()),
                    ));
                });
        });
}

pub fn cleanup_game_over(
    mut commands: Commands,
    query: Query<Entity, With<GameOverUI>>,
) {
    for e in query.iter() {
        commands.entity(e).despawn();
    }
}

pub fn restart_game(
    keys: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    game_entities: Query<Entity, With<GameEntity>>,
    mut current_level: ResMut<CurrentLevelIndex>,
    mut lives: ResMut<Lives>,
    mut score: ResMut<Score>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keys.just_pressed(KeyCode::Space) {
        for entity in game_entities.iter() {
            commands.entity(entity).despawn();
        }

        current_level.0 = 0;
        lives.0 = 3;
        score.0 = 0;
        next_state.set(GameState::Playing);
    }
}
