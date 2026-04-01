use bevy::prelude::*;

pub const MENU_BUTTON_WIDTH: f32 = 320.0;
pub const MENU_BUTTON_PADDING_X: f32 = 18.0;
pub const MENU_BUTTON_PADDING_Y: f32 = 10.0;
pub const TITLE_SIZE: f32 = 64.0;
pub const ITEM_SIZE: f32 = 36.0;
pub const SUBTITLE_SIZE: f32 = 24.0;
pub const PANEL_WIDTH: f32 = 420.0;

pub fn screen_root_node() -> Node {
    Node {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        padding: UiRect::all(Val::Px(20.0)),
        ..default()
    }
}

pub fn screen_panel_node() -> Node {
    Node {
        width: Val::Px(PANEL_WIDTH),
        padding: UiRect::all(Val::Px(22.0)),
        flex_direction: FlexDirection::Column,
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        row_gap: Val::Px(12.0),
        ..default()
    }
}

pub fn menu_button_node() -> Node {
    Node {
        width: Val::Px(MENU_BUTTON_WIDTH),
        padding: UiRect::axes(
            Val::Px(MENU_BUTTON_PADDING_X),
            Val::Px(MENU_BUTTON_PADDING_Y),
        ),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    }
}

pub fn screen_overlay_color() -> BackgroundColor {
    BackgroundColor(Color::srgba(0.04, 0.04, 0.06, 0.72))
}

pub fn panel_color() -> BackgroundColor {
    BackgroundColor(Color::srgba(0.08, 0.09, 0.11, 0.94))
}

pub fn selected_color() -> BackgroundColor {
    BackgroundColor(Color::srgba(0.22, 0.22, 0.26, 0.95))
}

pub fn idle_color() -> BackgroundColor {
    BackgroundColor(Color::srgba(0.12, 0.12, 0.15, 0.88))
}

pub fn accent_text() -> Color {
    Color::srgb(1.0, 1.0, 0.0)
}

pub fn muted_text() -> Color {
    Color::srgb(0.75, 0.77, 0.82)
}
