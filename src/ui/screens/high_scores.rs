use bevy::prelude::*;

use crate::app::states::GameState;
use crate::gameplay::resources::HighScores;
use crate::ui::screens::style::*;

#[derive(Component)]
pub struct HighScoresUI;

#[derive(Component)]
pub struct HighScoresBackButton;

#[derive(Component)]
pub struct HighScoresBackLabel;

pub fn setup_high_scores(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    high_scores: Res<HighScores>,
) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");

    commands
        .spawn((screen_root_node(), screen_overlay_color(), HighScoresUI))
        .with_children(|parent| {
            parent
                .spawn((screen_panel_node(), panel_color()))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("HIGH SCORES"),
                        TextFont {
                            font: font.clone(),
                            font_size: TITLE_SIZE - 8.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));

                    if high_scores.entries.is_empty() {
                        parent.spawn((
                            Text::new("No runs saved yet"),
                            TextFont {
                                font: font.clone(),
                                font_size: ITEM_SIZE - 6.0,
                                ..default()
                            },
                            TextColor(muted_text()),
                            Node {
                                margin: UiRect::top(Val::Px(12.0)),
                                ..default()
                            },
                        ));
                    } else {
                        parent
                            .spawn(Node {
                                width: Val::Percent(100.0),
                                flex_direction: FlexDirection::Column,
                                row_gap: Val::Px(8.0),
                                margin: UiRect::top(Val::Px(8.0)),
                                ..default()
                            })
                            .with_children(|parent| {
                                for (index, entry) in high_scores.entries.iter().enumerate() {
                                    parent.spawn((
                                        Text::new(format!(
                                            "{}. {:>5}  {}  Level {}",
                                            index + 1,
                                            entry.score,
                                            entry.result,
                                            entry.level_reached,
                                        )),
                                        TextFont {
                                            font: font.clone(),
                                            font_size: ITEM_SIZE - 10.0,
                                            ..default()
                                        },
                                        TextColor(Color::WHITE),
                                    ));
                                }
                            });
                    }

                    parent
                        .spawn((
                            Button,
                            selected_color(),
                            HighScoresBackButton,
                            Node {
                                margin: UiRect::top(Val::Px(16.0)),
                                ..menu_button_node()
                            },
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                Text::new("Back"),
                                TextFont {
                                    font: font.clone(),
                                    font_size: ITEM_SIZE,
                                    ..default()
                                },
                                TextColor(accent_text()),
                                HighScoresBackLabel,
                            ));
                        });

                    parent.spawn((
                        Text::new("Press Esc or Enter to return"),
                        TextFont {
                            font,
                            font_size: SUBTITLE_SIZE - 2.0,
                            ..default()
                        },
                        TextColor(muted_text()),
                    ));
                });
        });
}

pub fn cleanup_high_scores(
    mut commands: Commands,
    query: Query<Entity, With<HighScoresUI>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn high_scores_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keys.just_pressed(KeyCode::Escape)
        || keys.just_pressed(KeyCode::Enter)
        || keys.just_pressed(KeyCode::NumpadEnter)
    {
        next_state.set(GameState::MainMenu);
    }
}

pub fn high_scores_mouse_input(
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<HighScoresBackButton>)>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for interaction in interaction_query.iter_mut() {
        if *interaction == Interaction::Pressed {
            next_state.set(GameState::MainMenu);
        }
    }
}

pub fn update_high_scores_visuals(
    mut button_query: Query<(&Interaction, &mut BackgroundColor), With<HighScoresBackButton>>,
    mut label_query: Query<&mut TextColor, With<HighScoresBackLabel>>,
) {
    let mut is_active = false;

    for (interaction, mut background) in button_query.iter_mut() {
        is_active = *interaction == Interaction::Hovered || *interaction == Interaction::Pressed;
        background.0 = if is_active { selected_color().0 } else { idle_color().0 };
    }

    for mut color in label_query.iter_mut() {
        color.0 = if is_active { accent_text() } else { Color::WHITE };
    }
}
