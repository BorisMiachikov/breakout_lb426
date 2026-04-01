use bevy::prelude::*;

use crate::app::states::GameState;
use crate::ui::screens::style::*;

#[derive(Component)]
pub struct LevelCompleteUI;

pub fn setup_level_complete(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");

    commands
        .spawn((
            screen_root_node(),
            screen_overlay_color(),
            LevelCompleteUI,
        ))
        .with_children(|parent| {
            parent
                .spawn((screen_panel_node(), panel_color()))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("LEVEL COMPLETE"),
                        TextFont {
                            font: font.clone(),
                            font_size: TITLE_SIZE - 8.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));

                    parent.spawn((
                        Text::new("Prepare for the next stage"),
                        TextFont {
                            font: font.clone(),
                            font_size: ITEM_SIZE - 4.0,
                            ..default()
                        },
                        TextColor(accent_text()),
                    ));

                    parent.spawn((
                        Text::new("Press Enter or Space to continue"),
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

pub fn cleanup_level_complete(
    mut commands: Commands,
    query: Query<Entity, With<LevelCompleteUI>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn level_complete_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keys.just_pressed(KeyCode::Enter)
        || keys.just_pressed(KeyCode::NumpadEnter)
        || keys.just_pressed(KeyCode::Space)
    {
        next_state.set(GameState::Playing);
    }
}
