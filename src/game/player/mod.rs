use bevy::prelude::*;

pub mod components;
mod systems;

use systems::*;

use super::SimulationState;
use crate::AppState;
use crate::game::SoundState;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum PlayerSystemSet {
    Movement,
    Confinement,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            // Configure system sets
            .configure_sets(
                Update,
                PlayerSystemSet::Movement.before(PlayerSystemSet::Confinement),
            )
            // On Enter State
            .add_systems(OnEnter(AppState::Game), spawn_player)
            // Systems
            .add_systems(
                Update,
                player_movement
                    .in_set(PlayerSystemSet::Movement)
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(SimulationState::Running)),
            )
            .add_systems(
                Update,
                confine_player_movement
                    .in_set(PlayerSystemSet::Confinement)
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(SimulationState::Running)),
            )
            .add_systems(
                Update,
                (enemy_hit_player, player_hit_star, toggle_sound)
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(SimulationState::Running)),
            )
            // On Exit State
            .add_systems(OnExit(AppState::Game), despawn_player);
    }
}
