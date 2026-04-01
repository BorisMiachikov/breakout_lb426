# Listing — Breakout LB426

## Cargo.toml

```toml
[package]
name = "breakout_lb426"
version = "0.1.0"
edition = "2021"

[dependencies]
bevy = "0.18.1"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

---

## src/main.rs

```rust
use bevy::prelude::*;
mod app;
mod core;
mod gameplay;
mod ui;

fn main() {
    App::new()
        .add_plugins(app::plugins::AppPlugins)
        .run();
}
```

---

## src/app/mod.rs

```rust
pub mod plugins;
pub mod states;
```

---

## src/app/plugins.rs

```rust
use bevy::prelude::*;
use bevy::window::WindowResolution;

use crate::app::states::GameState;
use crate::core::config::ConfigPlugin;
use crate::core::camera::setup_camera;
use crate::gameplay::plugin::GameplayPlugin;
use crate::ui::plugin::UiPlugin;

pub struct AppPlugins;

impl Plugin for AppPlugins {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: WindowResolution::new(800, 600),
                    title: "Breakout".into(),
                    resizable: true,
                    ..default()
                }),
                ..default()
            }))
            .init_state::<GameState>()
            .add_systems(Startup, setup_camera)
            .add_systems(Update, crate::core::camera::camera_scaling)
            .add_plugins((
                ConfigPlugin,
                GameplayPlugin,
                UiPlugin,
            ));
    }
}
```

---

## src/app/states.rs

```rust
use bevy::prelude::*;

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum GameState {
    #[default]
    MainMenu,
    Playing,
    Paused,
    GameOver,
    Settings,
}
```

---

## src/core/mod.rs

```rust
pub mod config;
pub mod camera;
```

---

## src/core/config.rs

```rust
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs;

const CONFIG_PATH: &str = "config.json";

#[derive(Resource, Serialize, Deserialize, Clone)]
pub struct GameConfig {
    pub window_width: f32,
    pub window_height: f32,
    pub music_volume: f32,
    pub sfx_volume: f32,
}

impl Default for GameConfig {
    fn default() -> Self {
        Self {
            window_width: 960.0,
            window_height: 540.0,
            music_volume: 0.5,
            sfx_volume: 0.7,
        }
    }
}

impl GameConfig {
    pub fn load() -> Self {
        if let Ok(data) = fs::read_to_string(CONFIG_PATH) {
            if let Ok(config) = serde_json::from_str(&data) {
                return config;
            }
        }
        let default = Self::default();
        default.save();
        default
    }

    pub fn save(&self) {
        if let Ok(json) = serde_json::to_string_pretty(self) {
            let _ = fs::write(CONFIG_PATH, json);
        }
    }
}

pub struct ConfigPlugin;

impl Plugin for ConfigPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameConfig::load());
    }
}
```

---

## src/core/camera.rs

```rust
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

const VIRTUAL_WIDTH: f32 = 800.0;
const VIRTUAL_HEIGHT: f32 = 600.0;

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
```

---

## src/gameplay/mod.rs

```rust
pub mod plugin;
pub mod components;
pub mod systems;
pub mod resources;
pub mod spawn;
```

---

## src/gameplay/plugin.rs

```rust
use bevy::prelude::*;

use crate::app::states::GameState;
use crate::gameplay::systems::*;
use crate::gameplay::spawn::*;
use crate::gameplay::resources::*;

pub struct GameplayPlugin;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(Lives(3))
            .insert_resource(Score(0))

            .add_systems(OnEnter(GameState::MainMenu), cleanup_game)
            .add_systems(OnEnter(GameState::Playing), spawn_game)

            .add_systems(
                Update,
                (
                    paddle_input,
                    paddle_movement,
                    ball_movement,
                    (
                        ball_wall_collision,
                        ball_paddle_collision,
                        ball_brick_collision,
                        ball_death,
                    ),
                )
                    .chain()
                    .run_if(in_state(GameState::Playing)),
            )
            .add_systems(
                Update,
                game_pause.run_if(in_state(GameState::Playing)),
            );
    }
}
```

---

## src/gameplay/components/mod.rs

```rust
pub mod ball;
pub mod brick;
pub mod collider;
pub mod paddle;
pub mod velocity;
```

---

## src/gameplay/components/paddle.rs

```rust
use bevy::prelude::*;

#[derive(Component)]
pub struct Paddle {
    pub speed: f32,
    pub direction: f32,
}
```

---

## src/gameplay/components/ball.rs

```rust
use bevy::prelude::*;

