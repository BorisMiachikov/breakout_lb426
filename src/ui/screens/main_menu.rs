use bevy::app::AppExit;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::app::states::GameState;
use crate::core::audio::{play_sfx, AudioAssets};
use crate::core::config::GameConfig;
use crate::ui::components::{
    action_card_detail_color, action_card_title_color, spawn_action_card,
    spawn_screen_background, spawn_screen_header, ActionCardDetail, ActionCardTitle,
};
use crate::ui::screens::style::*;

#[derive(Component)]
pub struct MainMenuUI;

#[derive(Component)]
pub struct MainMenuPanel;

#[derive(Component)]
pub struct MainMenuControlsCard;

#[derive(Component)]
pub struct MenuItem {
    pub index: usize,
    pub enabled: bool,
}

#[derive(Component)]
pub struct MenuItemMarker;

#[derive(Resource)]
pub struct MenuState {
    pub selected: usize,
}

#[derive(Resource)]
pub struct MainMenuLayout {
    pub compact: bool,
}

struct MenuEntry {
    title: &'static str,
    detail: &'static str,
    enabled: bool,
}

const MENU_ITEMS: [MenuEntry; 5] = [
    MenuEntry {
        title: "Start Game",
        detail: "Play the full five-level campaign.",
        enabled: true,
    },
    MenuEntry {
        title: "High Scores",
        detail: "Review your best campaign runs.",
        enabled: true,
    },
    MenuEntry {
        title: "Settings",
        detail: "Adjust audio and window preferences.",
        enabled: true,
    },
    MenuEntry {
        title: "Level Editor",
        detail: "Planned for Phase 6. Visible now so the menu flow is stable.",
        enabled: false,
    },
    MenuEntry {
        title: "Quit",
        detail: "Exit the game.",
        enabled: true,
    },
];

const MAIN_MENU_PANEL_WIDTH: f32 = 540.0;
const MAIN_MENU_COMPACT_HEIGHT_THRESHOLD: f32 = 720.0;

fn main_menu_panel_color() -> BackgroundColor {
    BackgroundColor(Color::srgba(0.02, 0.04, 0.07, 0.42))
}

fn main_menu_selected_color() -> BackgroundColor {
    BackgroundColor(Color::srgba(0.10, 0.19, 0.31, 0.42))
}

fn main_menu_idle_color() -> BackgroundColor {
    BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.0))
}

fn main_menu_disabled_color() -> BackgroundColor {
    BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.0))
}

fn main_menu_info_card_color() -> BackgroundColor {
    BackgroundColor(Color::srgba(0.04, 0.08, 0.12, 0.24))
}

fn main_menu_panel_node(compact: bool) -> Node {
    Node {
        width: Val::Px(MAIN_MENU_PANEL_WIDTH),
        padding: UiRect::all(Val::Px(if compact { 12.0 } else { 18.0 })),
        flex_direction: FlexDirection::Column,
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Stretch,
        row_gap: Val::Px(if compact { 8.0 } else { 10.0 }),
        ..default()
    }
}

fn compact_menu_button_node(compact: bool) -> Node {
    Node {
        width: Val::Px(MENU_BUTTON_WIDTH),
        padding: UiRect::axes(Val::Px(16.0), Val::Px(if compact { 5.0 } else { 8.0 })),
        border: UiRect::all(Val::Px(1.0)),
        justify_content: JustifyContent::SpaceBetween,
        align_items: AlignItems::FlexStart,
        flex_direction: FlexDirection::Column,
        row_gap: Val::Px(if compact { 1.0 } else { 2.0 }),
        ..default()
    }
}

fn main_menu_selected_border() -> BorderColor {
    BorderColor::all(Color::srgba(0.58, 0.88, 1.0, 0.90))
}

fn main_menu_idle_border() -> BorderColor {
    BorderColor::all(Color::srgba(0.0, 0.0, 0.0, 0.0))
}

fn main_menu_disabled_border() -> BorderColor {
    BorderColor::all(Color::srgba(0.0, 0.0, 0.0, 0.0))
}

