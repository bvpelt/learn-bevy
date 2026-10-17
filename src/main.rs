pub mod events;
mod game;
mod main_menu;
mod systems;

use game::GamePlugin;
use main_menu::MainMenuPlugin;

use systems::*;

use bevy::prelude::*;

fn main() {
    App::new()
        // Bevy Plugins
        .add_plugins(DefaultPlugins)
        // My Plugins
        .add_plugins(MainMenuPlugin)
        .add_plugins(GamePlugin)
        // Startup system
        .add_systems(Startup, spawn_camera)
        // Systems
        .add_systems(Update, exit_game)
        .add_systems(Update, handle_game_over)
        .run();
}
