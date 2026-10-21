use bevy::prelude::*;

pub mod components;
pub mod resources;
mod systems;

use crate::{AppState, game::SimulationState};
use resources::*;
use systems::*;

pub const NUMBER_OF_ENEMIES: usize = 4; // Number of enemies at the start of a game (capped by the level maximum).
pub const ENEMY_SPEED: f32 = 200.0; // This is the speed at which enemies move.
pub const ENEMY_SIZE: f32 = 64.0; // This is the enemy sprite size.
pub const ENEMY_SAFE_SPAWN_DISTANCE: f32 = 200.0; // Minimum distance between a new enemy and the player.

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app
            // Resources
            .init_resource::<EnemySpawnTimer>()
            // Startup systems
            .add_systems(OnEnter(AppState::Game), spawn_enemies)
            // Systems
            .add_systems(
                Update,
                (
                    enemy_movement,
                    update_enemy_detection,
                    confine_enemy_movement,
                    tick_enemy_spawn_timer,
                    spawn_enemies_over_time,
                )
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(SimulationState::Running)),
            )
            .add_systems(
                Update,
                reset_enemy_spawn_timer_on_level_up.run_if(in_state(AppState::Game)),
            )
            // Exit State Systems
            .add_systems(OnExit(AppState::Game), despawn_enemies);
    }
}
