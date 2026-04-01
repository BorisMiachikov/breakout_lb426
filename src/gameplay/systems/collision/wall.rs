use bevy::prelude::*;

use crate::core::camera::{playfield_left, playfield_right, VIRTUAL_HEIGHT};
use crate::gameplay::components::{ball::Ball, collider::Collider};

pub fn ball_wall_collision(
    mut query: Query<(&mut Ball, &mut Transform, &Collider)>,
) {
    let half_height = VIRTUAL_HEIGHT / 2.0;
    let left = playfield_left();
    let right = playfield_right();

    for (mut ball, mut transform, collider) in query.iter_mut() {
        let half_w = collider.size.x / 2.0;
        let half_h = collider.size.y / 2.0;

        if transform.translation.x - half_w <= left {
            transform.translation.x = left + half_w;
            ball.velocity.x = ball.velocity.x.abs();
        }

        if transform.translation.x + half_w >= right {
            transform.translation.x = right - half_w;
            ball.velocity.x = -ball.velocity.x.abs();
        }

        if transform.translation.y + half_h >= half_height {
            transform.translation.y = half_height - half_h;
            ball.velocity.y = -ball.velocity.y.abs();
        }
    }
}
