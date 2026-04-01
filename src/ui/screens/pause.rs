use bevy::prelude::*;

use crate::app::states::GameState;
use crate::ui::screens::style::*;

#[derive(Component)]
pub struct PauseUI;

#[derive(Component)]
pub struct PauseItem {
    pub index: usize,
}

#[derive(Component)]
pub struct PauseItemLabel;

#[derive(Resource)]
pub struct PauseState {
    pub selected: usize,
}

const PAUSE_ITEMS: [&str; 2] = ["Resume", "Main Menu"];

pub fn setup_pause_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.insert_resource(PauseState { selected: 0 });

    let font = asset_server.load("fonts/FiraSans-Bold.ttf");

    commands
        .spawn((
            screen_root_node(),
            screen_overlay_color(),
            PauseUI,
        ))
        .with_children(|parent| {
            parent
                .spawn((screen_panel_node(), panel_color()))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("PAUSED"),
                        TextFont {
                            font: font.clone(),
                            font_size: TITLE_SIZE,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));

                    for (index, label) in PAUSE_ITEMS.iter().enumerate() {
                        parent
                            .spawn((
                                Button,
                                menu_button_node(),
                                if index == 0 { selected_color() } else { idle_color() },
                                PauseItem { index },
                            ))
                            .with_children(|parent| {
                                parent.spawn((
                                    Text::new(*label),
                                    TextFont {
                                        font: font.clone(),
                                        font_size: ITEM_SIZE,
                                        ..default()
                                    },
                                    TextColor(if index == 0 { accent_text() } else { Color::WHITE }),
                                    PauseItemLabel,
                                ));
                            });
                    }

                    parent.spawn((
                        Text::new("Mouse supported | Esc = Resume"),
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

pub fn cleanup_pause_ui(
    mut commands: Commands,
    query: Query<Entity, With<PauseUI>>,
) {
    for e in query.iter() {
        commands.entity(e).despawn();
    }
    commands.remove_resource::<PauseState>();
}

pub fn pause_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut pause_state: ResMut<PauseState>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        next_state.set(GameState::Playing);
    }

    if keys.just_pressed(KeyCode::ArrowUp) && pause_state.selected > 0 {
        pause_state.selected -= 1;
    }

    if keys.just_pressed(KeyCode::ArrowDown) && pause_state.selected < PAUSE_ITEMS.len() - 1 {
        pause_state.selected += 1;
    }

    if keys.just_pressed(KeyCode::Enter) || keys.just_pressed(KeyCode::NumpadEnter) {
        activate_pause_item(pause_state.selected, &mut next_state);
    }
}

pub fn pause_mouse_input(
    mut interaction_query: Query<(&Interaction, &PauseItem), (Changed<Interaction>, With<Button>)>,
    mut pause_state: ResMut<PauseState>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for (interaction, item) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Hovered => pause_state.selected = item.index,
            Interaction::Pressed => {
                pause_state.selected = item.index;
                activate_pause_item(item.index, &mut next_state);
            }
            Interaction::None => {}
        }
    }
}

pub fn update_pause_visuals(
    pause_state: Res<PauseState>,
    mut query: Query<(&PauseItem, &Children, &mut BackgroundColor)>,
    mut text_query: Query<&mut TextColor, With<PauseItemLabel>>,
) {
    if !pause_state.is_changed() {
        return;
    }

    for (item, children, mut background) in query.iter_mut() {
        let is_selected = item.index == pause_state.selected;
        background.0 = if is_selected { selected_color().0 } else { idle_color().0 };

        for child in children.iter() {
            if let Ok(mut color) = text_query.get_mut(child) {
                color.0 = if is_selected { accent_text() } else { Color::WHITE };
            }
        }
    }
}

fn activate_pause_item(selected: usize, next_state: &mut ResMut<NextState<GameState>>) {
    match selected {
        0 => next_state.set(GameState::Playing),
        1 => next_state.set(GameState::MainMenu),
        _ => {}
    }
}
