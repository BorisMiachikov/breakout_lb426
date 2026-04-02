use bevy::prelude::*;

#[derive(Component)]
pub struct Brick {
    pub health: u8,
    pub score: u32,
}
