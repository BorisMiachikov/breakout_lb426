use bevy::prelude::*;
use crate::gameplay::components::{ball::Ball, collider::Collider, paddle::Paddle};
use crate::utils::math::aabb_collision;

const MAX_SPEED: f32 = 400.0;

pub fn ball_paddle_collision(
    mut ball_q: Query<(&mut Ball, &Transform, &Collider), Without<Paddle>>,
    paddle_q: Query<(&Transform, &Collider), With<Paddle>>,
) {
    let Ok((paddle_tf, paddle_col)) = paddle_q.single() else { return };

    for (mut ball, ball_tf, ball_col) in ball_q.iter_mut() {
        if aabb_collision(
            ball_tf.translation,
            ball_col.size,
            paddle_tf.translation,
            paddle_col.size,
        ) {
            // отражение вверх
            ball.velocity.y = ball.velocity.y.abs();

            // влияние позиции удара
            let offset =
                (ball_tf.translation.x - paddle_tf.translation.x) / (paddle_col.size.x / 2.0);

            ball.velocity.x += offset * 300.0;

            // --- ОГРАНИЧЕНИЕ СКОРОСТИ ---
            let speed = ball.velocity.length();
            if speed > MAX_SPEED {
                ball.velocity = ball.velocity.normalize() * MAX_SPEED;
            }
        }
    }
}