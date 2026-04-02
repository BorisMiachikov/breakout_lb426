use bevy::prelude::*;

use crate::core::camera::{PLAYFIELD_CENTER_X, VIRTUAL_HEIGHT};
use crate::gameplay::components::paddle::Paddle;
use crate::gameplay::components::ball::Ball;
use crate::gameplay::components::collider::Collider;
use crate::gameplay::resources::{CampaignManifest, CurrentLevelIndex, Lives, Score};

mod level;
pub use level::CurrentLevelPath;

#[derive(Component)]
pub struct GameEntity;

pub fn cleanup_game(mut commands: Commands, query: Query<Entity, With<GameEntity>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn reset_game_resources(mut lives: ResMut<Lives>, mut score: ResMut<Score>) {
    lives.0 = 3;
    score.0 = 0;
}

pub fn spawn_game(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut current_level_path: ResMut<CurrentLevelPath>,
    current_level_index: Res<CurrentLevelIndex>,
    manifest: Res<CampaignManifest>,
    ball_query: Query<Entity, With<Ball>>,
) {
    if !ball_query.is_empty() {
        return;
    }

    spawn_game_entities(
        &mut commands,
        &asset_server,
        &mut current_level_path,
        current_level_index.0,
        &manifest,
    );
}

pub fn spawn_game_entities(
    commands: &mut Commands,
    asset_server: &AssetServer,
    current_level_path: &mut CurrentLevelPath,
    current_level_index: usize,
    manifest: &CampaignManifest,
) {
    if let Some(level_path) = manifest.levels.get(current_level_index) {
        current_level_path.0 = level_path.clone();
    }

    spawn_game_background(commands.reborrow(), asset_server);
    spawn_paddle(commands.reborrow(), asset_server);
    spawn_ball(commands.reborrow(), asset_server);
    level::setup_level(commands.reborrow(), current_level_path);
}

fn spawn_game_background(mut commands: Commands, asset_server: &AssetServer) {
    let texture = asset_server.load("backgrounds/game_sat.png");
    let size = Vec2::splat(VIRTUAL_HEIGHT);

    commands.spawn((
        Sprite {
            image: texture,
            custom_size: Some(size),
            color: Color::srgba(1.0, 1.0, 1.0, 0.95),
            ..Default::default()
        },
        Transform::from_translation(Vec3::new(PLAYFIELD_CENTER_X, 0.0, -20.0)),
        GlobalTransform::default(),
        GameEntity,
    ));
}

fn spawn_paddle(mut commands: Commands, asset_server: &AssetServer) {
    let size = Vec2::new(100.0, 20.0);
    let texture = asset_server.load("textures/Paddle.png");

    commands.spawn((
        Sprite {
            image: texture,
            custom_size: Some(size),
            ..Default::default()
        },
        Transform::from_translation(Vec3::new(PLAYFIELD_CENTER_X, -250.0, 0.0)),
        GlobalTransform::default(),
        Paddle {
            speed: 500.0,
            direction: 0.0,
        },
        Collider { size },
        GameEntity,
    ));
}

fn spawn_ball(mut commands: Commands, asset_server: &AssetServer) {
    let size = Vec2::new(20.0, 20.0);
    let texture = asset_server.load("textures/ball.png");

    commands.spawn((
        Sprite {
            image: texture,
            custom_size: Some(size),
            ..Default::default()
        },
        Transform::from_translation(Vec3::new(PLAYFIELD_CENTER_X, -220.0, 0.0)),
        GlobalTransform::default(),
        Ball {
            velocity: Vec2::ZERO,
            launched: false,
        },
        Collider { size },
        GameEntity,
    ));
}
