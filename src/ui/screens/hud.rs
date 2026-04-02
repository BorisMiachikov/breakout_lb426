use bevy::prelude::*;

use crate::gameplay::resources::{CampaignManifest, CurrentLevelIndex, HighScores, Lives, Score};
use crate::core::camera::SIDEBAR_WIDTH;
use crate::ui::screens::style::*;

#[derive(Component)]
pub struct PlayingHud;

#[derive(Component, Clone, Copy)]
pub enum HudValueKind {
    Score,
    Lives,
    Level,
    BestScore,
}

pub fn setup_playing_hud(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    current_level: Res<CurrentLevelIndex>,
    manifest: Res<CampaignManifest>,
    score: Res<Score>,
    lives: Res<Lives>,
    high_scores: Res<HighScores>,
) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let sidebar_width = SIDEBAR_WIDTH - 24.0;
    let best_score = high_scores.entries.first().map(|entry| entry.score).unwrap_or(0);

    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(0.0),
                right: Val::Px(0.0),
                width: Val::Px(SIDEBAR_WIDTH),
                height: Val::Percent(100.0),
                padding: UiRect::axes(Val::Px(18.0), Val::Px(18.0)),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::FlexStart,
                row_gap: Val::Px(18.0),
                ..default()
            },
            panel_color(),
            PlayingHud,
        ))
        .with_children(|parent| {
            parent
                .spawn(header_block_node())
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("RUN HUD"),
                        TextFont {
                            font: font.clone(),
                            font_size: SUBTITLE_SIZE - 2.0,
                            ..default()
                        },
                        TextColor(secondary_accent_text()),
                    ));

                    parent.spawn((
                        Text::new("Campaign Status"),
                        TextFont {
                            font: font.clone(),
                            font_size: ITEM_SIZE - 2.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));
                });

            spawn_info_block(parent, &font, "Level", &format!(
                "{} / {}",
                current_level.0 + 1,
                manifest.levels.len()
            ), HudValueKind::Level, sidebar_width);

            spawn_info_block(parent, &font, "Score", &score.0.to_string(), HudValueKind::Score, sidebar_width);
            spawn_info_block(parent, &font, "Lives", &lives.0.to_string(), HudValueKind::Lives, sidebar_width);
            spawn_info_block(parent, &font, "Best Score", &best_score.to_string(), HudValueKind::BestScore, sidebar_width);

            parent
                .spawn((section_card_node(sidebar_width), card_color()))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("Quick Tips"),
                        TextFont {
                            font: font.clone(),
                            font_size: 18.0,
                            ..default()
                        },
                        TextColor(subtle_text()),
                    ));

                    parent.spawn((
                        Text::new("Esc pauses the run.\nNumpad * skips the current level."),
                        TextFont {
                            font,
                            font_size: 17.0,
                            ..default()
                        },
                        TextColor(muted_text()),
                    ));
                });
        });
}

fn spawn_info_block(
    parent: &mut ChildSpawnerCommands,
    font: &Handle<Font>,
    label: &str,
    value: &str,
    value_kind: HudValueKind,
    width: f32,
) {
    parent
        .spawn((
            section_card_node(width),
            card_color(),
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new(label),
                TextFont {
                    font: font.clone(),
                    font_size: 16.0,
                    ..default()
                },
                TextColor(muted_text()),
            ));

            parent.spawn((
                Text::new(value),
                TextFont {
                    font: font.clone(),
                    font_size: 28.0,
                    ..default()
                },
                TextColor(if matches!(value_kind, HudValueKind::BestScore) {
                    secondary_accent_text()
                } else {
                    Color::WHITE
                }),
                value_kind,
            ));
        });
}

pub fn update_playing_hud(
    score: Res<Score>,
    lives: Res<Lives>,
    current_level: Res<CurrentLevelIndex>,
    manifest: Res<CampaignManifest>,
    high_scores: Res<HighScores>,
    mut query: Query<(&HudValueKind, &mut Text)>,
) {
    if !(score.is_changed()
        || lives.is_changed()
        || current_level.is_changed()
        || manifest.is_changed()
        || high_scores.is_changed())
    {
        return;
    }

    let best_score = high_scores.entries.first().map(|entry| entry.score).unwrap_or(0);

    for (kind, mut text) in query.iter_mut() {
        match kind {
            HudValueKind::Score => text.0 = score.0.to_string(),
            HudValueKind::Lives => text.0 = lives.0.to_string(),
            HudValueKind::Level => text.0 = format!("{} / {}", current_level.0 + 1, manifest.levels.len()),
            HudValueKind::BestScore => text.0 = best_score.to_string(),
        }
    }
}

pub fn cleanup_playing_hud(
    mut commands: Commands,
    query: Query<Entity, With<PlayingHud>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}
