use bevy::prelude::*;

use crate::core::camera::{playfield_left, playfield_right};
use crate::gameplay::components::ball::Ball;
use crate::gameplay::components::collider::Collider;
use crate::gameplay::components::paddle::Paddle;

const BALL_STICK_OFFSET: f32 = 2.0;

pub fn ball_movement(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &Ball)>,
) {
    for (mut transform, ball) in query.iter_mut() {
        if !ball.launched {
            continue;
        }

        transform.translation.x += ball.velocity.x * time.delta_secs();
        transform.translation.y += ball.velocity.y * time.delta_secs();
    }
}

pub fn stick_ball_to_paddle(
    paddle_q: Query<(&Transform, &Collider), (With<Paddle>, Without<Ball>)>,
    mut ball_q: Query<(&mut Ball, &mut Transform, &Collider), With<Ball>>,
) {
    let Ok((paddle_tf, paddle_col)) = paddle_q.single() else {
        return;
    };

    for (mut ball, mut ball_tf, ball_col) in ball_q.iter_mut() {
        if ball.launched {
            continue;
        }

        ball.velocity = Vec2::ZERO;
        ball_tf.translation.x = paddle_tf.translation.x;
        ball_tf.translation.y = paddle_tf.translation.y
            + paddle_col.size.y / 2.0
            + ball_col.size.y / 2.0
            + BALL_STICK_OFFSET;
    }
}

pub fn paddle_movement(
    time: Res<Time>,
    mut query: Query<
        (&Paddle, &mut Transform, &crate::gameplay::components::collider::Collider),
        Without<Ball>,
    >,
) {
    for (paddle, mut transform, collider) in query.iter_mut() {
        transform.translation.x += paddle.direction * paddle.speed * time.delta_secs();

        let half_width = collider.size.x / 2.0;
        let min_x = playfield_left() + half_width;
        let max_x = playfield_right() - half_width;

        transform.translation.x = transform.translation.x.clamp(min_x, max_x);
    }
}