pub fn setup_main_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let compact = window_query
        .single()
        .map(|window| is_compact_layout(window.height()))
        .unwrap_or(false);

    commands.insert_resource(MenuState {
        selected: first_enabled_index(),
    });
    commands.insert_resource(MainMenuLayout { compact });

    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    commands
        .spawn((
            screen_root_node(),
            screen_overlay_color(),
            MainMenuUI,
        ))
        .with_children(|parent| {
            spawn_screen_background(parent, &asset_server, "backgrounds/menu.png");

            parent
                .spawn((main_menu_panel_node(compact), main_menu_panel_color(), MainMenuPanel))
                .with_children(|parent| {
                    spawn_screen_header(
                        parent,
                        &font,
                        "ARCADE PROJECT",
                        "BREAKOUT",
                        "A small Bevy campaign with persistent runs, sharper UI, and room for an in-game editor.",
                    );

                    for (i, item) in MENU_ITEMS.iter().enumerate() {
                        let is_selected = i == first_enabled_index();
                        parent
                            .spawn((
                                Button,
                                compact_menu_button_node(compact),
                                if item.enabled {
                                    if is_selected {
                                        main_menu_selected_color()
                                    } else {
                                        main_menu_idle_color()
                                    }
                                } else {
                                    main_menu_disabled_color()
                                },
                                if item.enabled {
                                    if is_selected {
                                        main_menu_selected_border()
                                    } else {
                                        main_menu_idle_border()
                                    }
                                } else {
                                    main_menu_disabled_border()
                                },
                                MenuItem {
                                    index: i,
                                    enabled: item.enabled,
                                },
                            ))
                            .with_children(|parent| {
                                parent.spawn((
                                    Text::new(">"),
                                    TextFont {
                                        font: font.clone(),
                                        font_size: ITEM_SIZE - 8.0,
                                        ..default()
                                    },
                                    TextColor(if is_selected {
                                        secondary_accent_text()
                                    } else {
                                        Color::srgba(0.0, 0.0, 0.0, 0.0)
                                    }),
                                    MenuItemMarker,
                                ));

                                spawn_action_card(
                                    parent,
                                    &font,
                                    item.title,
                                    item.detail,
                                    is_selected,
                                    item.enabled,
                                );
                            });
                    }

                    if !compact {
                        parent
                            .spawn((
                                section_card_node(MENU_BUTTON_WIDTH),
                                main_menu_info_card_color(),
                                MainMenuControlsCard,
                            ))
                            .with_children(|parent| {
                                parent.spawn((
                                    Text::new("Controls"),
                                    TextFont {
                                        font: font.clone(),
                                        font_size: SUBTITLE_SIZE - 2.0,
                                        ..default()
                                    },
                                    TextColor(subtle_text()),
                                ));

                                parent.spawn((
                                    Text::new("Arrow keys navigate. Enter confirms. Mouse hover and click are supported across menus."),
                                    TextFont {
                                        font,
                                        font_size: SUBTITLE_SIZE - 6.0,
                                        ..default()
                                    },
                                    TextColor(muted_text()),
                                ));
                            });
                    }
                });
        });
}

pub fn cleanup_main_menu(
    mut commands: Commands,
    query: Query<Entity, With<MainMenuUI>>,
) {
    for e in query.iter() {
        commands.entity(e).despawn();
    }
    commands.remove_resource::<MenuState>();
    commands.remove_resource::<MainMenuLayout>();
}

pub fn main_menu_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    audio_assets: Res<AudioAssets>,
    config: Res<GameConfig>,
    mut menu_state: ResMut<MenuState>,
    mut next_state: ResMut<NextState<GameState>>,
    mut exit: MessageWriter<AppExit>,
) {
    if keys.just_pressed(KeyCode::ArrowUp) {
        menu_state.selected = previous_enabled_index(menu_state.selected);
    }

    if keys.just_pressed(KeyCode::ArrowDown) {
        menu_state.selected = next_enabled_index(menu_state.selected);
    }

    if keys.just_pressed(KeyCode::Enter) || keys.just_pressed(KeyCode::NumpadEnter) {
        play_sfx(&mut commands, &audio_assets.bounce, &config, 0.45);
        activate_menu_item(menu_state.selected, &mut next_state, &mut exit);
    }
}

