use bevy::prelude::*;
use crate::app::states::GameState;
use bevy::app::AppExit;


#[derive(Component)]
pub struct MainMenuUI;

#[derive(Component)]
pub struct MenuItem {
    pub index: usize,
}

#[derive(Resource)]
pub struct MenuState {
    pub selected: usize,
}

const MENU_ITEMS: [&str; 3] = ["Start Game", "Settings", "Quit"];

pub fn setup_main_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.insert_resource(MenuState { selected: 0 });

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
        MainMenuUI,
    ))
    .with_children(|parent| {
        // Заголовок
        parent.spawn((
            Text::new("BREAKOUT"),
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

        // Пункты меню
        for (i, item) in MENU_ITEMS.iter().enumerate() {
            parent.spawn((
                Text::new(*item),
                TextFont {
                    font: font.clone(),
                    font_size: 40.0,
                    ..default()
                },
                TextColor(if i == 0 { Color::srgb(1.0, 1.0, 0.0)  } else { Color::WHITE }),
                MenuItem { index: i },
                Node {
                    align_self: AlignSelf::Center,
                    ..default()
                },
            ));
        }
    });
}

pub fn cleanup_main_menu(
    mut commands: Commands,
    query: Query<Entity, With<MainMenuUI>>,
) {
    for e in query.iter() {
        commands.entity(e).despawn();
    }
}

pub fn main_menu_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut menu_state: ResMut<MenuState>,
    mut next_state: ResMut<NextState<GameState>>,
    mut exit: MessageWriter<AppExit>,
) {
    if keys.just_pressed(KeyCode::ArrowUp) {
        if menu_state.selected > 0 {
            menu_state.selected -= 1;
        }
        info!("Menu selected: {}", menu_state.selected);
    }

    if keys.just_pressed(KeyCode::ArrowDown) {
        if menu_state.selected < MENU_ITEMS.len() - 1 {
            menu_state.selected += 1;
        }
        info!("Menu selected: {}", menu_state.selected);
    }

    if keys.just_pressed(KeyCode::Enter) || keys.just_pressed(KeyCode::NumpadEnter) {
        info!("Menu Enter pressed, selected: {}", menu_state.selected);
        match menu_state.selected {
            0 => {
                info!("Starting game...");
                next_state.set(GameState::Playing);
            }
            1 => {
                info!("Opening settings...");
                next_state.set(GameState::Settings);
            }
            2 => { exit.write(AppExit::Success); }
            _ => {}
        }
    }
}

pub fn update_menu_visuals(
    menu_state: Res<MenuState>,
    mut query: Query<(&MenuItem, &mut TextColor)>,
) {
    for (item, mut color) in query.iter_mut() {
        if item.index == menu_state.selected {
            color.0 = Color::srgb(1.0, 1.0, 0.0);
        } else {
            color.0 = Color::WHITE;
        }
    }
}