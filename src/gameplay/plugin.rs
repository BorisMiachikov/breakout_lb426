use bevy::prelude::*;

use crate::app::states::GameState;
use crate::gameplay::systems::*;
use crate::gameplay::spawn::*;
use crate::gameplay::resources::*;

pub struct GameplayPlugin;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(CampaignManifest::default())
            .insert_resource(CurrentLevelIndex::default())
            .insert_resource(CurrentLevelPath::default())
            .insert_resource(HighScores::load())
            .insert_resource(LatestRecordedRun::default())
            .insert_resource(Lives(3))
            .insert_resource(Score(0))

            .add_systems(
                OnEnter(GameState::MainMenu),
                (
                    cleanup_game,
                    reset_game_resources,
                    reset_campaign_progress,
                    clear_latest_recorded_run,
                ),
            )
            .add_systems(OnEnter(GameState::LevelComplete), (cleanup_game, advance_to_next_level))
            .add_systems(OnEnter(GameState::GameOver), record_game_over_score)
            .add_systems(OnEnter(GameState::Victory), record_victory_score)
            .add_systems(OnEnter(GameState::Victory), cleanup_game)
            .add_systems(OnEnter(GameState::Playing), spawn_game)

            .add_systems(
                Update,
                (
                    launch_ball,
                    paddle_input,
                    paddle_movement,
                    paddle_mouse_control,
                    stick_ball_to_paddle,
                    ball_movement,
                    (
                        ball_wall_collision,
                        ball_paddle_collision,
                        ball_brick_collision,
                        ball_death,
                        check_level_complete,
                    ),
                )
                    .chain()
                    .run_if(in_state(GameState::Playing)),
            )
            .add_systems(
                Update,
                (game_pause, debug_next_level).run_if(in_state(GameState::Playing)),
            );
    }
}
