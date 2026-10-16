use bevy::prelude::*;

#[derive(Component)]
pub struct Player {}

#[derive(Component)]
pub struct Enemy {
    pub direction: Vec2, // This is the direction in which the enemy is moving.
}
#[derive(Component)]
pub struct Star {}
