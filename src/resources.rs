use bevy::prelude::*;

pub const STAR_SPAWN_TIME: f32 = 1.0; // This is the time in seconds between star spawns.
pub const ENEMY_SPAWN_TIME: f32 = 5.0; // This is the time in seconds between star spawns.

#[derive(Resource)]
pub struct Score {
    pub value: u32,
}

// default trait implementation for Score
impl Default for Score {
    fn default() -> Self {
        Score { value: 0 }
    }
}

#[derive(Resource, Debug)]
pub struct HighScores {
    pub scores: Vec<(String, u32)>, // player name and score
}

// default trait implementation for Score
impl Default for HighScores {
    fn default() -> Self {
        HighScores { scores: Vec::new() }
    }
}

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
