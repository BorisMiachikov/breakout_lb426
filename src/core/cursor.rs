use bevy::prelude::*;
use bevy::window::CursorOptions;

pub fn hide_cursor(
    mut cursor_options: Single<&mut CursorOptions>,
) {
    cursor_options.visible = false;
}

pub fn show_cursor(
    mut cursor_options: Single<&mut CursorOptions>,
) {
    cursor_options.visible = true;
}
