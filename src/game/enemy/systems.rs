use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::prelude::*;

use super::components::*;
use super::resources::*;
use super::{ENEMY_SAFE_SPAWN_DISTANCE, ENEMY_SIZE, ENEMY_SPEED};
use crate::events::LevelUp;
use crate::game::HUD_HEIGHT;
use crate::game::SoundState;
use crate::game::level::resources::Level;
use crate::game::player::components::Player;

const MAX_SPAWN_ATTEMPTS: usize = 100;

/// Random position inside the playfield (below the HUD) that is at least
/// ENEMY_SAFE_SPAWN_DISTANCE away from `avoid` (the player position).
fn random_spawn_position(window: &Window, avoid: Vec2) -> Vec2 {
    let mut rng = rand::rng();
    let half = ENEMY_SIZE / 2.0;
    let mut position = Vec2::ZERO;

    for _ in 0..MAX_SPAWN_ATTEMPTS {
        position = Vec2::new(
            rng.random_range(half..window.width() - half),
            rng.random_range(half..window.height() - HUD_HEIGHT - half),
        );

        if position.distance(avoid) >= ENEMY_SAFE_SPAWN_DISTANCE {
            break;
        }
    }

    position
}

fn spawn_enemy(commands: &mut Commands, asset_server: &AssetServer, window: &Window, avoid: Vec2) {
    let mut rng = rand::rng();
    let position = random_spawn_position(window, avoid);

    commands.spawn((
        Sprite {
            image: asset_server.load("sprites/ball_red_large.png"),
            ..default()
        },
        Transform::from_xyz(position.x, position.y, 0.0),
        Enemy {
            direction: Vec2::new(rng.random(), rng.random()).normalize(),
        },
    ));
}

pub fn spawn_enemies(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    level: Res<Level>,
    mut enemy_spawn_timer: ResMut<EnemySpawnTimer>,
) {
    // The timer follows the level
    enemy_spawn_timer.timer = Timer::from_seconds(level.enemy_spawn_secs, TimerMode::Repeating);

    let window = window_query.single().unwrap();

    // The player is not spawned yet (it is spawned in the same OnEnter schedule),
    // but it always starts in the center of the window.
    let player_start = Vec2::new(window.width() / 2.0, window.height() / 2.0);

    for _ in 0..level.initial_enemies() {
        spawn_enemy(&mut commands, &asset_server, window, player_start);
    }
}

pub fn despawn_enemies(mut commands: Commands<'_, '_>, enemy_query: Query<Entity, With<Enemy>>) {
    for enemy_entity in enemy_query.iter() {
        commands.entity(enemy_entity).despawn();
    }
}

pub fn enemy_movement(mut enemy_query: Query<(&mut Transform, &Enemy)>, time: Res<Time>) {
    for (mut transform, enemy) in enemy_query.iter_mut() {
        let direction: Vec3 = Vec3::new(enemy.direction.x, enemy.direction.y, 0.0);
        transform.translation += direction * ENEMY_SPEED * time.delta_secs();
    }
}

pub fn update_enemy_detection(
    mut enemy_query: Query<(&Transform, &mut Enemy)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    sound_state: Res<State<SoundState>>,
) {
    let window = window_query.single().unwrap();
    let half_enemy_size = ENEMY_SIZE / 2.0;

    let x_min: f32 = 0.0 + half_enemy_size;
    let x_max: f32 = window.width() - half_enemy_size;
    let y_min: f32 = 0.0 + half_enemy_size;
    let y_max: f32 = window.height() - HUD_HEIGHT - half_enemy_size; // stay below the HUD

    for (transform, mut enemy) in enemy_query.iter_mut() {
        let mut direction_changed: bool = false;

        let translation = transform.translation;

        if translation.x <= x_min || translation.x >= x_max {
            enemy.direction.x *= -1.0; // Reverse the x direction
            direction_changed = true;
        }

        if translation.y <= y_min || translation.y >= y_max {
            enemy.direction.y *= -1.0; // Reverse the y direction
            direction_changed = true;
        }

        // if direction changed play sound effect
        if direction_changed && sound_state.get() == &SoundState::SoundOn {
            let sound_effect_1 = asset_server.load("audio/pluck_001.ogg");
            let sound_effect_2 = asset_server.load("audio/pluck_002.ogg");
            // random play sound effect 1 or 2
            let sound_effect = if rand::random_range(0.0..1.0) > 0.5 {
                sound_effect_1
            } else {
                sound_effect_2
            };
            commands.spawn(AudioPlayer::new(sound_effect));
        }
    }
}

pub fn confine_enemy_movement(
    mut enemy_query: Query<&mut Transform, With<Enemy>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.single().unwrap();

    let half_enemy_size = ENEMY_SIZE / 2.0;
    let x_min = half_enemy_size;
    let x_max = window.width() - half_enemy_size;
    let y_min = half_enemy_size;
    let y_max = window.height() - HUD_HEIGHT - half_enemy_size; // stay below the HUD

    for mut transform in enemy_query.iter_mut() {
        let mut translation = transform.translation;

        // Bound enemy x position
        if translation.x < x_min {
            translation.x = x_min;
        } else if translation.x > x_max {
            translation.x = x_max;
        }

        // Bound enemy y position
        if translation.y < y_min {
            translation.y = y_min;
        } else if translation.y > y_max {
            translation.y = y_max;
        }

        transform.translation = translation;
    }
}

pub fn tick_enemy_spawn_timer(time: Res<Time>, mut enemy_spawn_timer: ResMut<EnemySpawnTimer>) {
    enemy_spawn_timer.timer.tick(time.delta());
}

pub fn spawn_enemies_over_time(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    enemy_spawn_timer: Res<EnemySpawnTimer>,
    enemy_query: Query<(), With<Enemy>>,
    player_query: Query<&Transform, With<Player>>,
    level: Res<Level>,
) {
    // spawn only when the timer has finished and the level maximum is not reached
    if enemy_spawn_timer.timer.is_finished() && enemy_query.iter().count() < level.max_enemies {
        let window = window_query.single().unwrap();

        // keep away from the player's current position
        let player_position = match player_query.single() {
            Ok(transform) => transform.translation.truncate(),
            Err(_) => Vec2::new(window.width() / 2.0, window.height() / 2.0),
        };

        spawn_enemy(&mut commands, &asset_server, window, player_position);
    }
}

/// New level: use the (faster) enemy spawn time of that level.
pub fn reset_enemy_spawn_timer_on_level_up(
    mut level_up_reader: MessageReader<LevelUp>,
    level: Res<Level>,
    mut enemy_spawn_timer: ResMut<EnemySpawnTimer>,
) {
    if level_up_reader.read().last().is_some() {
        enemy_spawn_timer.timer = Timer::from_seconds(level.enemy_spawn_secs, TimerMode::Repeating);
    }
}
