mod components;
mod styles;
mod systems;

use crate::AppState;
use bevy::prelude::*;
use systems::layout::*;
use systems::updates::{update_enemy_text, update_level_text, update_score_text};

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app
            // On Enter
            .add_systems(OnEnter(AppState::Game), spawn_hud_menu)
            // Systems
            .add_systems(
                Update,
                (update_score_text, update_level_text, update_enemy_text)
                    .run_if(in_state(AppState::Game)),
            )
            // On Exit
            .add_systems(OnExit(AppState::Game), despawn_hud_menu);
    }
}
