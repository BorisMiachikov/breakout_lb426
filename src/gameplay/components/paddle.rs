use bevy::prelude::*;

#[derive(Component)]
pub struct Paddle {
    pub speed: f32,
    pub direction: f32, // -1.0 влево, 1.0 вправо, 0.0 — стоим
}