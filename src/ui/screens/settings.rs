use bevy::prelude::*;

use crate::app::states::GameState;
use crate::core::config::GameConfig;

#[derive(Component)]
pub struct SettingsUI;

#[derive(Component)]
pub struct SettingsItem {
    pub index: usize,
}

#[derive(Resource)]
pub struct SettingsState {
    pub selected: usize,
}

const SETTINGS_ITEMS: [&str; 3] = ["Music", "SFX", "Back"];

pub fn setup_settings_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    config: Res<GameConfig>,
) {
    commands.insert_resource(SettingsState { selected: 0 });

    let font = asset_server.load("fonts/FiraSans-Bold.ttf");

    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                row_gap: Val::Px(10.0),
                ..default()
            },
            SettingsUI,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("SETTINGS"),
                TextFont {
                    font: font.clone(),
                    font_size: 64.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                Node {
                    align_self: AlignSelf::Center,
                    ..default()
                },
            ));

            parent.spawn((
                Text::new(format!("Music: {:.1}", config.music_volume)),
                TextFont {
                    font: font.clone(),
                    font_size: 40.0,
                    ..default()
                },
                TextColor(Color::srgb(1.0, 1.0, 0.0)),
                SettingsItem { index: 0 },
                Node {
                    align_self: AlignSelf::Center,
                    ..default()
                },
            ));

            parent.spawn((
                Text::new(format!("SFX: {:.1}", config.sfx_volume)),
                TextFont {
                    font: font.clone(),
                    font_size: 40.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                SettingsItem { index: 1 },
                Node {
                    align_self: AlignSelf::Center,
                    ..default()
                },
            ));

            parent.spawn((
                Text::new("Back"),
                TextFont {
                    font: font.clone(),
                    font_size: 40.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                SettingsItem { index: 2 },
                Node {
                    align_self: AlignSelf::Center,
                    ..default()
                },
            ));

            parent.spawn((
                Text::new("Left/Right change value | Enter = save | Esc = back"),
                TextFont {
                    font,
                    font_size: 24.0,
                    ..default()
                },
                TextColor(Color::srgb(0.5, 0.5, 0.5)),
                Node {
                    align_self: AlignSelf::Center,
                    margin: UiRect::top(Val::Px(20.0)),
                    ..default()
                },
            ));
        });
}

pub fn cleanup_settings_ui(
    mut commands: Commands,
    query: Query<Entity, With<SettingsUI>>,
) {
    for e in query.iter() {
        commands.entity(e).despawn();
    }
    commands.remove_resource::<SettingsState>();
}

pub fn settings_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut settings_state: ResMut<SettingsState>,
    mut config: ResMut<GameConfig>,
    mut next_state: ResMut<NextState<GameState>>,
    mut query: Query<(&SettingsItem, &mut TextColor, &mut Text)>,
) {
    let mut changed = false;
    let mut update_display = false;

    if keys.just_pressed(KeyCode::ArrowUp) {
        if settings_state.selected > 0 {
            settings_state.selected -= 1;
        }
        update_display = true;
    }

    if keys.just_pressed(KeyCode::ArrowDown) {
        if settings_state.selected < SETTINGS_ITEMS.len() - 1 {
            settings_state.selected += 1;
        }
        update_display = true;
    }

    if settings_state.selected == 0 {
        if keys.just_pressed(KeyCode::ArrowLeft) {
            config.music_volume = (config.music_volume - 0.1).clamp(0.0, 1.0);
            changed = true;
            update_display = true;
        }
        if keys.just_pressed(KeyCode::ArrowRight) {
            config.music_volume = (config.music_volume + 0.1).clamp(0.0, 1.0);
            changed = true;
            update_display = true;
        }
    } else if settings_state.selected == 1 {
        if keys.just_pressed(KeyCode::ArrowLeft) {
            config.sfx_volume = (config.sfx_volume - 0.1).clamp(0.0, 1.0);
            changed = true;
            update_display = true;
        }
        if keys.just_pressed(KeyCode::ArrowRight) {
            config.sfx_volume = (config.sfx_volume + 0.1).clamp(0.0, 1.0);
            changed = true;
            update_display = true;
        }
    }

    if keys.just_pressed(KeyCode::Enter) || keys.just_pressed(KeyCode::NumpadEnter) {
        if settings_state.selected == 2 {
            config.save();
            info!("Config saved");
            next_state.set(GameState::MainMenu);
        } else {
            config.save();
            info!("Config saved");
        }
    }

    if keys.just_pressed(KeyCode::Escape) {
        next_state.set(GameState::MainMenu);
    }

    if update_display || changed {
        for (item, mut color, mut text) in query.iter_mut() {
            if item.index == settings_state.selected {
                color.0 = Color::srgb(1.0, 1.0, 0.0);
            } else {
                color.0 = Color::WHITE;
            }

            if item.index == 0 {
                text.0 = format!("Music: {:.1}", config.music_volume);
            } else if item.index == 1 {
                text.0 = format!("SFX: {:.1}", config.sfx_volume);
            }
        }

        if changed {
            info!("Music: {:.1}, SFX: {:.1}", config.music_volume, config.sfx_volume);
        }
    }
}
