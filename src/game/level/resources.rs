use bevy::prelude::*;

use crate::game::enemy::NUMBER_OF_ENEMIES;
use crate::game::enemy::resources::ENEMY_SPAWN_TIME;
use crate::game::star::resources::STAR_SPAWN_TIME;

pub const FIRST_LEVEL: u32 = 1;

// Number of stars that are on screen when a level starts.
pub const INITIAL_STARS: u32 = 5;

// Level 1 values and how they change per level.
const BASE_MAX_ENEMIES: usize = 4;
const EXTRA_MAX_ENEMIES_PER_LEVEL: usize = 2;

const BASE_STARS: u32 = 6;
const EXTRA_STARS_PER_LEVEL: u32 = 3;

// Spawn time is multiplied by this factor for every level above 1 (smaller = faster spawning).
const ENEMY_SPAWN_FACTOR: f32 = 0.85;
const MIN_ENEMY_SPAWN_TIME: f32 = 1.0;
const STAR_SPAWN_FACTOR: f32 = 0.9;
const MIN_STAR_SPAWN_TIME: f32 = 0.3;

#[derive(Resource, Debug, Clone)]
pub struct Level {
    pub number: u32,
    pub max_enemies: usize, // maximum number of enemies alive at the same time
    pub enemy_spawn_secs: f32, // seconds between enemy spawns
    pub star_spawn_secs: f32, // seconds between star spawns
    pub stars_total: u32,   // stars that have to be collected to finish the level
    pub stars_spawned: u32, // stars spawned so far in this level
    pub stars_collected: u32, // stars collected so far in this level
}

impl Level {
    /// All level related parameters are derived from the level number.
    pub fn new(number: u32) -> Self {
        let steps = number.saturating_sub(1);

        Level {
            number,
            max_enemies: BASE_MAX_ENEMIES + EXTRA_MAX_ENEMIES_PER_LEVEL * steps as usize,
            enemy_spawn_secs: (ENEMY_SPAWN_TIME * ENEMY_SPAWN_FACTOR.powi(steps as i32))
                .max(MIN_ENEMY_SPAWN_TIME),
            star_spawn_secs: (STAR_SPAWN_TIME * STAR_SPAWN_FACTOR.powi(steps as i32))
                .max(MIN_STAR_SPAWN_TIME),
            stars_total: BASE_STARS + EXTRA_STARS_PER_LEVEL * steps,
            stars_spawned: 0,
            stars_collected: 0,
        }
    }

    /// Go to the next level; re-evaluates every level related parameter.
    pub fn advance(&mut self) {
        *self = Level::new(self.number + 1);
    }

    pub fn initial_enemies(&self) -> usize {
        NUMBER_OF_ENEMIES.min(self.max_enemies)
    }

    pub fn initial_stars(&self) -> u32 {
        INITIAL_STARS.min(self.stars_total)
    }
}

impl Default for Level {
    fn default() -> Self {
        Level::new(FIRST_LEVEL)
    }
}