#[derive(Component)]
pub struct Ball {
    pub velocity: Vec2,
}
```

---

## src/gameplay/components/collider.rs

```rust
use bevy::prelude::*;

#[derive(Component)]
pub struct Collider { pub size: Vec2 }
```

---

## src/gameplay/components/brick.rs

```rust
use bevy::prelude::*;

#[derive(Component, Clone, Copy)]
pub enum BrickType {
    Normal,
    Strong,
}

#[derive(Component)]
pub struct Brick {
    pub brick_type: BrickType,
    pub health: u8,
    pub score: u32,
}
```

---

## src/gameplay/components/velocity.rs

```rust
use bevy::prelude::*;

#[derive(Component)]
pub struct Velocity(pub Vec2);
```

---

## src/gameplay/resources/mod.rs

```rust
pub mod lives;
pub mod score;

pub use lives::Lives;
pub use score::Score;
```

---

## src/gameplay/resources/lives.rs

```rust
use bevy::prelude::*;

#[derive(Resource)]
pub struct Lives(pub u32);
```

---

## src/gameplay/resources/score.rs

```rust
use bevy::prelude::*;

#[derive(Resource)]
pub struct Score(pub u32);
```

---

## src/gameplay/systems/mod.rs

```rust
pub mod collision;
pub mod movement;
pub mod inputs;

pub use collision::*;
pub use movement::*;
pub use inputs::*;
```

---

## src/gameplay/systems/inputs.rs

```rust
use bevy::prelude::*;
use bevy::input::keyboard::KeyCode;
use crate::gameplay::components::paddle::Paddle;
use crate::app::states::GameState;

pub fn paddle_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Paddle>,
) {
    for mut paddle in query.iter_mut() {
        let mut direction = 0.0;
        if keyboard.pressed(KeyCode::KeyA) || keyboard.pressed(KeyCode::ArrowLeft) {
            direction -= 1.0;
        }
        if keyboard.pressed(KeyCode::KeyD) || keyboard.pressed(KeyCode::ArrowRight) {
            direction += 1.0;
        }
        paddle.direction = direction;
    }
}

pub fn game_pause(
    keys: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        next_state.set(GameState::Paused);
    }
}
```

---

## src/gameplay/systems/movement.rs

```rust
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crate::gameplay::components::paddle::Paddle;
use crate::gameplay::components::ball::Ball;

pub fn ball_movement(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &Ball)>,
) {
    for (mut transform, ball) in query.iter_mut() {
        transform.translation.x += ball.velocity.x * time.delta_secs();
        transform.translation.y += ball.velocity.y * time.delta_secs();
    }
}

pub fn paddle_movement(
    time: Res<Time>,
    windows: Query<&Window, With<PrimaryWindow>>,
    mut query: Query<(&Paddle, &mut Transform, &crate::gameplay::components::collider::Collider), Without<Ball>>,
) {
    let _window = windows.single().expect("Primary window not found");

    let screen_half_width = 400.0;

    for (paddle, mut transform, collider) in query.iter_mut() {
        transform.translation.x += paddle.direction * paddle.speed * time.delta_secs();
        let half_width = collider.size.x / 2.0;
        let min_x = -screen_half_width + half_width;
        let max_x = screen_half_width - half_width;
        transform.translation.x = transform.translation.x.clamp(min_x, max_x);
    }
}
```

---

## src/gameplay/systems/collision/mod.rs

```rust
pub mod wall;
pub mod paddle;
pub mod bricks;
pub mod death;

pub use wall::*;
pub use paddle::*;
pub use bricks::*;
pub use death::*;
```

---

## src/gameplay/systems/collision/wall.rs

```rust
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crate::gameplay::components::{ball::Ball, collider::Collider};

pub fn ball_wall_collision(
    windows: Query<&Window, With<PrimaryWindow>>,
    mut query: Query<(&mut Ball, &Transform, &Collider)>,
) {
    let Ok(window) = windows.single() else { return };
    let half_width = window.width() / 2.0;
    let half_height = window.height() / 2.0;

    for (mut ball, transform, collider) in query.iter_mut() {
        let half_w = collider.size.x / 2.0;
        let half_h = collider.size.y / 2.0;

        if transform.translation.x - half_w <= -half_width
            || transform.translation.x + half_w >= half_width
        {
            ball.velocity.x *= -1.0;
        }
        if transform.translation.y + half_h >= half_height {
            ball.velocity.y *= -1.0;
        }
    }
}
```

---

## src/gameplay/systems/collision/paddle.rs

```rust
use bevy::prelude::*;
use crate::gameplay::components::{ball::Ball, collider::Collider, paddle::Paddle};

