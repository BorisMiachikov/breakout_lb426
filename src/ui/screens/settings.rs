use bevy::prelude::*;

use crate::app::states::GameState;
use crate::core::config::GameConfig;
use crate::ui::screens::style::*;

#[derive(Component)]
pub struct SettingsUI;

#[derive(Component)]
pub struct SettingsRow {
    pub index: usize,
}

#[derive(Component)]
pub struct SettingsTitleLabel {
    pub index: usize,
}

#[derive(Component)]
pub struct SettingsValueLabel {
    pub index: usize,
}

#[derive(Component)]
pub struct SettingsAdjustButton {
    pub index: usize,
    pub delta: f32,
}

#[derive(Component)]
pub struct SettingsActionButton {
    pub index: usize,
}

#[derive(Resource)]
pub struct SettingsState {
    pub selected: usize,
}

const SETTINGS_ITEMS: [&str; 4] = ["Music", "SFX", "Save", "Back"];

pub fn setup_settings_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    config: Res<GameConfig>,
) {
    commands.insert_resource(SettingsState { selected: 0 });

    let font = asset_server.load("fonts/FiraSans-Bold.ttf");

    commands
        .spawn((
            screen_root_node(),
            screen_overlay_color(),
            SettingsUI,
        ))
        .with_children(|parent| {
            parent
                .spawn((screen_panel_node(), panel_color()))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("SETTINGS"),
                        TextFont {
                            font: font.clone(),
                            font_size: TITLE_SIZE,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                        Node {
                            margin: UiRect::bottom(Val::Px(10.0)),
                            ..default()
                        },
                    ));

                    spawn_adjust_row(parent, &font, 0, &config, true);
                    spawn_adjust_row(parent, &font, 1, &config, false);
                    spawn_action_row(parent, &font, 2, "Save", false);
                    spawn_action_row(parent, &font, 3, "Back", false);

                    parent.spawn((
                        Text::new("Mouse: use - / + buttons | Keyboard: arrows + Enter | Esc = back"),
                        TextFont {
                            font,
                            font_size: SUBTITLE_SIZE,
                            ..default()
                        },
                        TextColor(muted_text()),
                        Node {
                            margin: UiRect::top(Val::Px(10.0)),
                            ..default()
                        },
                    ));
                });
        });
}

fn spawn_adjust_row(
    parent: &mut ChildSpawnerCommands,
    font: &Handle<Font>,
    index: usize,
    config: &GameConfig,
    selected: bool,
) {
    parent
        .spawn((
            Node {
                width: Val::Px(320.0),
                padding: UiRect::axes(Val::Px(16.0), Val::Px(10.0)),
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                ..default()
            },
            if selected { selected_color() } else { idle_color() },
            SettingsRow { index },
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new(settings_title(index)),
                TextFont {
                    font: font.clone(),
                    font_size: ITEM_SIZE - 10.0,
                    ..default()
                },
                TextColor(if selected {
                    accent_text()
                } else {
                    Color::WHITE
                }),
                SettingsTitleLabel { index },
            ));

            parent
                .spawn((
                    Node {
                        column_gap: Val::Px(10.0),
                        align_items: AlignItems::Center,
                        ..default()
                    },
                ))
                .with_children(|parent| {
                    spawn_adjust_button(parent, font, index, -0.1, "-");
                    parent.spawn((
                        Text::new(settings_value(index, config)),
                        TextFont {
                            font: font.clone(),
                            font_size: ITEM_SIZE - 6.0,
                            ..default()
                        },
                        TextColor(if selected {
                            accent_text()
                        } else {
                            Color::WHITE
                        }),
                        SettingsValueLabel { index },
                        Node {
                            width: Val::Px(56.0),
                            ..default()
                        },
                    ));
                    spawn_adjust_button(parent, font, index, 0.1, "+");
                });
        });
}

