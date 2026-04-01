use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub const VIRTUAL_WIDTH: f32 = 860.0;
pub const VIRTUAL_HEIGHT: f32 = 540.0;
pub const SIDEBAR_WIDTH: f32 = 220.0;
pub const PLAYFIELD_WIDTH: f32 = VIRTUAL_WIDTH - SIDEBAR_WIDTH;
pub const PLAYFIELD_CENTER_X: f32 = -SIDEBAR_WIDTH / 2.0;

pub fn playfield_left() -> f32 {
    PLAYFIELD_CENTER_X - PLAYFIELD_WIDTH / 2.0
}

pub fn playfield_right() -> f32 {
    PLAYFIELD_CENTER_X + PLAYFIELD_WIDTH / 2.0
}

pub fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

pub fn camera_scaling(
    windows: Query<&Window, With<PrimaryWindow>>,
    mut camera_q: Query<&mut Projection, With<Camera>>,
) {
    let Ok(window) = windows.single() else { return };
    let Ok(mut projection) = camera_q.single_mut() else { return };

    if let Projection::Orthographic(ref mut ortho) = *projection {
        let scale_x = window.width() / VIRTUAL_WIDTH;
        let scale_y = window.height() / VIRTUAL_HEIGHT;

        ortho.scale = 1.0 / scale_x.min(scale_y);
    }
}
