mod components;
mod systems;

use bevy::prelude::*;

use crate::AppState;
use systems::*;

pub struct NameEntryPlugin;

impl Plugin for NameEntryPlugin {
    fn build(&self, app: &mut App) {
        app
            // On Enter
            .add_systems(OnEnter(AppState::NameEntry), spawn_name_entry)
            // Systems
            .add_systems(
                Update,
                (
                    name_typing,
                    update_name_text,
                    interact_with_start_button,
                    interact_with_back_button,
                )
                    .run_if(in_state(AppState::NameEntry)),
            )
            // On Exit
            .add_systems(OnExit(AppState::NameEntry), despawn_name_entry);
    }
}
