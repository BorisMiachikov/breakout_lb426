use bevy::prelude::*;
use bevy::window::WindowResolution;

use crate::app::states::GameState;
use crate::core::camera::{camera_scaling, setup_camera};
use crate::core::config::GameConfig;
use crate::core::cursor::{hide_cursor, show_cursor};
use crate::gameplay::plugin::GameplayPlugin;
use crate::ui::plugin::UiPlugin;

pub struct AppPlugins;

impl Plugin for AppPlugins {
    fn build(&self, app: &mut App) {
        let config = GameConfig::load();

        app
            .add_plugins(DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: WindowResolution::new(
                        config.window_width as u32,
                        config.window_height as u32,
                    ),
                    title: "Breakout".into(),
                    resizable: true,
                    ..default()
                }),
                ..default()
            }))
            .insert_resource(config)
            .init_state::<GameState>()
            .add_systems(Startup, setup_camera)
            .add_systems(Update, camera_scaling)
            .add_systems(OnEnter(GameState::Playing), hide_cursor)
            .add_systems(OnEnter(GameState::MainMenu), show_cursor)
            .add_systems(OnEnter(GameState::HighScores), show_cursor)
            .add_systems(OnEnter(GameState::Settings), show_cursor)
            .add_systems(OnEnter(GameState::Paused), show_cursor)
            .add_systems(OnEnter(GameState::GameOver), show_cursor)
            .add_systems(OnEnter(GameState::LevelComplete), show_cursor)
            .add_systems(OnEnter(GameState::Victory), show_cursor)
            .add_plugins((
                GameplayPlugin,
                UiPlugin,
            ));
    }
}
