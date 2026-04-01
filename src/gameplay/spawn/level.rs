use std::fs;

use bevy::prelude::*;
use serde::Deserialize;

use crate::core::camera::{playfield_left, PLAYFIELD_WIDTH};
use crate::gameplay::components::brick::{Brick, BrickType};
use crate::gameplay::components::collider::Collider;

use super::GameEntity;

const DEFAULT_LEVEL_PATH: &str = "assets/levels/level1.json";

#[derive(Resource, Clone, Debug)]
pub struct CurrentLevelPath(pub String);

impl Default for CurrentLevelPath {
    fn default() -> Self {
        Self(DEFAULT_LEVEL_PATH.to_string())
    }
}

#[derive(Deserialize, Clone, Debug)]
struct LevelFile {
    #[serde(default = "default_brick_width")]
    brick_width: f32,
    #[serde(default = "default_brick_height")]
    brick_height: f32,
    #[serde(default = "default_spacing")]
    spacing: f32,
    #[serde(default = "default_top_y")]
    top_y: f32,
    rows: Vec<String>,
}

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

pub fn setup_level(mut commands: Commands, current_level: &CurrentLevelPath) {
    let level = load_level(&current_level.0).unwrap_or_else(|error| {
        warn!(
            "Failed to load level from '{}': {}. Falling back to built-in level.",
            current_level.0, error
        );
        default_level()
    });

    spawn_level(&mut commands, &level);
}

fn load_level(path: &str) -> Result<LevelFile, String> {
    let raw = fs::read_to_string(path)
        .map_err(|error| format!("could not read level file: {error}"))?;
    let level: LevelFile = serde_json::from_str(&raw)
        .map_err(|error| format!("could not parse level json: {error}"))?;

    validate_level(&level)?;
    Ok(level)
}

fn validate_level(level: &LevelFile) -> Result<(), String> {
    if level.rows.is_empty() {
        return Err("level must contain at least one row".to_string());
    }

    if level.brick_width <= 0.0 || level.brick_height <= 0.0 {
        return Err("brick size must be positive".to_string());
    }

    if level.spacing < 0.0 {
        return Err("spacing must be non-negative".to_string());
    }

    let expected_columns = level.rows[0].chars().count();
    if expected_columns == 0 {
        return Err("level rows must not be empty".to_string());
    }

    for (row_index, row) in level.rows.iter().enumerate() {
        let columns = row.chars().count();
        if columns != expected_columns {
            return Err(format!(
                "row {} has {} columns, expected {}",
                row_index, columns, expected_columns
            ));
        }

        for cell in row.chars() {
            if !matches!(cell, 'N' | 'S' | '.') {
                return Err(format!(
                    "unsupported cell '{}'. Use 'N', 'S', or '.'",
                    cell
                ));
            }
        }
    }

    Ok(())
}

fn spawn_level(commands: &mut Commands, level: &LevelFile) {
    let cols = level.rows[0].chars().count();
    let layout_width = cols as f32 * level.brick_width + (cols.saturating_sub(1)) as f32 * level.spacing;
    let start_x = playfield_left() + (PLAYFIELD_WIDTH - layout_width) / 2.0 + level.brick_width / 2.0;

    for (row_index, row) in level.rows.iter().enumerate() {
        for (col_index, cell) in row.chars().enumerate() {
            let Some(brick_type) = cell_to_brick_type(cell) else {
                continue;
            };

            let x = start_x + col_index as f32 * (level.brick_width + level.spacing);

            let y = level.top_y - row_index as f32 * (level.brick_height + level.spacing);

            let block = Block {
                position: Vec3::new(x, y, 0.0),
                size: Vec2::new(level.brick_width, level.brick_height),
                color: Color::srgba(0.3, 0.7 - row_index as f32 * 0.1, 0.5, 1.0),
            };

            spawn_block(commands, &block, brick_type);
        }
    }
}

fn cell_to_brick_type(cell: char) -> Option<BrickType> {
    match cell {
        'N' => Some(BrickType::Normal),
        'S' => Some(BrickType::Strong),
        '.' => None,
        _ => None,
    }
}

fn default_level() -> LevelFile {
    LevelFile {
        brick_width: default_brick_width(),
        brick_height: default_brick_height(),
        spacing: default_spacing(),
        top_y: default_top_y(),
        rows: vec![
            "NNNNNNNNNN".to_string(),
            "SSSSSSSSSS".to_string(),
            "NNNNNNNNNN".to_string(),
            "SSSSSSSSSS".to_string(),
            "NNNNNNNNNN".to_string(),
        ],
    }
}

fn default_brick_width() -> f32 {
    50.0
}

fn default_brick_height() -> f32 {
    20.0
}

fn default_spacing() -> f32 {
    5.0
}

fn default_top_y() -> f32 {
    250.0
}