pub fn main_menu_mouse_input(
    mut commands: Commands,
    audio_assets: Res<AudioAssets>,
    config: Res<GameConfig>,
    mut interaction_query: Query<
        (&Interaction, &MenuItem),
        (Changed<Interaction>, With<Button>),
    >,
    mut menu_state: ResMut<MenuState>,
    mut next_state: ResMut<NextState<GameState>>,
    mut exit: MessageWriter<AppExit>,
) {
    for (interaction, item) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Hovered => {
                if item.enabled {
                    menu_state.selected = item.index;
                }
            }
            Interaction::Pressed => {
                if item.enabled {
                    menu_state.selected = item.index;
                    play_sfx(&mut commands, &audio_assets.bounce, &config, 0.45);
                    activate_menu_item(item.index, &mut next_state, &mut exit);
                }
            }
            Interaction::None => {}
        }
    }
}

pub fn update_menu_visuals(
    menu_state: Res<MenuState>,
    mut query: Query<(&MenuItem, &Children, &mut BackgroundColor, &mut BorderColor)>,
    mut text_query: Query<
        &mut TextColor,
        (With<ActionCardTitle>, Without<ActionCardDetail>),
    >,
    mut detail_query: Query<
        &mut TextColor,
        (With<ActionCardDetail>, Without<ActionCardTitle>),
    >,
    mut marker_query: Query<
        &mut TextColor,
        (
            With<MenuItemMarker>,
            Without<ActionCardTitle>,
            Without<ActionCardDetail>,
        ),
    >,
) {
    for (item, children, mut background, mut border_color) in query.iter_mut() {
        let is_selected = item.enabled && item.index == menu_state.selected;

        background.0 = if item.enabled {
            if is_selected {
                main_menu_selected_color().0
            } else {
                main_menu_idle_color().0
            }
        } else {
            main_menu_disabled_color().0
        };

        *border_color = if item.enabled {
            if is_selected {
                main_menu_selected_border()
            } else {
                main_menu_idle_border()
            }
        } else {
            main_menu_disabled_border()
        };

        for child in children.iter() {
            if let Ok(mut color) = text_query.get_mut(child) {
                color.0 = action_card_title_color(is_selected, item.enabled);
            }

            if let Ok(mut color) = detail_query.get_mut(child) {
                color.0 = action_card_detail_color(is_selected, item.enabled);
            }

            if let Ok(mut color) = marker_query.get_mut(child) {
                color.0 = if is_selected && item.enabled {
                    secondary_accent_text()
                } else {
                    Color::srgba(0.0, 0.0, 0.0, 0.0)
                };
            }
        }
    }
}

pub fn adapt_main_menu_layout(
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut layout: ResMut<MainMenuLayout>,
    mut panel_query: Query<&mut Node, (With<MainMenuPanel>, Without<MenuItem>)>,
    mut button_query: Query<&mut Node, (With<MenuItem>, Without<MainMenuPanel>)>,
    mut controls_query: Query<&mut Visibility, With<MainMenuControlsCard>>,
) {
    let Ok(window) = window_query.single() else {
        return;
    };

    let compact = is_compact_layout(window.height());
    if layout.compact == compact {
        return;
    }

    layout.compact = compact;

    for mut node in panel_query.iter_mut() {
        *node = main_menu_panel_node(compact);
    }

    for mut node in button_query.iter_mut() {
        *node = compact_menu_button_node(compact);
    }

    for mut visibility in controls_query.iter_mut() {
        *visibility = if compact {
            Visibility::Hidden
        } else {
            Visibility::Inherited
        };
    }
}

fn activate_menu_item(
    selected: usize,
    next_state: &mut ResMut<NextState<GameState>>,
    exit: &mut MessageWriter<AppExit>,
) {
    match selected {
        0 => next_state.set(GameState::Playing),
        1 => next_state.set(GameState::HighScores),
        2 => next_state.set(GameState::Settings),
        4 => {
            exit.write(AppExit::Success);
        }
        _ => {}
    }
}

fn first_enabled_index() -> usize {
    MENU_ITEMS.iter().position(|item| item.enabled).unwrap_or(0)
}

fn previous_enabled_index(current: usize) -> usize {
    for index in (0..current).rev() {
        if MENU_ITEMS[index].enabled {
            return index;
        }
    }

    current
}

fn next_enabled_index(current: usize) -> usize {
    for index in current + 1..MENU_ITEMS.len() {
        if MENU_ITEMS[index].enabled {
            return index;
        }
    }

    current
}

fn is_compact_layout(window_height: f32) -> bool {
    window_height <= MAIN_MENU_COMPACT_HEIGHT_THRESHOLD
}
