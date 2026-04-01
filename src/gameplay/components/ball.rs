use bevy::prelude::*;

#[derive(Component)]
pub struct Ball {
    pub velocity: Vec2,
    pub launched: bool,
}
