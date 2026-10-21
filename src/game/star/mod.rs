use bevy::prelude::*;

pub mod components;
pub mod resources;
mod systems;

use resources::*;
use systems::*;

use super::SimulationState;
use crate::AppState;

// The number of stars per level is defined in game::level::resources
pub const STAR_SIZE: f32 = 30.0; // This is the star sprite size.

pub struct StarPlugin;

impl Plugin for StarPlugin {
    fn build(&self, app: &mut App) {
        app
            // Resources
            .init_resource::<StarSpawnTimer>()
            // On Enter State
            .add_systems(OnEnter(AppState::Game), spawn_stars)
            // Systems
            .add_systems(
                Update,
                (tick_star_spawn_timer, spawn_stars_over_time)
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(SimulationState::Running)),
            )
            .add_systems(
                Update,
                spawn_stars_on_level_up.run_if(in_state(AppState::Game)),
            )
            // Exit State Systems
            .add_systems(OnExit(AppState::Game), despawn_stars);
    }
}
