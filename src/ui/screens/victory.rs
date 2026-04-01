use bevy::prelude::*;

use crate::app::states::GameState;
use crate::gameplay::resources::Score;
use crate::ui::screens::style::*;

#[derive(Component)]
pub struct VictoryUI;

pub fn setup_victory(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    score: Res<Score>,
) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");

    commands
        .spawn((
            screen_root_node(),
            screen_overlay_color(),
            VictoryUI,
        ))
        .with_children(|parent| {
            parent
                .spawn((screen_panel_node(), panel_color()))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("YOU WIN"),
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
                            font_size: ITEM_SIZE - 2.0,
                            ..default()
                        },
                        TextColor(accent_text()),
                    ));

                    parent.spawn((
                        Text::new("Press Enter to return to main menu"),
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

pub fn cleanup_victory(
    mut commands: Commands,
    query: Query<Entity, With<VictoryUI>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn victory_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keys.just_pressed(KeyCode::Enter) || keys.just_pressed(KeyCode::NumpadEnter) {
        next_state.set(GameState::MainMenu);
    }
}
