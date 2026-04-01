use bevy::prelude::*;

use crate::app::states::GameState;
use crate::gameplay::systems::*;
use crate::gameplay::spawn::*;
use crate::gameplay::resources::*;

pub struct GameplayPlugin;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(Lives(3))
            .insert_resource(Score(0))

            .add_systems(OnEnter(GameState::MainMenu), (cleanup_game, reset_game_resources))
            .add_systems(OnEnter(GameState::Playing), spawn_game)

            .add_systems(
                Update,
                (
                    paddle_input,
                    paddle_movement,
                    ball_movement,
                    (
                        ball_wall_collision,
                        ball_paddle_collision,
                        ball_brick_collision,
                        ball_death,
                    ),
                )
                    .chain()
                    .run_if(in_state(GameState::Playing)),
            )
            .add_systems(
                Update,
                game_pause.run_if(in_state(GameState::Playing)),
            );
    }
}
