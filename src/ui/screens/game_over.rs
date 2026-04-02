use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::app::states::GameState;
use crate::gameplay::resources::{CurrentLevelIndex, Lives, Score};
use crate::gameplay::spawn::GameEntity;
use crate::ui::components::{spawn_screen_background, spawn_screen_header};
use crate::ui::screens::style::*;

#[derive(Component)]
pub struct GameOverUI;

#[derive(Component)]
pub struct GameOverPanel;

#[derive(Component)]
pub struct GameOverSummaryCard;

#[derive(Component)]
pub struct GameOverNextStepCard;

#[derive(Resource)]
pub struct GameOverLayout {
    pub compact: bool,
}

const GAME_OVER_COMPACT_HEIGHT_THRESHOLD: f32 = 760.0;

fn game_over_panel_color() -> BackgroundColor {
    BackgroundColor(Color::srgba(0.08, 0.04, 0.07, 0.82))
}

fn game_over_card_color() -> BackgroundColor {
    BackgroundColor(Color::srgba(0.14, 0.08, 0.12, 0.76))
}

fn game_over_panel_node(compact: bool) -> Node {
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

pub fn setup_game_over(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    score: Res<Score>,
    current_level: Res<CurrentLevelIndex>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let compact = window_query
        .single()
        .map(|window| is_game_over_compact(window.height()))
        .unwrap_or(false);
    commands.insert_resource(GameOverLayout { compact });

    let font = asset_server.load("fonts/FiraSans-Bold.ttf");

    commands
        .spawn((screen_root_node(), screen_overlay_color(), GameOverUI))
        .with_children(|parent| {
            spawn_screen_background(parent, &asset_server, "backgrounds/game.png");

            parent
                .spawn((game_over_panel_node(compact), game_over_panel_color(), GameOverPanel))
                .with_children(|parent| {
                    spawn_screen_header(parent, &font, "RUN ENDED", "GAME OVER", "");

                    parent
                        .spawn((adaptive_section_card_node(MENU_BUTTON_WIDTH, compact), game_over_card_color(), GameOverSummaryCard))
                        .with_children(|parent| {
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
                                Text::new(format!("Reached Level {}", current_level.0 + 1)),
                                TextFont {
                                    font: font.clone(),
                                    font_size: ITEM_SIZE - 8.0,
                                    ..default()
                                },
                                TextColor(muted_text()),
                            ));
                        });

                    parent
                        .spawn((adaptive_section_card_node(MENU_BUTTON_WIDTH, compact), game_over_card_color(), GameOverNextStepCard))
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
                                Text::new("Space restarts the campaign immediately. Enter opens the high score board."),
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

pub fn cleanup_game_over(
    mut commands: Commands,
    query: Query<Entity, With<GameOverUI>>,
) {
    for e in query.iter() {
        commands.entity(e).despawn();
    }
    commands.remove_resource::<GameOverLayout>();
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
    } else if keys.just_pressed(KeyCode::Enter) || keys.just_pressed(KeyCode::NumpadEnter) {
        next_state.set(GameState::HighScores);
    }
}

pub fn adapt_game_over_layout(
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut layout: ResMut<GameOverLayout>,
    mut panel_query: Query<
        &mut Node,
        (
            With<GameOverPanel>,
            Without<GameOverSummaryCard>,
            Without<GameOverNextStepCard>,
        ),
    >,
    mut summary_query: Query<
        &mut Node,
        (
            With<GameOverSummaryCard>,
            Without<GameOverPanel>,
            Without<GameOverNextStepCard>,
        ),
    >,
    mut next_step_query: Query<
        &mut Node,
        (
            With<GameOverNextStepCard>,
            Without<GameOverPanel>,
            Without<GameOverSummaryCard>,
        ),
    >,
) {
    let Ok(window) = window_query.single() else {
        return;
    };

    let compact = is_game_over_compact(window.height());
    if layout.compact == compact {
        return;
    }

    layout.compact = compact;

    for mut node in panel_query.iter_mut() {
        *node = game_over_panel_node(compact);
    }

    for mut node in summary_query.iter_mut() {
        *node = adaptive_section_card_node(MENU_BUTTON_WIDTH, compact);
    }

    for mut node in next_step_query.iter_mut() {
        *node = adaptive_section_card_node(MENU_BUTTON_WIDTH, compact);
    }
}

fn is_game_over_compact(window_height: f32) -> bool {
    window_height <= GAME_OVER_COMPACT_HEIGHT_THRESHOLD
}
