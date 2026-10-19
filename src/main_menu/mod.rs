mod components;
mod styles;
mod systems;

use crate::AppState;
use bevy::prelude::*;

use systems::layout::*;
pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            // Startup
            //.add_systems(Startup, main_menu)
            // On Enter State
            .add_systems(OnEnter(AppState::MainMenu), spawn_main_menu)
            // On Exit State
            .add_systems(OnExit(AppState::MainMenu), despawn_main_menu);
    }
}

pub fn main_menu() {
    println!("You are on the main menu");
}
