mod components;
mod styles;
mod systems;

use bevy::prelude::*;

use crate::game::SimulationState;

use systems::interactions::*;
use systems::layout::*;

pub struct PauseMenuPlugin;

impl Plugin for PauseMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            // On Enter
            .add_systems(OnEnter(SimulationState::Paused), spawn_pause_menu)
            // Systems
            .add_systems(
                Update,
                (
                    interact_with_resume_button,
                    interact_with_main_button,
                    interact_with_quit_button,
                )
                    .run_if(in_state(SimulationState::Paused)), // <-- in_state()
            )
            // On Exit
            .add_systems(OnExit(SimulationState::Paused), despawn_pause_menu);
    }
}
