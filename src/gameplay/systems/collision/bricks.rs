use bevy::prelude::*;
use crate::gameplay::components::{ball::Ball, collider::Collider, brick::Brick, paddle::Paddle};
use crate::gameplay::resources::score::Score;
use crate::utils::math::aabb_collision;

pub fn ball_brick_collision(
    mut commands: Commands,
    mut score: ResMut<Score>,
    mut ball_q: Query<(&mut Ball, &mut Transform, &Collider), (With<Ball>, Without<Paddle>)>,
    mut brick_q: Query<(Entity, &mut Brick, &Transform, &Collider), (With<Brick>, Without<Ball>)>,
) {
    let Ok((mut ball, mut ball_tf, ball_col)) = ball_q.single_mut() else { return };

    for (entity, mut brick, brick_tf, brick_col) in brick_q.iter_mut() {
        if aabb_collision(
            ball_tf.translation,
            ball_col.size,
            brick_tf.translation,
            brick_col.size,
        ) {
            let dx = ball_tf.translation.x - brick_tf.translation.x;
            let dy = ball_tf.translation.y - brick_tf.translation.y;

            let overlap_x = (ball_col.size.x + brick_col.size.x) / 2.0 - dx.abs();
            let overlap_y = (ball_col.size.y + brick_col.size.y) / 2.0 - dy.abs();

            if overlap_x < overlap_y {
                ball.velocity.x *= -1.0;
                if dx > 0.0 {
                    ball_tf.translation.x += overlap_x;
                } else {
                    ball_tf.translation.x -= overlap_x;
                }
            } else {
                ball.velocity.y *= -1.0;
                if dy > 0.0 {
                    ball_tf.translation.y += overlap_y;
                } else {
                    ball_tf.translation.y -= overlap_y;
                }
            }

            brick.health -= 1;

            if brick.health == 0 {
                score.0 += brick.score;
                commands.entity(entity).despawn();
            }

            break;
        }
    }
}
