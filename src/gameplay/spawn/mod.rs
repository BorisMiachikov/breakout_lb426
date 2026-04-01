use bevy::prelude::*;

use crate::gameplay::components::paddle::Paddle;
use crate::gameplay::components::ball::Ball;
use crate::gameplay::components::collider::Collider;
use crate::gameplay::resources::{Lives, Score};

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
    current_level: Res<CurrentLevelPath>,
    ball_query: Query<Entity, With<Ball>>,
) {
    if !ball_query.is_empty() {
        return;
    }
    spawn_paddle(commands.reborrow());
    spawn_ball(commands.reborrow(), &asset_server);
    level::setup_level(commands, current_level);
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

    let mut velocity = Vec2::new(200.0, 200.0);
    let speed = velocity.length();
    velocity = velocity.normalize() * speed;
    let texture = asset_server.load("textures/ball.png");

    commands.spawn((
        Sprite {
            image: texture,
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
