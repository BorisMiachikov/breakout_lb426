use bevy::prelude::*;

use crate::gameplay::resources::{CampaignManifest, CurrentLevelIndex, Lives, Score};
use crate::core::camera::SIDEBAR_WIDTH;

#[derive(Component)]
pub struct PlayingHud;

#[derive(Component, Clone, Copy)]
pub enum HudValueKind {
    Score,
    Lives,
    Level,
}

pub fn setup_playing_hud(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    current_level: Res<CurrentLevelIndex>,
    manifest: Res<CampaignManifest>,
    score: Res<Score>,
    lives: Res<Lives>,
) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let sidebar_width = SIDEBAR_WIDTH - 24.0;

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
            BackgroundColor(Color::srgba(0.08, 0.09, 0.11, 0.92)),
            PlayingHud,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("RUN"),
                TextFont {
                    font: font.clone(),
                    font_size: 34.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));

            spawn_info_block(parent, &font, "Level", &format!(
                "{} / {}",
                current_level.0 + 1,
                manifest.levels.len()
            ), HudValueKind::Level, sidebar_width);

            spawn_info_block(parent, &font, "Score", &score.0.to_string(), HudValueKind::Score, sidebar_width);
            spawn_info_block(parent, &font, "Lives", &lives.0.to_string(), HudValueKind::Lives, sidebar_width);

            parent.spawn((
                Text::new("Numpad *\nSkip level"),
                TextFont {
                    font,
                    font_size: 18.0,
                    ..default()
                },
                TextColor(Color::srgb(0.72, 0.74, 0.78)),
                Node {
                    margin: UiRect::top(Val::Px(18.0)),
                    ..default()
                },
            ));
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
            Node {
                width: Val::Px(width),
                padding: UiRect::all(Val::Px(12.0)),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(6.0),
                ..default()
            },
            BackgroundColor(Color::srgba(0.15, 0.16, 0.20, 0.95)),
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new(label),
                TextFont {
                    font: font.clone(),
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::srgb(0.70, 0.73, 0.78)),
            ));

            parent.spawn((
                Text::new(value),
                TextFont {
                    font: font.clone(),
                    font_size: 28.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                value_kind,
            ));
        });
}

pub fn update_playing_hud(
    score: Res<Score>,
    lives: Res<Lives>,
    current_level: Res<CurrentLevelIndex>,
    manifest: Res<CampaignManifest>,
    mut query: Query<(&HudValueKind, &mut Text)>,
) {
    if !(score.is_changed()
        || lives.is_changed()
        || current_level.is_changed()
        || manifest.is_changed())
    {
        return;
    }

    for (kind, mut text) in query.iter_mut() {
        match kind {
            HudValueKind::Score => text.0 = score.0.to_string(),
            HudValueKind::Lives => text.0 = lives.0.to_string(),
            HudValueKind::Level => text.0 = format!("{} / {}", current_level.0 + 1, manifest.levels.len()),
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
