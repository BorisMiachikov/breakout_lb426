use bevy::prelude::*;

use crate::core::audio::{play_sfx, AudioAssets};
use crate::core::camera::{playfield_left, playfield_right, VIRTUAL_HEIGHT};
use crate::core::config::GameConfig;
use crate::gameplay::components::{ball::Ball, collider::Collider};

pub fn ball_wall_collision(
    mut commands: Commands,
    audio_assets: Res<AudioAssets>,
    config: Res<GameConfig>,
    mut query: Query<(&mut Ball, &mut Transform, &Collider)>,
) {
    let half_height = VIRTUAL_HEIGHT / 2.0;
    let left = playfield_left();
    let right = playfield_right();

    for (mut ball, mut transform, collider) in query.iter_mut() {
        let half_w = collider.size.x / 2.0;
        let half_h = collider.size.y / 2.0;
        let mut did_bounce = false;

        if transform.translation.x - half_w <= left {
            transform.translation.x = left + half_w;
            ball.velocity.x = ball.velocity.x.abs();
            did_bounce = true;
        }

        if transform.translation.x + half_w >= right {
            transform.translation.x = right - half_w;
            ball.velocity.x = -ball.velocity.x.abs();
            did_bounce = true;
        }

        if transform.translation.y + half_h >= half_height {
            transform.translation.y = half_height - half_h;
            ball.velocity.y = -ball.velocity.y.abs();
            did_bounce = true;
        }

        if did_bounce {
            play_sfx(&mut commands, &audio_assets.bounce, &config, 0.55);
        }
    }
}
