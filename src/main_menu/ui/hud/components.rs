use bevy::prelude::*;

#[derive(Component)]
pub struct HudMenu {}

#[derive(Component)]
pub struct ScoreText; // star counter (left)

#[derive(Component)]
pub struct LevelText; // current level (middle)

#[derive(Component)]
pub struct EnemyText;
