use bevy::prelude::*;

use crate::app::states::GameState;
use crate::core::camera::{PLAYFIELD_CENTER_X, VIRTUAL_HEIGHT};
use crate::gameplay::components::{ball::Ball, collider::Collider};
use crate::gameplay::resources::lives::Lives;

pub fn ball_death(
    mut ball_q: Query<(&mut Ball, &mut Transform, &Collider)>,
    mut lives: ResMut<Lives>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let half_height = VIRTUAL_HEIGHT / 2.0;

    for (mut ball, mut transform, collider) in ball_q.iter_mut() {
        let half_h = collider.size.y / 2.0;

        if transform.translation.y - half_h <= -half_height {
            if lives.0 > 0 {
                lives.0 -= 1;
            }

            if lives.0 == 0 {
                next_state.set(GameState::GameOver);
            }

            transform.translation = Vec3::new(PLAYFIELD_CENTER_X, -220.0, 0.0);
            ball.velocity = Vec2::new(200.0, 200.0);

            let max_speed = 400.0;
            let speed = ball.velocity.length();
            if speed > max_speed {
                ball.velocity = ball.velocity.normalize() * max_speed;
            }
        }
    }
}
