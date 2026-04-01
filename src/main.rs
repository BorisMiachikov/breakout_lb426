use bevy::prelude::*;

mod app;
mod core;
mod gameplay;
mod ui;
mod utils; // ← ВАЖНО: добавить это

fn main() {
    App::new()
        .add_plugins(app::plugins::AppPlugins)
        .run();
}