use bevy::prelude::*;

pub mod resources;
mod systems;

use resources::*;
use systems::*;

use super::SimulationState;
use crate::AppState;
use crate::events::LevelUp;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app
            // Resources
            .init_resource::<Level>()
            // Messages
            .add_message::<LevelUp>()
            // Systems
            .add_systems(
                Update,
                check_level_complete
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(SimulationState::Running)),
            )
            // On Exit State: next game (restart or from main menu) starts at level 1
            .add_systems(OnExit(AppState::Game), reset_level);
    }
}
