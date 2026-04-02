use bevy::prelude::*;

pub const MENU_BUTTON_WIDTH: f32 = 320.0;
pub const WIDE_PANEL_WIDTH: f32 = 560.0;
pub const MENU_BUTTON_PADDING_X: f32 = 18.0;
pub const MENU_BUTTON_PADDING_Y: f32 = 10.0;
pub const TITLE_SIZE: f32 = 64.0;
pub const ITEM_SIZE: f32 = 36.0;
pub const SUBTITLE_SIZE: f32 = 24.0;

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

pub fn menu_button_node() -> Node {
    Node {
        width: Val::Px(MENU_BUTTON_WIDTH),
        padding: UiRect::axes(
            Val::Px(MENU_BUTTON_PADDING_X),
            Val::Px(MENU_BUTTON_PADDING_Y),
        ),
        justify_content: JustifyContent::SpaceBetween,
        align_items: AlignItems::FlexStart,
        flex_direction: FlexDirection::Column,
        row_gap: Val::Px(4.0),
        ..default()
    }
}

pub fn header_block_node() -> Node {
    Node {
        width: Val::Percent(100.0),
        margin: UiRect::bottom(Val::Px(6.0)),
        flex_direction: FlexDirection::Column,
        row_gap: Val::Px(8.0),
        ..default()
    }
}

pub fn section_card_node(width: f32) -> Node {
    Node {
        width: Val::Px(width),
        padding: UiRect::all(Val::Px(14.0)),
        flex_direction: FlexDirection::Column,
        row_gap: Val::Px(6.0),
        ..default()
    }
}

pub fn adaptive_section_card_node(width: f32, compact: bool) -> Node {
    Node {
        width: Val::Px(width),
        padding: UiRect::all(Val::Px(if compact { 10.0 } else { 14.0 })),
        flex_direction: FlexDirection::Column,
        row_gap: Val::Px(if compact { 4.0 } else { 6.0 }),
        ..default()
    }
}

pub fn screen_overlay_color() -> BackgroundColor {
    BackgroundColor(Color::srgba(0.03, 0.05, 0.08, 0.72))
}

pub fn panel_color() -> BackgroundColor {
    BackgroundColor(Color::srgba(0.08, 0.11, 0.16, 0.95))
}

pub fn selected_color() -> BackgroundColor {
    BackgroundColor(Color::srgba(0.16, 0.27, 0.36, 0.96))
}

pub fn card_color() -> BackgroundColor {
    BackgroundColor(Color::srgba(0.12, 0.17, 0.24, 0.92))
}

pub fn accent_text() -> Color {
    Color::srgb(1.0, 0.84, 0.36)
}

pub fn secondary_accent_text() -> Color {
    Color::srgb(0.47, 0.83, 0.96)
}

pub fn muted_text() -> Color {
    Color::srgb(0.72, 0.78, 0.84)
}

pub fn disabled_text() -> Color {
    Color::srgb(0.44, 0.50, 0.57)
}

pub fn subtle_text() -> Color {
    Color::srgb(0.88, 0.91, 0.94)
}
