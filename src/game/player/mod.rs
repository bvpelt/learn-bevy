use bevy::prelude::*;

pub mod components;
mod systems;

use systems::*;

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
            // Startup system
            .add_systems(Startup, spawn_player)
            // Systems
            .add_systems(Update, player_movement.in_set(PlayerSystemSet::Movement))
            .add_systems(
                Update,
                confine_player_movement.in_set(PlayerSystemSet::Confinement),
            )
            .add_systems(Update, enemy_hit_player)
            .add_systems(Update, player_hit_star);
    }
}
