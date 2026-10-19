use bevy::prelude::*;

pub mod components;
pub mod resources;
mod systems;

use crate::{AppState, game::SimulationState};
use resources::*;
use systems::*;

pub const NUMBER_OF_ENEMIES: usize = 4;
pub const ENEMY_SPEED: f32 = 200.0; // This is the speed at which enemies move.
pub const ENEMY_SIZE: f32 = 64.0; // This is the enemy sprite size.
pub const PLAY_SOUNDS: bool = true;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app
            // Resources
            .init_resource::<EnemySpawnTimer>()
            // Startup systems
            .add_systems(Startup, spawn_enemies)
            // Systems
            /*
            .add_systems(
                Update,
                enemy_movement
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(SimulationState::Running)),
            )
            .add_systems(Update, update_enemy_detection)
            .add_systems(Update, confine_enemy_movement)
            .add_systems(Update, tick_enemy_spawn_timer)
            .add_systems(Update, spawn_enemies_over_time)
             */
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
            );
    }
}
