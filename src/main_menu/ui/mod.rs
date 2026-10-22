mod game_over_menu;
mod hud;
mod name_entry;
mod pause_menu;
mod scoreboard;

use game_over_menu::GameOverMenuPlugin;
use hud::HudPlugin;
use name_entry::NameEntryPlugin;
use pause_menu::PauseMenuPlugin;
use scoreboard::ScoreboardPlugin;

use bevy::prelude::*;

pub struct GameUIPlugin;

impl Plugin for GameUIPlugin {
    fn build(&self, app: &mut App) {
        app
            // Plugins
            .add_plugins((
                HudPlugin,
                PauseMenuPlugin,
                GameOverMenuPlugin,
                NameEntryPlugin,
                ScoreboardPlugin,
            ));
    }
}
