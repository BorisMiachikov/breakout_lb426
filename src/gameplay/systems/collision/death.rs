use bevy::prelude::*;

use crate::app::states::GameState;
use crate::core::camera::VIRTUAL_HEIGHT;
use crate::gameplay::components::{ball::Ball, collider::Collider, paddle::Paddle};
use crate::gameplay::resources::lives::Lives;

pub fn ball_death(
    mut ball_q: Query<(&mut Ball, &mut Transform, &Collider), (With<Ball>, Without<Paddle>)>,
    paddle_q: Query<(&Transform, &Collider), (With<Paddle>, Without<Ball>)>,
    mut lives: ResMut<Lives>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let half_height = VIRTUAL_HEIGHT / 2.0;
    let Ok((paddle_tf, paddle_col)) = paddle_q.single() else {
        return;
    };

    for (mut ball, mut transform, collider) in ball_q.iter_mut() {
        let half_h = collider.size.y / 2.0;

        if transform.translation.y - half_h <= -half_height {
            if lives.0 > 0 {
                lives.0 -= 1;
            }

            if lives.0 == 0 {
                next_state.set(GameState::GameOver);
            }

            transform.translation = Vec3::new(
                paddle_tf.translation.x,
                paddle_tf.translation.y + paddle_col.size.y / 2.0 + collider.size.y / 2.0 + 2.0,
                0.0,
            );
            ball.velocity = Vec2::ZERO;
            ball.launched = false;
        }
    }
}