pub fn ball_paddle_collision(
    mut ball_q: Query<(&mut Ball, &Transform, &Collider), Without<Paddle>>,
    paddle_q: Query<(&Transform, &Collider), With<Paddle>>,
) {
    let Ok((paddle_tf, paddle_col)) = paddle_q.single() else { return };

    for (mut ball, ball_tf, ball_col) in ball_q.iter_mut() {
        if collide(ball_tf.translation, ball_col.size, paddle_tf.translation, paddle_col.size) {
            ball.velocity.y = ball.velocity.y.abs();
            let offset =
                (ball_tf.translation.x - paddle_tf.translation.x) / (paddle_col.size.x / 2.0);
            ball.velocity.x += offset * 300.0;
        }
    }
}

fn collide(a_pos: Vec3, a_size: Vec2, b_pos: Vec3, b_size: Vec2) -> bool {
    (a_pos.x - b_pos.x).abs() < (a_size.x + b_size.x) / 2.0
        && (a_pos.y - b_pos.y).abs() < (a_size.y + b_size.y) / 2.0
}
```

---

## src/gameplay/systems/collision/bricks.rs

```rust
use bevy::prelude::*;
use crate::gameplay::components::{ball::Ball, collider::Collider, brick::Brick, paddle::Paddle};
use crate::gameplay::resources::score::Score;

fn collide(a_pos: Vec3, a_size: Vec2, b_pos: Vec3, b_size: Vec2) -> bool {
    (a_pos.x - b_pos.x).abs() < (a_size.x + b_size.x) / 2.0
        && (a_pos.y - b_pos.y).abs() < (a_size.y + b_size.y) / 2.0
}

pub fn ball_brick_collision(
    mut commands: Commands,
    mut score: ResMut<Score>,
    mut ball_q: Query<(&mut Ball, &mut Transform, &Collider), Without<Paddle>>,
    mut brick_q: Query<(Entity, &mut Brick, &Transform, &Collider), Without<Ball>>,
) {
    let Ok((mut ball, mut ball_tf, ball_col)) = ball_q.single_mut() else { return };

    for (entity, mut brick, brick_tf, brick_col) in brick_q.iter_mut() {
        if collide(ball_tf.translation, ball_col.size, brick_tf.translation, brick_col.size) {
            let dx = ball_tf.translation.x - brick_tf.translation.x;
            let dy = ball_tf.translation.y - brick_tf.translation.y;
            let overlap_x = (ball_col.size.x + brick_col.size.x) / 2.0 - dx.abs();
            let overlap_y = (ball_col.size.y + brick_col.size.y) / 2.0 - dy.abs();

            if overlap_x < overlap_y {
                ball.velocity.x *= -1.0;
                if dx > 0.0 { ball_tf.translation.x += overlap_x; }
                else { ball_tf.translation.x -= overlap_x; }
            } else {
                ball.velocity.y *= -1.0;
                if dy > 0.0 { ball_tf.translation.y += overlap_y; }
                else { ball_tf.translation.y -= overlap_y; }
            }

            brick.health -= 1;
            if brick.health == 0 {
                score.0 += brick.score;
                commands.entity(entity).despawn();
            }
            break;
        }
    }
}
```

---

## src/gameplay/systems/collision/death.rs

```rust
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crate::gameplay::components::{ball::Ball, collider::Collider};
use crate::gameplay::resources::lives::Lives;
use crate::app::states::GameState;

pub fn ball_death(
    windows: Query<&Window, With<PrimaryWindow>>,
    mut ball_q: Query<(&mut Ball, &mut Transform, &Collider)>,
    mut lives: ResMut<Lives>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let Ok(window) = windows.single() else { return };
    let half_height = window.height() / 2.0;

    for (mut ball, mut transform, collider) in ball_q.iter_mut() {
        let half_h = collider.size.y / 2.0;
        if transform.translation.y - half_h <= -half_height {
            if lives.0 > 0 { lives.0 -= 1; }
            if lives.0 == 0 {
                next_state.set(GameState::GameOver);
            }
            transform.translation = Vec3::new(0.0, -220.0, 0.0);
            ball.velocity = Vec2::new(200.0, 200.0);
        }
    }
}
```

---

## src/gameplay/spawn/mod.rs

```rust
use bevy::prelude::*;
use crate::gameplay::components::paddle::Paddle;
use crate::gameplay::components::ball::Ball;
use crate::gameplay::components::collider::Collider;

