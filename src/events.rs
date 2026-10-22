use bevy::prelude::*;

#[derive(Message)]
pub struct GameOver {
    pub score: u32,
    pub level: u32,
}

#[derive(Message)]
pub struct LevelUp;
