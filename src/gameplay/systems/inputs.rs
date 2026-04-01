use bevy::prelude::*;
use bevy::input::keyboard::KeyCode;
use crate::gameplay::components::paddle::Paddle;
use crate::app::states::GameState;

pub fn paddle_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Paddle>,
) {
    for mut paddle in query.iter_mut() {
        let mut direction = 0.0;

        if keyboard.pressed(KeyCode::KeyA) || keyboard.pressed(KeyCode::ArrowLeft) {
            direction -= 1.0;
        }

        if keyboard.pressed(KeyCode::KeyD) || keyboard.pressed(KeyCode::ArrowRight) {
            direction += 1.0;
        }

        paddle.direction = direction;
    }
}

pub fn game_pause(
    keys: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        next_state.set(GameState::Paused);
    }
}