fn spawn_adjust_button(
    parent: &mut ChildSpawnerCommands,
    font: &Handle<Font>,
    index: usize,
    delta: f32,
    label: &str,
) {
    parent
        .spawn((
            Button,
            Node {
                width: Val::Px(44.0),
                height: Val::Px(44.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            selected_color(),
            SettingsAdjustButton { index, delta },
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new(label),
                TextFont {
                    font: font.clone(),
                    font_size: 28.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
        });
}

fn spawn_action_row(
    parent: &mut ChildSpawnerCommands,
    font: &Handle<Font>,
    index: usize,
    label: &str,
    selected: bool,
) {
    parent
        .spawn((
            Button,
            Node {
                width: Val::Px(MENU_BUTTON_WIDTH),
                padding: UiRect::axes(Val::Px(MENU_BUTTON_PADDING_X), Val::Px(MENU_BUTTON_PADDING_Y)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            if selected { selected_color() } else { idle_color() },
            SettingsRow { index },
            SettingsActionButton { index },
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new(label),
                TextFont {
                    font: font.clone(),
                    font_size: ITEM_SIZE,
                    ..default()
                },
                TextColor(if selected {
                    accent_text()
                } else {
                    Color::WHITE
                }),
                SettingsTitleLabel { index },
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
) {
    if keys.just_pressed(KeyCode::ArrowUp) && settings_state.selected > 0 {
        settings_state.selected -= 1;
    }

    if keys.just_pressed(KeyCode::ArrowDown) && settings_state.selected < SETTINGS_ITEMS.len() - 1 {
        settings_state.selected += 1;
    }

    if keys.just_pressed(KeyCode::ArrowLeft) {
        adjust_selected_setting(settings_state.selected, -0.1, &mut config);
    }

    if keys.just_pressed(KeyCode::ArrowRight) {
        adjust_selected_setting(settings_state.selected, 0.1, &mut config);
    }

    if keys.just_pressed(KeyCode::Enter) || keys.just_pressed(KeyCode::NumpadEnter) {
        activate_settings_item(settings_state.selected, &mut config, &mut next_state);
    }

    if keys.just_pressed(KeyCode::Escape) {
        next_state.set(GameState::MainMenu);
    }
}

pub fn settings_mouse_input(
    mut adjust_query: Query<
        (&Interaction, &SettingsAdjustButton),
        (
            Changed<Interaction>,
            With<Button>,
            With<SettingsAdjustButton>,
            Without<SettingsActionButton>,
        ),
    >,
    mut action_query: Query<
        (&Interaction, &SettingsActionButton),
        (
            Changed<Interaction>,
            With<Button>,
            With<SettingsActionButton>,
            Without<SettingsAdjustButton>,
        ),
    >,
    mut settings_state: ResMut<SettingsState>,
    mut config: ResMut<GameConfig>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for (interaction, button) in adjust_query.iter_mut() {
        match *interaction {
            Interaction::Hovered => {
                settings_state.selected = button.index;
            }
            Interaction::Pressed => {
                settings_state.selected = button.index;
                adjust_selected_setting(button.index, button.delta, &mut config);
            }
            Interaction::None => {}
        }
    }

    for (interaction, button) in action_query.iter_mut() {
        match *interaction {
            Interaction::Hovered => {
                settings_state.selected = button.index;
            }
            Interaction::Pressed => {
                settings_state.selected = button.index;
                activate_settings_item(button.index, &mut config, &mut next_state);
            }
            Interaction::None => {}
        }
    }
}

pub fn update_settings_visuals(
    settings_state: Res<SettingsState>,
    config: Res<GameConfig>,
    mut row_query: Query<(&SettingsRow, &mut BackgroundColor)>,
    mut title_query: Query<
        (&SettingsTitleLabel, &mut TextColor),
        Without<SettingsValueLabel>,
    >,
    mut value_query: Query<
        (&SettingsValueLabel, &mut Text, &mut TextColor),
        Without<SettingsTitleLabel>,
    >,
) {
    if !(settings_state.is_changed() || config.is_changed()) {
        return;
    }

    for (row, mut background) in row_query.iter_mut() {
        let is_selected = row.index == settings_state.selected;
        background.0 = if is_selected {
            selected_color().0
        } else {
            idle_color().0
        };
    }

    for (label, mut color) in title_query.iter_mut() {
        color.0 = if label.index == settings_state.selected {
            accent_text()
        } else {
            Color::WHITE
        };
    }

    for (label, mut text, mut color) in value_query.iter_mut() {
        text.0 = settings_value(label.index, &config);
        color.0 = if label.index == settings_state.selected {
            accent_text()
        } else {
            Color::WHITE
        };
    }
}

fn settings_title(index: usize) -> String {
    match index {
        0 => "Music".to_string(),
        1 => "SFX".to_string(),
        2 => "Save".to_string(),
        3 => "Back".to_string(),
        _ => String::new(),
    }
}

fn settings_value(index: usize, config: &GameConfig) -> String {
    match index {
        0 => format!("{:.1}", config.music_volume),
        1 => format!("{:.1}", config.sfx_volume),
        2 => String::new(),
        3 => String::new(),
        _ => String::new(),
    }
}

fn adjust_selected_setting(index: usize, delta: f32, config: &mut GameConfig) {
    match index {
        0 => {
            config.music_volume = (config.music_volume + delta).clamp(0.0, 1.0);
        }
        1 => {
            config.sfx_volume = (config.sfx_volume + delta).clamp(0.0, 1.0);
        }
        _ => {}
    }
}

fn activate_settings_item(
    selected: usize,
    config: &mut GameConfig,
    next_state: &mut ResMut<NextState<GameState>>,
) {
    match selected {
        2 => {
            config.save();
        }
        3 => {
            next_state.set(GameState::MainMenu);
        }
        _ => {}
    }
}
