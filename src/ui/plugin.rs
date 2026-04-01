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
                (main_menu_input, update_menu_visuals).run_if(in_state(GameState::MainMenu)),
            )

            // Settings
            .add_systems(OnEnter(GameState::Settings), setup_settings_ui)
            .add_systems(OnExit(GameState::Settings), cleanup_settings_ui)
            .add_systems(
                Update,
                settings_input.run_if(in_state(GameState::Settings)),
            )

            // Pause
            .add_systems(OnEnter(GameState::Paused), setup_pause_ui)
            .add_systems(OnExit(GameState::Paused), cleanup_pause_ui)
            .add_systems(
                Update,
                pause_input.run_if(in_state(GameState::Paused)),
            )

            // Game Over
            .add_systems(OnEnter(GameState::GameOver), setup_game_over)
            .add_systems(OnExit(GameState::GameOver), cleanup_game_over)
            .add_systems(
                Update,
                restart_game.run_if(in_state(GameState::GameOver)),
            );
    }
}