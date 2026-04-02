use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::app::states::GameState;
use crate::core::audio::{play_sfx, AudioAssets};
use crate::core::config::GameConfig;
use crate::gameplay::resources::{HighScores, LatestRecordedRun};
use crate::ui::components::{spawn_screen_background, spawn_screen_header};
use crate::ui::screens::style::*;

#[derive(Component)]
pub struct HighScoresUI;

#[derive(Component)]
pub struct HighScoresPanel;

#[derive(Component)]
pub struct HighScoresEntryCard;

#[derive(Component)]
pub struct HighScoresBackButton;

#[derive(Component)]
pub struct HighScoresBackLabel;

#[derive(Component)]
pub struct HighScoresHint;

#[derive(Resource)]
pub struct HighScoresLayout {
    pub compact: bool,
}

const HIGH_SCORES_COMPACT_HEIGHT_THRESHOLD: f32 = 760.0;

fn high_scores_panel_color() -> BackgroundColor {
    BackgroundColor(Color::srgba(0.03, 0.05, 0.09, 0.56))
}

fn high_scores_entry_color() -> BackgroundColor {
    BackgroundColor(Color::srgba(0.07, 0.11, 0.16, 0.52))
}

fn high_scores_button_color() -> BackgroundColor {
    BackgroundColor(Color::srgba(0.06, 0.10, 0.15, 0.42))
}

fn high_scores_panel_node(compact: bool) -> Node {
    Node {
        width: Val::Px(WIDE_PANEL_WIDTH),
        padding: UiRect::all(Val::Px(if compact { 16.0 } else { 24.0 })),
        flex_direction: FlexDirection::Column,
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Stretch,
        row_gap: Val::Px(if compact { 10.0 } else { 14.0 }),
        ..default()
    }
}

fn high_scores_entry_node(compact: bool) -> Node {
    Node {
        width: Val::Px(WIDE_PANEL_WIDTH - 48.0),
        padding: UiRect::all(Val::Px(if compact { 10.0 } else { 14.0 })),
        flex_direction: FlexDirection::Column,
        row_gap: Val::Px(if compact { 4.0 } else { 6.0 }),
        ..default()
    }
}

pub fn setup_high_scores(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    high_scores: Res<HighScores>,
    latest_recorded_run: Res<LatestRecordedRun>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let compact = window_query
        .single()
        .map(|window| is_high_scores_compact(window.height()))
        .unwrap_or(false);
    commands.insert_resource(HighScoresLayout { compact });

    let font = asset_server.load("fonts/FiraSans-Bold.ttf");

    commands
        .spawn((screen_root_node(), screen_overlay_color(), HighScoresUI))
        .with_children(|parent| {
            spawn_screen_background(parent, &asset_server, "backgrounds/menu.png");

            parent
                .spawn((high_scores_panel_node(compact), high_scores_panel_color(), HighScoresPanel))
                .with_children(|parent| {
                    spawn_screen_header(
                        parent,
                        &font,
                        "RUN HISTORY",
                        "HIGH SCORES",
                        "Your best runs are saved here across campaign attempts.",
                    );

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
                                    let is_latest = latest_recorded_run.0 == Some(entry.run_id);
                                    let label = if is_latest { "  NEW" } else { "" };
                                    parent
                                        .spawn((high_scores_entry_node(compact), high_scores_entry_color(), HighScoresEntryCard))
                                        .with_children(|parent| {
                                            parent.spawn((
                                                Text::new(format!(
                                                    "{}. {} points  {}{}",
                                                    index + 1,
                                                    entry.score,
                                                    entry.result,
                                                    label,
                                                )),
                                                TextFont {
                                                    font: font.clone(),
                                                    font_size: ITEM_SIZE - 8.0,
                                                    ..default()
                                                },
                                                TextColor(if is_latest {
                                                    accent_text()
                                                } else {
                                                    Color::WHITE
                                                }),
                                            ));

                                            parent.spawn((
                                                Text::new(format!(
                                                    "{}  |  Level {}  |  Run #{}",
                                                    entry.player_label, entry.level_reached, entry.run_id
                                                )),
                                                TextFont {
                                                    font: font.clone(),
                                                    font_size: SUBTITLE_SIZE - 6.0,
                                                    ..default()
                                                },
                                                TextColor(if is_latest {
                                                    subtle_text()
                                                } else {
                                                    muted_text()
                                                }),
                                            ));
                                        });
                                }
                            });
                    }

                    parent
                        .spawn((
                            Button,
                            high_scores_button_color(),
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
                        Text::new("Press Esc or Enter to return to main menu"),
                        TextFont {
                            font,
                            font_size: SUBTITLE_SIZE - 2.0,
                            ..default()
                        },
                        TextColor(muted_text()),
                        HighScoresHint,
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
    commands.remove_resource::<HighScoresLayout>();
}

pub fn high_scores_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    audio_assets: Res<AudioAssets>,
    config: Res<GameConfig>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keys.just_pressed(KeyCode::Escape)
        || keys.just_pressed(KeyCode::Enter)
        || keys.just_pressed(KeyCode::NumpadEnter)
    {
        play_sfx(&mut commands, &audio_assets.bounce, &config, 0.40);
        next_state.set(GameState::MainMenu);
    }
}

pub fn high_scores_mouse_input(
    mut commands: Commands,
    audio_assets: Res<AudioAssets>,
    config: Res<GameConfig>,
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<HighScoresBackButton>)>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for interaction in interaction_query.iter_mut() {
        if *interaction == Interaction::Pressed {
            play_sfx(&mut commands, &audio_assets.bounce, &config, 0.40);
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
        background.0 = if is_active {
            selected_color().0
        } else {
            high_scores_button_color().0
        };
    }

    for mut color in label_query.iter_mut() {
        color.0 = if is_active { accent_text() } else { Color::WHITE };
    }
}

pub fn adapt_high_scores_layout(
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut layout: ResMut<HighScoresLayout>,
    mut panel_query: Query<&mut Node, (With<HighScoresPanel>, Without<HighScoresEntryCard>)>,
    mut entry_query: Query<&mut Node, (With<HighScoresEntryCard>, Without<HighScoresPanel>)>,
    mut hint_query: Query<&mut Visibility, With<HighScoresHint>>,
) {
    let Ok(window) = window_query.single() else {
        return;
    };

    let compact = is_high_scores_compact(window.height());
    if layout.compact == compact {
        return;
    }

    layout.compact = compact;

    for mut node in panel_query.iter_mut() {
        *node = high_scores_panel_node(compact);
    }

    for mut node in entry_query.iter_mut() {
        *node = high_scores_entry_node(compact);
    }

    for mut visibility in hint_query.iter_mut() {
        *visibility = if compact {
            Visibility::Hidden
        } else {
            Visibility::Inherited
        };
    }
}

fn is_high_scores_compact(window_height: f32) -> bool {
    window_height <= HIGH_SCORES_COMPACT_HEIGHT_THRESHOLD
}