mod level;

#[derive(Component)]
pub struct GameEntity;

pub fn cleanup_game(mut commands: Commands, query: Query<Entity, With<GameEntity>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn spawn_game(mut commands: Commands, ball_query: Query<Entity, With<Ball>>) {
    if !ball_query.is_empty() {
        return; // возврат из паузы — не спавним заново
    }
    spawn_paddle(commands.reborrow());
    spawn_ball(commands.reborrow());
    level::setup_level(commands);
}

fn spawn_paddle(mut commands: Commands) {
    let size = Vec2::new(120.0, 20.0);
    commands.spawn((
        Sprite {
            color: Color::srgba(0.8, 0.2, 0.2, 1.0),
            custom_size: Some(size),
            ..Default::default()
        },
        Transform::from_translation(Vec3::new(0.0, -250.0, 0.0)),
        GlobalTransform::default(),
        Paddle { speed: 500.0, direction: 0.0 },
        Collider { size },
        GameEntity,
    ));
}

fn spawn_ball(mut commands: Commands) {
    let size = Vec2::new(20.0, 20.0);

    let mut velocity = Vec2::new(200.0, 200.0);
    let speed = velocity.length();
    velocity = velocity.normalize() * speed;

    commands.spawn((
        Sprite {
            color: Color::srgba(0.9, 0.9, 0.2, 1.0),
            custom_size: Some(size),
            ..Default::default()
        },
        Transform::from_translation(Vec3::new(0.0, -220.0, 0.0)),
        GlobalTransform::default(),
        Ball { velocity },
        Collider { size },
        GameEntity,
    ));
}
```

---

## src/gameplay/spawn/level.rs

```rust
use bevy::prelude::*;
use crate::gameplay::components::collider::Collider;
use crate::gameplay::components::brick::{Brick, BrickType};
use super::GameEntity;

pub struct Block {
    pub position: Vec3,
    pub size: Vec2,
    pub color: Color,
}

pub fn spawn_block(commands: &mut Commands, block: &Block, brick_type: BrickType) {
    let (health, color, score) = match brick_type {
        BrickType::Normal => (1, block.color, 100),
        BrickType::Strong => (2, Color::srgba(0.8, 0.4, 0.2, 1.0), 200),
    };
    commands.spawn((
        Sprite { color, custom_size: Some(block.size), ..Default::default() },
        Transform::from_translation(block.position),
        GlobalTransform::default(),
        Collider { size: block.size },
        Brick { brick_type, health, score },
        GameEntity,
    ));
}

pub fn setup_level(mut commands: Commands) {
    let rows = 5;
    let cols = 10;
    let block_width = 50.0;
    let block_height = 20.0;
    let spacing = 5.0;
    let commands = &mut commands;

    for row in 0..rows {
        for col in 0..cols {
            let x = col as f32 * (block_width + spacing)
                - (cols as f32 * (block_width + spacing)) / 2.0
                + block_width / 2.0;
            let y = 250.0 - row as f32 * (block_height + spacing);
            let block = Block {
                position: Vec3::new(x, y, 0.0),
                size: Vec2::new(block_width, block_height),
                color: Color::srgba(0.3, 0.7 - row as f32 * 0.1, 0.5, 1.0),
            };
            let brick_type = if row % 2 == 0 { BrickType::Normal } else { BrickType::Strong };
            spawn_block(commands, &block, brick_type);
        }
    }
}
```

---

## src/ui/mod.rs

```rust
pub mod plugin;
pub mod screens;
```

---

## src/ui/plugin.rs

```rust
use bevy::prelude::*;
use crate::app::states::GameState;
use crate::ui::screens::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app
            // Main Menu
            .add_systems(OnEnter(GameState::MainMenu), setup_main_menu)
            .add_systems(OnExit(GameState::MainMenu), cleanup_main_menu)
            .add_systems(
                Update,
                (main_menu_input, update_menu_visuals).run_if(in_state(GameState::MainMenu)),
            )

            // Settings
            .add_systems(OnEnter(GameState::Settings), setup_settings_ui)
            .add_systems(OnExit(GameState::Settings), cleanup_settings_ui)
            .add_systems(
                Update,
                settings_input.run_if(in_state(GameState::Settings)),
            )

            // Pause
            .add_systems(OnEnter(GameState::Paused), setup_pause_ui)
            .add_systems(OnExit(GameState::Paused), cleanup_pause_ui)
            .add_systems(
                Update,
                pause_input.run_if(in_state(GameState::Paused)),
            )

            // Game Over
            .add_systems(OnEnter(GameState::GameOver), setup_game_over)
            .add_systems(OnExit(GameState::GameOver), cleanup_game_over)
            .add_systems(
                Update,
                restart_game.run_if(in_state(GameState::GameOver)),
            );
    }
}
```

---

## src/ui/screens/mod.rs

```rust
pub mod main_menu;
pub mod pause;
pub mod game_over;
pub mod settings;

