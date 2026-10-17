pub mod enemy;
mod player;
pub mod score;
pub mod star;

use enemy::EnemyPlugin;
use player::PlayerPlugin;
use score::ScorePlugin;
use star::StarPlugin;

use bevy::prelude::*;

use crate::events::GameOver;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            // Messages
            .add_message::<GameOver>()
            // Plugins
            .add_plugins((EnemyPlugin, PlayerPlugin, ScorePlugin, StarPlugin));
    }
}
