use bevy::app::AppExit;
use bevy::prelude::*;

use crate::app::states::GameState;
use crate::ui::screens::style::*;

#[derive(Component)]
pub struct MainMenuUI;

#[derive(Component)]
pub struct MenuItem {
    pub index: usize,
}

#[derive(Component)]
pub struct MenuItemLabel;

#[derive(Resource)]
pub struct MenuState {
    pub selected: usize,
}

const MENU_ITEMS: [&str; 4] = ["Start Game", "High Scores", "Settings", "Quit"];

pub fn setup_main_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.insert_resource(MenuState { selected: 0 });

    let font = asset_server.load("fonts/FiraSans-Bold.ttf");

    commands
        .spawn((
            screen_root_node(),
            screen_overlay_color(),
            MainMenuUI,
        ))
        .with_children(|parent| {
            parent
                .spawn((screen_panel_node(), panel_color()))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("BREAKOUT"),
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

                    for (i, item) in MENU_ITEMS.iter().enumerate() {
                        parent
                            .spawn((
                                Button,
                                menu_button_node(),
                                if i == 0 { selected_color() } else { idle_color() },
                                MenuItem { index: i },
                            ))
                            .with_children(|parent| {
                                parent.spawn((
                                    Text::new(*item),
                                    TextFont {
                                        font: font.clone(),
                                        font_size: ITEM_SIZE,
                                        ..default()
                                    },
                                    TextColor(if i == 0 { accent_text() } else { Color::WHITE }),
                                    MenuItemLabel,
                                ));
                            });
                    }

                    parent.spawn((
                        Text::new("Keyboard and mouse supported"),
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

pub fn cleanup_main_menu(
    mut commands: Commands,
    query: Query<Entity, With<MainMenuUI>>,
) {
    for e in query.iter() {
        commands.entity(e).despawn();
    }
    commands.remove_resource::<MenuState>();
}

pub fn main_menu_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut menu_state: ResMut<MenuState>,
    mut next_state: ResMut<NextState<GameState>>,
    mut exit: MessageWriter<AppExit>,
) {
    if keys.just_pressed(KeyCode::ArrowUp) && menu_state.selected > 0 {
        menu_state.selected -= 1;
    }

    if keys.just_pressed(KeyCode::ArrowDown) && menu_state.selected < MENU_ITEMS.len() - 1 {
        menu_state.selected += 1;
    }

    if keys.just_pressed(KeyCode::Enter) || keys.just_pressed(KeyCode::NumpadEnter) {
        activate_menu_item(menu_state.selected, &mut next_state, &mut exit);
    }
}

pub fn main_menu_mouse_input(
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
                menu_state.selected = item.index;
            }
            Interaction::Pressed => {
                menu_state.selected = item.index;
                activate_menu_item(item.index, &mut next_state, &mut exit);
            }
            Interaction::None => {}
        }
    }
}

pub fn update_menu_visuals(
    menu_state: Res<MenuState>,
    mut query: Query<(&MenuItem, &Children, &mut BackgroundColor)>,
    mut text_query: Query<&mut TextColor, With<MenuItemLabel>>,
) {
    for (item, children, mut background) in query.iter_mut() {
        let is_selected = item.index == menu_state.selected;

        background.0 = if is_selected { selected_color().0 } else { idle_color().0 };

        for child in children.iter() {
            if let Ok(mut color) = text_query.get_mut(child) {
                color.0 = if is_selected { accent_text() } else { Color::WHITE };
            }
        }
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
        3 => {
            exit.write(AppExit::Success);
        }
        _ => {}
    }
}
