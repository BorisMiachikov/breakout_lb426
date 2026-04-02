use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::app::states::GameState;
use crate::core::audio::{play_sfx, AudioAssets};
use crate::core::config::GameConfig;
use crate::gameplay::resources::Score;
use crate::ui::components::{spawn_screen_background, spawn_screen_header};
use crate::ui::screens::style::*;

#[derive(Component)]
pub struct VictoryUI;

#[derive(Component)]
pub struct VictoryPanel;

#[derive(Component)]
pub struct VictoryScoreCard;

#[derive(Component)]
pub struct VictoryNextStepCard;

#[derive(Resource)]
pub struct VictoryLayout {
    pub compact: bool,
}

const VICTORY_COMPACT_HEIGHT_THRESHOLD: f32 = 760.0;

fn victory_panel_color() -> BackgroundColor {
    BackgroundColor(Color::srgba(0.04, 0.08, 0.11, 0.76))
}

fn victory_card_color() -> BackgroundColor {
    BackgroundColor(Color::srgba(0.08, 0.15, 0.20, 0.68))
}

fn victory_panel_node(compact: bool) -> Node {
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

pub fn setup_victory(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    score: Res<Score>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let compact = window_query
        .single()
        .map(|window| is_victory_compact(window.height()))
        .unwrap_or(false);
    commands.insert_resource(VictoryLayout { compact });

    let font = asset_server.load("fonts/FiraSans-Bold.ttf");

    commands
        .spawn((
            screen_root_node(),
            screen_overlay_color(),
            VictoryUI,
        ))
        .with_children(|parent| {
            spawn_screen_background(parent, &asset_server, "backgrounds/game_sat.png");

            parent
                .spawn((victory_panel_node(compact), victory_panel_color(), VictoryPanel))
                .with_children(|parent| {
                    spawn_screen_header(
                        parent,
                        &font,
                        "CAMPAIGN CLEARED",
                        "YOU WIN",
                        "All five levels are complete. Your score is now part of the persistent run history.",
                    );

                    parent
                        .spawn((adaptive_section_card_node(MENU_BUTTON_WIDTH, compact), victory_card_color(), VictoryScoreCard))
                        .with_children(|parent| {
                            parent.spawn((
                                Text::new(format!("Final Score: {}", score.0)),
                                TextFont {
                                    font: font.clone(),
                                    font_size: ITEM_SIZE - 2.0,
                                    ..default()
                                },
                                TextColor(accent_text()),
                            ));
                        });

                    parent
                        .spawn((adaptive_section_card_node(MENU_BUTTON_WIDTH, compact), victory_card_color(), VictoryNextStepCard))
                        .with_children(|parent| {
                            parent.spawn((
                                Text::new("Next Step"),
                                TextFont {
                                    font: font.clone(),
                                    font_size: SUBTITLE_SIZE,
                                    ..default()
                                },
                                TextColor(subtle_text()),
                            ));

                            parent.spawn((
                                Text::new("Enter opens the high score board. Esc returns to the main menu."),
                                TextFont {
                                    font,
                                    font_size: SUBTITLE_SIZE - 5.0,
                                    ..default()
                                },
                                TextColor(muted_text()),
                            ));
                        });
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
    commands.remove_resource::<VictoryLayout>();
}

pub fn victory_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    audio_assets: Res<AudioAssets>,
    config: Res<GameConfig>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keys.just_pressed(KeyCode::Enter) || keys.just_pressed(KeyCode::NumpadEnter) {
        play_sfx(&mut commands, &audio_assets.bounce, &config, 0.40);
        next_state.set(GameState::HighScores);
    } else if keys.just_pressed(KeyCode::Escape) {
        play_sfx(&mut commands, &audio_assets.bounce, &config, 0.40);
        next_state.set(GameState::MainMenu);
    }
}

pub fn adapt_victory_layout(
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut layout: ResMut<VictoryLayout>,
    mut panel_query: Query<
        &mut Node,
        (
            With<VictoryPanel>,
            Without<VictoryScoreCard>,
            Without<VictoryNextStepCard>,
        ),
    >,
    mut score_query: Query<
        &mut Node,
        (
            With<VictoryScoreCard>,
            Without<VictoryPanel>,
            Without<VictoryNextStepCard>,
        ),
    >,
    mut next_step_query: Query<
        &mut Node,
        (
            With<VictoryNextStepCard>,
            Without<VictoryPanel>,
            Without<VictoryScoreCard>,
        ),
    >,
) {
    let Ok(window) = window_query.single() else {
        return;
    };

    let compact = is_victory_compact(window.height());
    if layout.compact == compact {
        return;
    }

    layout.compact = compact;

    for mut node in panel_query.iter_mut() {
        *node = victory_panel_node(compact);
    }

    for mut node in score_query.iter_mut() {
        *node = adaptive_section_card_node(MENU_BUTTON_WIDTH, compact);
    }

    for mut node in next_step_query.iter_mut() {
        *node = adaptive_section_card_node(MENU_BUTTON_WIDTH, compact);
    }
}

fn is_victory_compact(window_height: f32) -> bool {
    window_height <= VICTORY_COMPACT_HEIGHT_THRESHOLD
}
