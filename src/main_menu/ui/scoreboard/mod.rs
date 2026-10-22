mod components;
mod systems;

use bevy::prelude::*;

use crate::AppState;
use systems::*;

pub struct ScoreboardPlugin;

impl Plugin for ScoreboardPlugin {
    fn build(&self, app: &mut App) {
        app
            // On Enter: built from the current HighScores
            .add_systems(OnEnter(AppState::Scoreboard), spawn_scoreboard)
            // Systems
            .add_systems(
                Update,
                interact_with_back_button.run_if(in_state(AppState::Scoreboard)),
            )
            // On Exit
            .add_systems(OnExit(AppState::Scoreboard), despawn_scoreboard);
    }
}
