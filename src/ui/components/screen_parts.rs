use bevy::prelude::*;

use crate::ui::screens::style::*;

#[derive(Component)]
pub struct ScreenBackground;

#[derive(Component)]
pub struct ActionCardTitle;

#[derive(Component)]
pub struct ActionCardDetail;

pub fn spawn_screen_header(
    parent: &mut ChildSpawnerCommands,
    font: &Handle<Font>,
    eyebrow: &str,
    title: &str,
    subtitle: &str,
) {
    parent
        .spawn(header_block_node())
        .with_children(|parent| {
            parent.spawn((
                Text::new(eyebrow),
                TextFont {
                    font: font.clone(),
                    font_size: SUBTITLE_SIZE - 4.0,
                    ..default()
                },
                TextColor(secondary_accent_text()),
            ));

            parent.spawn((
                Text::new(title),
                TextFont {
                    font: font.clone(),
                    font_size: TITLE_SIZE,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));

            parent.spawn((
                Text::new(subtitle),
                TextFont {
                    font: font.clone(),
                    font_size: SUBTITLE_SIZE - 3.0,
                    ..default()
                },
                TextColor(muted_text()),
            ));
        });
}

pub fn spawn_screen_background(
    parent: &mut ChildSpawnerCommands,
    asset_server: &AssetServer,
    path: &'static str,
) {
    parent.spawn((
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(0.0),
            right: Val::Px(0.0),
            top: Val::Px(0.0),
            bottom: Val::Px(0.0),
            ..default()
        },
        ImageNode::new(asset_server.load(path)),
        ScreenBackground,
    ));
}

pub fn spawn_action_card(
    parent: &mut ChildSpawnerCommands,
    font: &Handle<Font>,
    title: &str,
    detail: &str,
    is_selected: bool,
    enabled: bool,
) {
    parent.spawn((
        Text::new(title),
        TextFont {
            font: font.clone(),
            font_size: ITEM_SIZE - 4.0,
            ..default()
        },
        TextColor(action_card_title_color(is_selected, enabled)),
        ActionCardTitle,
    ));

    parent.spawn((
        Text::new(detail),
        TextFont {
            font: font.clone(),
            font_size: SUBTITLE_SIZE - 8.0,
            ..default()
        },
        TextColor(action_card_detail_color(is_selected, enabled)),
        ActionCardDetail,
    ));
}

pub fn action_card_title_color(is_selected: bool, enabled: bool) -> Color {
    if !enabled {
        disabled_text()
    } else if is_selected {
        Color::srgb(0.82, 0.95, 1.0)
    } else {
        Color::srgba(1.0, 1.0, 1.0, 0.92)
    }
}

pub fn action_card_detail_color(is_selected: bool, enabled: bool) -> Color {
    if !enabled {
        disabled_text()
    } else if is_selected {
        Color::srgba(0.82, 0.93, 0.98, 0.90)
    } else {
        Color::srgba(0.72, 0.78, 0.84, 0.70)
    }
}
