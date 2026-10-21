pub mod enemy;
pub mod level;
mod player;
pub mod score;
pub mod star;
mod systems;

use enemy::EnemyPlugin;
use level::LevelPlugin;
use player::PlayerPlugin;
use score::ScorePlugin;
use star::StarPlugin;
use systems::*;

use bevy::prelude::*;

use crate::AppState;
use crate::events::GameOver;

/// Height in pixels of the HUD bar at the top of the window.
/// The playfield ends where the HUD starts.
pub const HUD_HEIGHT: f32 = 80.0;

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
            .add_plugins((
                LevelPlugin,
                EnemyPlugin,
                PlayerPlugin,
                ScorePlugin,
                StarPlugin,
            ))
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
