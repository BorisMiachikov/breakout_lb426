use bevy::prelude::*;
use crate::app::states::GameState;
use crate::ui::screens::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app
            // Main Menu
            .add_systems(OnEnter(GameState::MainMenu), setup_main_menu)
            .add_systems(OnExit(GameState::MainMenu), cleanup_main_menu)
            .add_systems(
                Update,
                (
                    main_menu_input,
                    main_menu_mouse_input,
                    update_menu_visuals,
                    adapt_main_menu_layout,
                )
                    .run_if(in_state(GameState::MainMenu)),
            )

            // High Scores
            .add_systems(OnEnter(GameState::HighScores), setup_high_scores)
            .add_systems(OnExit(GameState::HighScores), cleanup_high_scores)
            .add_systems(
                Update,
                (
                    high_scores_input,
                    high_scores_mouse_input,
                    update_high_scores_visuals,
                    adapt_high_scores_layout,
                )
                    .run_if(in_state(GameState::HighScores)),
            )

            // Playing HUD
            .add_systems(OnEnter(GameState::Playing), setup_playing_hud)
            .add_systems(OnExit(GameState::Playing), cleanup_playing_hud)
            .add_systems(Update, update_playing_hud.run_if(in_state(GameState::Playing)))

            // Settings
            .add_systems(OnEnter(GameState::Settings), setup_settings_ui)
            .add_systems(OnExit(GameState::Settings), cleanup_settings_ui)
            .add_systems(
                Update,
                (
                    settings_input,
                    settings_mouse_input,
                    update_settings_visuals,
                    adapt_settings_layout,
                )
                    .run_if(in_state(GameState::Settings)),
            )

            // Pause
            .add_systems(OnEnter(GameState::Paused), setup_pause_ui)
            .add_systems(OnExit(GameState::Paused), cleanup_pause_ui)
            .add_systems(
                Update,
                (pause_input, pause_mouse_input, update_pause_visuals, adapt_pause_layout)
                    .run_if(in_state(GameState::Paused)),
            )

            // Level Complete
            .add_systems(OnEnter(GameState::LevelComplete), setup_level_complete)
            .add_systems(OnExit(GameState::LevelComplete), cleanup_level_complete)
            .add_systems(
                Update,
                (level_complete_input, adapt_level_complete_layout)
                    .run_if(in_state(GameState::LevelComplete)),
            )

            // Game Over
            .add_systems(OnEnter(GameState::GameOver), setup_game_over)
            .add_systems(OnExit(GameState::GameOver), cleanup_game_over)
            .add_systems(
                Update,
                (restart_game, adapt_game_over_layout).run_if(in_state(GameState::GameOver)),
            )

            // Victory
            .add_systems(OnEnter(GameState::Victory), setup_victory)
            .add_systems(OnExit(GameState::Victory), cleanup_victory)
            .add_systems(
                Update,
                (victory_input, adapt_victory_layout).run_if(in_state(GameState::Victory)),
            );
    }
}
