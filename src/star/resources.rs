use bevy::prelude::*;

pub const STAR_SPAWN_TIME: f32 = 1.0; // This is the time in seconds between star spawns.

#[derive(Resource)]
pub struct StarSpawnTimer {
    pub timer: Timer,
}

// default trait implementation for StarSpawnTimer
impl Default for StarSpawnTimer {
    fn default() -> Self {
        StarSpawnTimer {
            timer: Timer::from_seconds(STAR_SPAWN_TIME, TimerMode::Repeating),
        }
    }
}
