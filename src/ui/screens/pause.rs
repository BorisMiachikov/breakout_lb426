use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::app::states::GameState;
use crate::ui::components::{
    action_card_detail_color, action_card_title_color, spawn_action_card,
    spawn_screen_background, spawn_screen_header, ActionCardDetail, ActionCardTitle,
};
use crate::ui::screens::style::*;

#[derive(Component)]
pub struct PauseUI;

#[derive(Component)]
pub struct PausePanel;

#[derive(Component)]
pub struct PauseItem {
    pub index: usize,
}

#[derive(Component)]
pub struct PauseControlsCard;

#[derive(Resource)]
pub struct PauseState {
    pub selected: usize,
}

#[derive(Resource)]
pub struct PauseLayout {
    pub compact: bool,
}

struct PauseEntry {
    title: &'static str,
    detail: &'static str,
}

const PAUSE_ITEMS: [PauseEntry; 2] = [
    PauseEntry {
        title: "Resume",
        detail: "Return to the current rally immediately.",
    },
    PauseEntry {
        title: "Main Menu",
        detail: "End the current run and go back to the title screen.",
    },
];

const PAUSE_COMPACT_HEIGHT_THRESHOLD: f32 = 760.0;

fn pause_panel_color() -> BackgroundColor {
    BackgroundColor(Color::srgba(0.05, 0.08, 0.12, 0.78))
}

fn pause_card_color() -> BackgroundColor {
    BackgroundColor(Color::srgba(0.10, 0.15, 0.21, 0.74))
}

fn pause_panel_node(compact: bool) -> Node {
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

fn pause_button_node(compact: bool) -> Node {
    Node {
        width: Val::Px(MENU_BUTTON_WIDTH),
        padding: UiRect::axes(Val::Px(18.0), Val::Px(if compact { 6.0 } else { 10.0 })),
        justify_content: JustifyContent::SpaceBetween,
        align_items: AlignItems::FlexStart,
        flex_direction: FlexDirection::Column,
        row_gap: Val::Px(if compact { 1.0 } else { 4.0 }),
        ..default()
    }
}

pub fn setup_pause_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let compact = window_query
        .single()
        .map(|window| is_pause_compact(window.height()))
        .unwrap_or(false);
    commands.insert_resource(PauseState { selected: 0 });
    commands.insert_resource(PauseLayout { compact });

    let font = asset_server.load("fonts/FiraSans-Bold.ttf");

    commands
        .spawn((
            screen_root_node(),
            screen_overlay_color(),
            PauseUI,
        ))
        .with_children(|parent| {
            spawn_screen_background(parent, &asset_server, "backgrounds/game_sat.png");

            parent
                .spawn((pause_panel_node(compact), pause_panel_color(), PausePanel))
                .with_children(|parent| {
                    spawn_screen_header(
                        parent,
                        &font,
                        "MATCH PAUSED",
                        "PAUSED",
                        "Take a breath, then choose whether to jump back in or reset the flow from the menu.",
                    );

                    for (index, item) in PAUSE_ITEMS.iter().enumerate() {
                        parent
                            .spawn((
                                Button,
                                pause_button_node(compact),
                                if index == 0 {
                                    selected_color()
                                } else {
                                    pause_card_color()
                                },
                                PauseItem { index },
                            ))
                            .with_children(|parent| {
                                spawn_action_card(
                                    parent,
                                    &font,
                                    item.title,
                                    item.detail,
                                    index == 0,
                                    true,
                                );
                            });
                    }

                    parent
                        .spawn((section_card_node(MENU_BUTTON_WIDTH), card_color(), PauseControlsCard))
                        .insert(pause_card_color())
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
                                Text::new("Esc resumes instantly. Arrow keys move selection. Enter confirms. Mouse hover and click also work."),
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

pub fn cleanup_pause_ui(
    mut commands: Commands,
    query: Query<Entity, With<PauseUI>>,
) {
    for e in query.iter() {
        commands.entity(e).despawn();
    }
    commands.remove_resource::<PauseState>();
    commands.remove_resource::<PauseLayout>();
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
    mut text_query: Query<
        &mut TextColor,
        (With<ActionCardTitle>, Without<ActionCardDetail>),
    >,
    mut detail_query: Query<
        &mut TextColor,
        (With<ActionCardDetail>, Without<ActionCardTitle>),
    >,
) {
    if !pause_state.is_changed() {
        return;
    }

    for (item, children, mut background) in query.iter_mut() {
        let is_selected = item.index == pause_state.selected;
        background.0 = if is_selected {
            selected_color().0
        } else {
            pause_card_color().0
        };

        for child in children.iter() {
            if let Ok(mut color) = text_query.get_mut(child) {
                color.0 = action_card_title_color(is_selected, true);
            }

            if let Ok(mut color) = detail_query.get_mut(child) {
                color.0 = action_card_detail_color(is_selected, true);
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

pub fn adapt_pause_layout(
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut layout: ResMut<PauseLayout>,
    mut panel_query: Query<&mut Node, (With<PausePanel>, Without<PauseItem>)>,
    mut button_query: Query<&mut Node, (With<PauseItem>, Without<PausePanel>)>,
    mut controls_query: Query<&mut Visibility, With<PauseControlsCard>>,
) {
    let Ok(window) = window_query.single() else {
        return;
    };

    let compact = is_pause_compact(window.height());
    if layout.compact == compact {
        return;
    }

    layout.compact = compact;

    for mut node in panel_query.iter_mut() {
        *node = pause_panel_node(compact);
    }

    for mut node in button_query.iter_mut() {
        *node = pause_button_node(compact);
    }

    for mut visibility in controls_query.iter_mut() {
        *visibility = if compact {
            Visibility::Hidden
        } else {
            Visibility::Inherited
        };
    }
}

fn is_pause_compact(window_height: f32) -> bool {
    window_height <= PAUSE_COMPACT_HEIGHT_THRESHOLD
}
