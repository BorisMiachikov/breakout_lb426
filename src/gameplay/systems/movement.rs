use bevy::prelude::*;

use crate::core::camera::VIRTUAL_WIDTH;
use crate::gameplay::components::ball::Ball;
use crate::gameplay::components::paddle::Paddle;

pub fn ball_movement(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &Ball)>,
) {
    for (mut transform, ball) in query.iter_mut() {
        transform.translation.x += ball.velocity.x * time.delta_secs();
        transform.translation.y += ball.velocity.y * time.delta_secs();
    }
}

pub fn paddle_movement(
    time: Res<Time>,
    mut query: Query<
        (&Paddle, &mut Transform, &crate::gameplay::components::collider::Collider),
        Without<Ball>,
    >,
) {
    let screen_half_width = VIRTUAL_WIDTH / 2.0;

    for (paddle, mut transform, collider) in query.iter_mut() {
        transform.translation.x += paddle.direction * paddle.speed * time.delta_secs();

        let half_width = collider.size.x / 2.0;
        let min_x = -screen_half_width + half_width;
        let max_x = screen_half_width - half_width;

        transform.translation.x = transform.translation.x.clamp(min_x, max_x);
    }
}
