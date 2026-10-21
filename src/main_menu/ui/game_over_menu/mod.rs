mod components;
mod styles;
mod systems;

use bevy::prelude::*;

use crate::AppState;

use systems::interactions::*;
use systems::layout::*;
use systems::updates::update_final_score_text;

pub struct GameOverMenuPlugin;

impl Plugin for GameOverMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            // On Enter
            .add_systems(OnEnter(AppState::GameOver), spawn_game_over_menu)
            // Systems
            .add_systems(
                Update,
                (
                    update_final_score_text,
                    interact_with_restart_button,
                    interact_with_main_menu_button,
                    interact_with_quit_button,
                )
                    .run_if(in_state(AppState::GameOver)),
            )
            // On Exit
            .add_systems(OnExit(AppState::GameOver), despawn_game_over_menu);
    }
}
