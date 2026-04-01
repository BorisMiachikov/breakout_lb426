use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crate::resources::config::GameConfig;

pub fn apply_window_settings(
    config: Res<GameConfig>,
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
) {
    if let Ok(mut window) = windows.single_mut() {
        window.resolution.set(config.window_width, config.window_height);
    }
}