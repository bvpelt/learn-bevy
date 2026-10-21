use bevy::prelude::*;

use super::resources::*;
use crate::events::LevelUp;

pub fn reset_level(mut level: ResMut<Level>) {
    *level = Level::default();
}

pub fn check_level_complete(mut level: ResMut<Level>, mut level_up_writer: MessageWriter<LevelUp>) {
    if level.stars_collected >= level.stars_total {
        level.advance();
        debug!("Level up! Now level {}", level.number);
        level_up_writer.write(LevelUp);
    }
}
