use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::app::states::GameState;
use crate::core::audio::{play_sfx, AudioAssets};
use crate::core::config::GameConfig;
use crate::gameplay::resources::{CampaignManifest, CurrentLevelIndex};
use crate::ui::components::{spawn_screen_background, spawn_screen_header};
use crate::ui::screens::style::*;

#[derive(Component)]
pub struct LevelCompleteUI;

#[derive(Component)]
pub struct LevelCompletePanel;

#[derive(Component)]
pub struct LevelCompleteCard;

#[derive(Resource)]
pub struct LevelCompleteLayout {
    pub compact: bool,
}

const LEVEL_COMPLETE_COMPACT_HEIGHT_THRESHOLD: f32 = 760.0;

fn level_complete_panel_color() -> BackgroundColor {
    BackgroundColor(Color::srgba(0.04, 0.08, 0.12, 0.74))
}

fn level_complete_card_color() -> BackgroundColor {
    BackgroundColor(Color::srgba(0.09, 0.15, 0.20, 0.66))
}

fn level_complete_panel_node(compact: bool) -> Node {
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

pub fn setup_level_complete(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    current_level: Res<CurrentLevelIndex>,
    manifest: Res<CampaignManifest>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let compact = window_query
        .single()
        .map(|window| is_level_complete_compact(window.height()))
        .unwrap_or(false);
    commands.insert_resource(LevelCompleteLayout { compact });

    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let current_stage = (current_level.0 + 1).min(manifest.levels.len());

    commands
        .spawn((
            screen_root_node(),
            screen_overlay_color(),
            LevelCompleteUI,
        ))
        .with_children(|parent| {
            spawn_screen_background(parent, &asset_server, "backgrounds/game_sat.png");

            parent
                .spawn((level_complete_panel_node(compact), level_complete_panel_color(), LevelCompletePanel))
                .with_children(|parent| {
                    spawn_screen_header(parent, &font, "STAGE CLEARED", "LEVEL COMPLETE", "");

                    parent
                        .spawn((adaptive_section_card_node(MENU_BUTTON_WIDTH, compact), level_complete_card_color(), LevelCompleteCard))
                        .with_children(|parent| {
                            parent.spawn((
                                Text::new(format!("Next Stage: Level {}", current_stage)),
                                TextFont {
                                    font: font.clone(),
                                    font_size: ITEM_SIZE - 4.0,
                                    ..default()
                                },
                                TextColor(accent_text()),
                            ));

                            parent.spawn((
                                Text::new("Press Enter or Space to continue the campaign."),
                                TextFont {
                                    font,
                                    font_size: SUBTITLE_SIZE - 4.0,
                                    ..default()
                                },
                                TextColor(muted_text()),
                            ));
                        });
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
    commands.remove_resource::<LevelCompleteLayout>();
}

pub fn level_complete_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    audio_assets: Res<AudioAssets>,
    config: Res<GameConfig>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keys.just_pressed(KeyCode::Enter)
        || keys.just_pressed(KeyCode::NumpadEnter)
        || keys.just_pressed(KeyCode::Space)
    {
        play_sfx(&mut commands, &audio_assets.bounce, &config, 0.45);
        next_state.set(GameState::Playing);
    }
}

pub fn adapt_level_complete_layout(
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut layout: ResMut<LevelCompleteLayout>,
    mut panel_query: Query<&mut Node, (With<LevelCompletePanel>, Without<LevelCompleteCard>)>,
    mut card_query: Query<&mut Node, (With<LevelCompleteCard>, Without<LevelCompletePanel>)>,
) {
    let Ok(window) = window_query.single() else {
        return;
    };

    let compact = is_level_complete_compact(window.height());
    if layout.compact == compact {
        return;
    }

    layout.compact = compact;

    for mut node in panel_query.iter_mut() {
        *node = level_complete_panel_node(compact);
    }

    for mut node in card_query.iter_mut() {
        *node = adaptive_section_card_node(MENU_BUTTON_WIDTH, compact);
    }
}

fn is_level_complete_compact(window_height: f32) -> bool {
    window_height <= LEVEL_COMPLETE_COMPACT_HEIGHT_THRESHOLD
}
