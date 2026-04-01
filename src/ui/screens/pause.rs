use bevy::prelude::*;
use crate::app::states::GameState;

#[derive(Component)]
pub struct PauseUI;

pub fn setup_pause_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");

    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            row_gap: Val::Px(10.0),
            ..default()
        },
        PauseUI,
    ))
    .with_children(|parent| {
        // Заголовок
        parent.spawn((
            Text::new("PAUSED"),
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

        // Подсказка
        parent.spawn((
            Text::new("ESC - Resume\nEnter - Quit to Menu"),
            TextFont {
                font,
                font_size: 32.0,
                ..default()
            },
            TextColor(Color::srgb(0.8, 0.8, 0.8)),
            Node {
                align_self: AlignSelf::Center,
                margin: UiRect::top(Val::Px(20.0)),
                ..default()
            },
        ));
    });
}

pub fn cleanup_pause_ui(
    mut commands: Commands,
    query: Query<Entity, With<PauseUI>>,
) {
    for e in query.iter() {
        commands.entity(e).despawn();
    }
}

pub fn pause_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        next_state.set(GameState::Playing);
    }

    if keys.just_pressed(KeyCode::Enter) || keys.just_pressed(KeyCode::NumpadEnter) {
        next_state.set(GameState::MainMenu);
    }
}
