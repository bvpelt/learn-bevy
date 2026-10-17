use bevy::prelude::*;

pub const ENEMY_SPAWN_TIME: f32 = 5.0; // This is the time in seconds between star spawns.

#[derive(Resource)]
pub struct EnemySpawnTimer {
    pub timer: Timer,
}

// default trait implementation for EnemySpawnTimer
impl Default for EnemySpawnTimer {
    fn default() -> Self {
        EnemySpawnTimer {
            timer: Timer::from_seconds(ENEMY_SPAWN_TIME, TimerMode::Repeating),
        }
    }
}
