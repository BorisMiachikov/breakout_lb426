use bevy::prelude::*;

use crate::gameplay::components::{ball::Ball, collider::Collider, paddle::Paddle};
use crate::utils::math::aabb_collision;

const MAX_SPEED: f32 = 400.0;
const MIN_SPEED: f32 = 220.0;
const MAX_BOUNCE_ANGLE: f32 = std::f32::consts::FRAC_PI_3;
const PADDLE_SEPARATION_EPSILON: f32 = 0.5;

pub fn ball_paddle_collision(
    mut ball_q: Query<(&mut Ball, &mut Transform, &Collider), (With<Ball>, Without<Paddle>)>,
    paddle_q: Query<(&Transform, &Collider), (With<Paddle>, Without<Ball>)>,
) {
    let Ok((paddle_tf, paddle_col)) = paddle_q.single() else {
        return;
    };

    for (mut ball, mut ball_tf, ball_col) in ball_q.iter_mut() {
        if !aabb_collision(
            ball_tf.translation,
            ball_col.size,
            paddle_tf.translation,
            paddle_col.size,
        ) {
            continue;
        }

        let offset =
            ((ball_tf.translation.x - paddle_tf.translation.x) / (paddle_col.size.x / 2.0))
                .clamp(-1.0, 1.0);

        let speed = ball.velocity.length().clamp(MIN_SPEED, MAX_SPEED);
        let bounce_angle = offset * MAX_BOUNCE_ANGLE;

        ball.velocity = Vec2::new(bounce_angle.sin() * speed, bounce_angle.cos() * speed);

        let paddle_top = paddle_tf.translation.y + paddle_col.size.y / 2.0;
        let ball_half_height = ball_col.size.y / 2.0;
        ball_tf.translation.y = paddle_top + ball_half_height + PADDLE_SEPARATION_EPSILON;
    }
}
