pub mod enemy;
mod player;
pub mod score;
pub mod star;
mod systems;

use enemy::EnemyPlugin;
use player::PlayerPlugin;
use score::ScorePlugin;
use star::StarPlugin;
use systems::*;

use bevy::prelude::*;

use crate::AppState;
use crate::events::GameOver;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            // Game state
            .init_state::<SimulationState>()
            // Sound state
            .init_state::<SoundState>()
            // Messages
            .add_message::<GameOver>()
            // On Enter System
            .add_systems(OnEnter(AppState::Game), pause_simulation)
            // Plugins
            .add_plugins((EnemyPlugin, PlayerPlugin, ScorePlugin, StarPlugin))
            // Systems
            .add_systems(
                Update,
                (toggle_simulation, toggle_sound).run_if(in_state(AppState::Game)),
            )
            // On Exit System
            .add_systems(OnExit(AppState::Game), resume_simulation);
    }
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum SimulationState {
    #[default]
    Running,
    Paused,
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum SoundState {
    #[default]
    SoundOff,
    SoundOn,
}