pub use main_menu::*;
pub use pause::*;
pub use game_over::*;
pub use settings::*;
```

---

## src/ui/screens/main_menu.rs

```rust
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

pub fn setup_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
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
        parent.spawn((
            Text::new("BREAKOUT"),
            TextFont { font: font.clone(), font_size: 64.0, ..default() },
            TextColor(Color::WHITE),
            Node { align_self: AlignSelf::Center, ..default() },
        ));
        for (i, item) in MENU_ITEMS.iter().enumerate() {
            parent.spawn((
                Text::new(*item),
                TextFont { font: font.clone(), font_size: 40.0, ..default() },
                TextColor(if i == 0 { Color::srgb(1.0, 1.0, 0.0) } else { Color::WHITE }),
                MenuItem { index: i },
                Node { align_self: AlignSelf::Center, ..default() },
            ));
        }
    });
}

pub fn cleanup_main_menu(mut commands: Commands, query: Query<Entity, With<MainMenuUI>>) {
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
    if keys.just_pressed(KeyCode::ArrowUp) && menu_state.selected > 0 {
        menu_state.selected -= 1;
    }
    if keys.just_pressed(KeyCode::ArrowDown) && menu_state.selected < MENU_ITEMS.len() - 1 {
        menu_state.selected += 1;
    }
    if keys.just_pressed(KeyCode::Enter) || keys.just_pressed(KeyCode::NumpadEnter) {
        match menu_state.selected {
            0 => next_state.set(GameState::Playing),
            1 => next_state.set(GameState::Settings),
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
        color.0 = if item.index == menu_state.selected {
            Color::srgb(1.0, 1.0, 0.0)
        } else {
            Color::WHITE
        };
    }
}
```

---

## src/ui/screens/pause.rs

```rust
use bevy::prelude::*;
use crate::app::states::GameState;

#[derive(Component)]
pub struct PauseUI;

pub fn setup_pause_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
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
        parent.spawn((
            Text::new("PAUSED"),
            TextFont { font: font.clone(), font_size: 64.0, ..default() },
            TextColor(Color::WHITE),
            Node { align_self: AlignSelf::Center, ..default() },
        ));
        parent.spawn((
            Text::new("ESC - Resume\nEnter - Quit to Menu"),
            TextFont { font, font_size: 32.0, ..default() },
            TextColor(Color::srgb(0.8, 0.8, 0.8)),
            Node { align_self: AlignSelf::Center, margin: UiRect::top(Val::Px(20.0)), ..default() },
        ));
    });
}

pub fn cleanup_pause_ui(mut commands: Commands, query: Query<Entity, With<PauseUI>>) {
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
```

---

## src/ui/screens/game_over.rs

```rust
use bevy::prelude::*;
use crate::app::states::GameState;

#[derive(Component)]
pub struct GameOverText;

pub fn setup_game_over(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Text::new("GAME OVER\nPress SPACE to restart"),
        TextFont {
            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
            font_size: 48.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Percent(40.0),
            left: Val::Percent(25.0),
            ..default()
        },
        GameOverText,
    ));
}

pub fn cleanup_game_over(mut commands: Commands, query: Query<Entity, With<GameOverText>>) {
    for e in query.iter() {
        commands.entity(e).despawn();
    }
}

pub fn restart_game(
    keys: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keys.just_pressed(KeyCode::Space) {
        next_state.set(GameState::Playing);
    }
}
```

---

## src/ui/screens/settings.rs

```rust
use bevy::prelude::*;
use crate::core::config::GameConfig;
use crate::app::states::GameState;

#[derive(Component)]
pub struct SettingsUI;

#[derive(Component)]
pub struct SettingsItem {
    pub index: usize,
}

#[derive(Resource)]
pub struct SettingsState {
    pub selected: usize,
}

