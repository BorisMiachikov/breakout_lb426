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
        Sprite {
            color,
            custom_size: Some(block.size),
            ..Default::default()
        },
        Transform::from_translation(block.position),
        GlobalTransform::default(),
        Collider { size: block.size },
        Brick {
            brick_type,
            health,
            score,
        },
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

            let brick_type = if row % 2 == 0 {
                BrickType::Normal
            } else {
                BrickType::Strong
            };

            spawn_block(commands, &block, brick_type);
        }
    }
}
