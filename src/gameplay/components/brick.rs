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