const SETTINGS_ITEMS: [&str; 3] = ["Music", "SFX", "Back"];

pub fn setup_settings_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    config: Res<GameConfig>,
) {
    commands.insert_resource(SettingsState { selected: 0 });
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
        SettingsUI,
    ))
    .with_children(|parent| {
        parent.spawn((
            Text::new("SETTINGS"),
            TextFont { font: font.clone(), font_size: 64.0, ..default() },
            TextColor(Color::WHITE),
            Node { align_self: AlignSelf::Center, ..default() },
        ));
        parent.spawn((
            Text::new(format!("Music: {:.1}", config.music_volume)),
            TextFont { font: font.clone(), font_size: 40.0, ..default() },
            TextColor(Color::srgb(1.0, 1.0, 0.0)),
            SettingsItem { index: 0 },
            Node { align_self: AlignSelf::Center, ..default() },
        ));
        parent.spawn((
            Text::new(format!("SFX: {:.1}", config.sfx_volume)),
            TextFont { font: font.clone(), font_size: 40.0, ..default() },
            TextColor(Color::WHITE),
            SettingsItem { index: 1 },
            Node { align_self: AlignSelf::Center, ..default() },
        ));
        parent.spawn((
            Text::new("Back"),
            TextFont { font: font.clone(), font_size: 40.0, ..default() },
            TextColor(Color::WHITE),
            SettingsItem { index: 2 },
            Node { align_self: AlignSelf::Center, ..default() },
        ));
        parent.spawn((
            Text::new("←/→ change value | Enter = save | Esc = back"),
            TextFont { font, font_size: 24.0, ..default() },
            TextColor(Color::srgb(0.5, 0.5, 0.5)),
            Node { align_self: AlignSelf::Center, margin: UiRect::top(Val::Px(20.0)), ..default() },
        ));
    });
}

pub fn cleanup_settings_ui(mut commands: Commands, query: Query<Entity, With<SettingsUI>>) {
    for e in query.iter() {
        commands.entity(e).despawn();
    }
    commands.remove_resource::<SettingsState>();
}

pub fn settings_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut settings_state: ResMut<SettingsState>,
    mut config: ResMut<GameConfig>,
    mut next_state: ResMut<NextState<GameState>>,
    mut query: Query<(&SettingsItem, &mut TextColor, &mut Text)>,
) {
    let mut changed = false;
    let mut update_display = false;

    if keys.just_pressed(KeyCode::ArrowUp) && settings_state.selected > 0 {
        settings_state.selected -= 1;
        update_display = true;
    }
    if keys.just_pressed(KeyCode::ArrowDown) && settings_state.selected < SETTINGS_ITEMS.len() - 1 {
        settings_state.selected += 1;
        update_display = true;
    }

    if settings_state.selected == 0 {
        if keys.just_pressed(KeyCode::ArrowLeft) {
            config.music_volume = (config.music_volume - 0.1).clamp(0.0, 1.0);
            changed = true; update_display = true;
        }
        if keys.just_pressed(KeyCode::ArrowRight) {
            config.music_volume = (config.music_volume + 0.1).clamp(0.0, 1.0);
            changed = true; update_display = true;
        }
    } else if settings_state.selected == 1 {
        if keys.just_pressed(KeyCode::ArrowLeft) {
            config.sfx_volume = (config.sfx_volume - 0.1).clamp(0.0, 1.0);
            changed = true; update_display = true;
        }
        if keys.just_pressed(KeyCode::ArrowRight) {
            config.sfx_volume = (config.sfx_volume + 0.1).clamp(0.0, 1.0);
            changed = true; update_display = true;
        }
    }

    if keys.just_pressed(KeyCode::Enter) || keys.just_pressed(KeyCode::NumpadEnter) {
        config.save();
        if settings_state.selected == 2 {
            next_state.set(GameState::MainMenu);
        }
    }
    if keys.just_pressed(KeyCode::Escape) {
        next_state.set(GameState::MainMenu);
    }

    if update_display || changed {
        for (item, mut color, mut text) in query.iter_mut() {
            color.0 = if item.index == settings_state.selected {
                Color::srgb(1.0, 1.0, 0.0)
            } else {
                Color::WHITE
            };
            if item.index == 0 { text.0 = format!("Music: {:.1}", config.music_volume); }
            else if item.index == 1 { text.0 = format!("SFX: {:.1}", config.sfx_volume); }
        }
    }
}
```

---

**Дата обновления:** 30 марта 2026 г.
