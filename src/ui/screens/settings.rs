use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::app::states::GameState;
use crate::core::audio::{play_sfx, AudioAssets};
use crate::core::config::GameConfig;
use crate::ui::components::{spawn_screen_background, spawn_screen_header};
use crate::ui::screens::style::*;

#[derive(Component)]
pub struct SettingsUI;

#[derive(Component)]
pub struct SettingsPanel;

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

#[derive(Component)]
pub struct SettingsControlsCard;

#[derive(Resource)]
pub struct SettingsState {
    pub selected: usize,
}

#[derive(Resource)]
pub struct SettingsLayout {
    pub compact: bool,
}

const SETTINGS_ITEMS: [&str; 4] = ["Music", "SFX", "Save", "Back"];
const SETTINGS_COMPACT_HEIGHT_THRESHOLD: f32 = 760.0;

fn settings_panel_color() -> BackgroundColor {
    BackgroundColor(Color::srgba(0.03, 0.05, 0.09, 0.56))
}

fn settings_row_color() -> BackgroundColor {
    BackgroundColor(Color::srgba(0.07, 0.11, 0.16, 0.46))
}

fn settings_card_color() -> BackgroundColor {
    BackgroundColor(Color::srgba(0.06, 0.10, 0.15, 0.36))
}

fn settings_panel_node(compact: bool) -> Node {
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

fn settings_row_node(compact: bool) -> Node {
    Node {
        width: Val::Px(MENU_BUTTON_WIDTH),
        padding: UiRect::axes(Val::Px(16.0), Val::Px(if compact { 8.0 } else { 12.0 })),
        justify_content: JustifyContent::SpaceBetween,
        align_items: AlignItems::Center,
        ..default()
    }
}

pub fn setup_settings_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    config: Res<GameConfig>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let compact = window_query
        .single()
        .map(|window| is_settings_compact(window.height()))
        .unwrap_or(false);
    commands.insert_resource(SettingsState { selected: 0 });
    commands.insert_resource(SettingsLayout { compact });

    let font = asset_server.load("fonts/FiraSans-Bold.ttf");

    commands
        .spawn((
            screen_root_node(),
            screen_overlay_color(),
            SettingsUI,
        ))
        .with_children(|parent| {
            spawn_screen_background(parent, &asset_server, "backgrounds/menu.png");

            parent
                .spawn((settings_panel_node(compact), settings_panel_color(), SettingsPanel))
                .with_children(|parent| {
                    spawn_screen_header(
                        parent,
                        &font,
                        "RUN OPTIONS",
                        "SETTINGS",
                        "Tune the audio mix now. Window settings are still read from config.json at startup.",
                    );

                    spawn_adjust_row(parent, &font, 0, &config, true, compact);
                    spawn_adjust_row(parent, &font, 1, &config, false, compact);
                    spawn_action_row(parent, &font, 2, "Save", false, compact);
                    spawn_action_row(parent, &font, 3, "Back", false, compact);

                    parent
                        .spawn((section_card_node(MENU_BUTTON_WIDTH), settings_card_color(), SettingsControlsCard))
                        .with_children(|parent| {
                            parent.spawn((
                                Text::new("Controls"),
                                TextFont {
                                    font: font.clone(),
                                    font_size: SUBTITLE_SIZE,
                                    ..default()
                                },
                                TextColor(subtle_text()),
                            ));

                            parent.spawn((
                                Text::new("Arrow keys select rows and adjust values. Enter activates Save or Back. Esc returns to the main menu."),
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

fn spawn_adjust_row(
    parent: &mut ChildSpawnerCommands,
    font: &Handle<Font>,
    index: usize,
    config: &GameConfig,
    selected: bool,
    compact: bool,
) {
    parent
        .spawn((
            settings_row_node(compact),
            if selected {
                selected_color()
            } else {
                settings_row_color()
            },
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
                            font_size: ITEM_SIZE - 2.0,
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
            settings_card_color(),
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
    compact: bool,
) {
    parent
        .spawn((
            Button,
            settings_row_node(compact),
            if selected {
                selected_color()
            } else {
                settings_row_color()
            },
            SettingsRow { index },
            SettingsActionButton { index },
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new(label),
                TextFont {
                    font: font.clone(),
                    font_size: ITEM_SIZE - 2.0,
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
    commands.remove_resource::<SettingsLayout>();
}

pub fn settings_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    audio_assets: Res<AudioAssets>,
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
        if settings_state.selected < 2 {
            play_sfx(&mut commands, &audio_assets.bounce, &config, 0.30);
        }
    }

    if keys.just_pressed(KeyCode::ArrowRight) {
        adjust_selected_setting(settings_state.selected, 0.1, &mut config);
        if settings_state.selected < 2 {
            play_sfx(&mut commands, &audio_assets.bounce, &config, 0.30);
        }
    }

    if keys.just_pressed(KeyCode::Enter) || keys.just_pressed(KeyCode::NumpadEnter) {
        activate_settings_item(
            settings_state.selected,
            &mut commands,
            &audio_assets,
            &mut config,
            &mut next_state,
        );
    }

    if keys.just_pressed(KeyCode::Escape) {
        play_sfx(&mut commands, &audio_assets.bounce, &config, 0.40);
        next_state.set(GameState::MainMenu);
    }
}

pub fn settings_mouse_input(
    mut commands: Commands,
    audio_assets: Res<AudioAssets>,
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
                play_sfx(&mut commands, &audio_assets.bounce, &config, 0.30);
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
                activate_settings_item(
                    button.index,
                    &mut commands,
                    &audio_assets,
                    &mut config,
                    &mut next_state,
                );
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
            settings_row_color().0
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

pub fn adapt_settings_layout(
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut layout: ResMut<SettingsLayout>,
    mut panel_query: Query<&mut Node, (With<SettingsPanel>, Without<SettingsRow>)>,
    mut row_query: Query<&mut Node, (With<SettingsRow>, Without<SettingsPanel>)>,
    mut controls_query: Query<&mut Visibility, With<SettingsControlsCard>>,
) {
    let Ok(window) = window_query.single() else {
        return;
    };

    let compact = is_settings_compact(window.height());
    if layout.compact == compact {
        return;
    }

    layout.compact = compact;

    for mut node in panel_query.iter_mut() {
        *node = settings_panel_node(compact);
    }

    for mut node in row_query.iter_mut() {
        *node = settings_row_node(compact);
    }

    for mut visibility in controls_query.iter_mut() {
        *visibility = if compact {
            Visibility::Hidden
        } else {
            Visibility::Inherited
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
    commands: &mut Commands,
    audio_assets: &Res<AudioAssets>,
    config: &mut GameConfig,
    next_state: &mut ResMut<NextState<GameState>>,
) {
    match selected {
        2 => {
            play_sfx(commands, &audio_assets.bounce, config, 0.50);
            config.save();
        }
        3 => {
            play_sfx(commands, &audio_assets.bounce, config, 0.40);
            next_state.set(GameState::MainMenu);
        }
        _ => {}
    }
}

fn is_settings_compact(window_height: f32) -> bool {
    window_height <= SETTINGS_COMPACT_HEIGHT_THRESHOLD
}
