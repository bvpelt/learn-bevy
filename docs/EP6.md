# EP6

Based on https://www.youtube.com/watch?v=GOl-kacs8TQ&list=PLVnntJRoP85JHGX7rGDu6LaF3fmDDbqyd&index=6


Learning targets
- spawn in more enemies over time
- use bevy events
- exit the game using events
- game over event

## Step 1
Add new enemies over time

```rust
use bevy::{prelude::*, window::PrimaryWindow};
use rand::prelude::*;

pub const PLAYER_SPEED: f32 = 500.0;
pub const PLAYER_SIZE: f32 = 64.0; // This is the player sprite size.
pub const NUMBER_OF_ENEMIES: usize = 4;
pub const ENEMY_SPEED: f32 = 200.0; // This is the speed at which enemies move.
pub const ENEMY_SIZE: f32 = 64.0; // This is the enemy sprite size.
pub const PLAY_SOUNDS: bool = true;
pub const NUMBER_OF_STARS: usize = 10;
pub const STAR_SIZE: f32 = 30.0; // This is the star sprite size.
pub const STAR_SPAWN_TIME: f32 = 1.0; // This is the time in seconds between star spawns.
pub const ENEMY_SPAWN_TIME: f32 = 5.0; // This is the time in seconds between star spawns.

// Components

#[derive(Component)]
pub struct Player {}

#[derive(Component)]
pub struct Enemy {
    pub direction: Vec2, // This is the direction in which the enemy is moving.
}
#[derive(Component)]
pub struct Star {}

// Resources

#[derive(Resource)]
pub struct Score {
    value: u32,
}

// default trait implementation for Score
impl Default for Score {
    fn default() -> Self {
        Score { value: 0 }
    }
}

#[derive(Resource)]
pub struct StarSpawnTimer {
    timer: Timer,
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
    timer: Timer,
}

// default trait implementation for EnemySpawnTimer
impl Default for EnemySpawnTimer {
    fn default() -> Self {
        EnemySpawnTimer {
            timer: Timer::from_seconds(ENEMY_SPAWN_TIME, TimerMode::Repeating),
        }
    }
}

// main entry point

pub fn ep60() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<Score>() // turbo fish syntax
        .init_resource::<StarSpawnTimer>()
        .init_resource::<EnemySpawnTimer>()
        .add_systems(Startup, spawn_camera)
        .add_systems(Startup, spawn_player)
        .add_systems(Startup, spawn_enemies)
        .add_systems(Startup, spawn_stars)
        .add_systems(Update, player_movement)
        .add_systems(Update, confine_player_movement)
        .add_systems(Update, enemy_movement)
        .add_systems(Update, update_enemy_detection)
        .add_systems(Update, confine_enemy_movement)
        .add_systems(Update, enemy_hit_player)
        .add_systems(Update, player_hit_star)
        .add_systems(Update, update_score)
        .add_systems(Update, tick_star_spawn_timer)
        .add_systems(Update, spawn_stars_over_time)
        .add_systems(Update, tick_enemy_spawn_timer)
        .add_systems(Update, spawn_enemies_over_time)
        .run();
}

// spawning resources

pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.single().unwrap();

    commands.spawn((
        Sprite {
            image: asset_server.load("sprites/ball_blue_large.png"),
            ..default()
        },
        Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        Player {},
    ));
}

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.single().unwrap();

    commands.spawn((
        Camera2d { ..default() },
        Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
    ));
}

pub fn spawn_enemies(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.single().unwrap();
    let mut rng = rand::rng();
    let enemy_half_size = ENEMY_SIZE / 2.0;
    for _ in 0..NUMBER_OF_ENEMIES {
        let x = rng.random_range(enemy_half_size..window.width() - enemy_half_size);
        let y = rng.random_range(enemy_half_size..window.height() - enemy_half_size);

        commands.spawn((
            Sprite {
                image: asset_server.load("sprites/ball_red_large.png"),
                ..default()
            },
            Transform::from_xyz(x, y, 0.0),
            Enemy {
                direction: Vec2::new(rng.random(), rng.random()).normalize(),
            },
        ));
    }
}

pub fn spawn_stars(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.single().unwrap();
    let mut rng = rand::rng();
    let star_half_size = STAR_SIZE / 2.0;
    for _ in 0..NUMBER_OF_STARS {
        let x = rng.random_range(star_half_size..window.width() - star_half_size);
        let y = rng.random_range(star_half_size..window.height() - star_half_size);

        commands.spawn((
            Sprite {
                image: asset_server.load("sprites/star.png"),
                ..default()
            },
            Transform::from_xyz(x, y, 0.0),
            Star {},
        ));
    }
}

// system functions

pub fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = player_query.single_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::ArrowLeft)
            || keyboard_input.pressed(KeyCode::Numpad4)
            || keyboard_input.pressed(KeyCode::KeyA)
        {
            direction += Vec3::new(-1.0, 0.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::ArrowRight)
            || keyboard_input.pressed(KeyCode::Numpad6)
            || keyboard_input.pressed(KeyCode::KeyD)
        {
            direction += Vec3::new(1.0, 0.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::ArrowUp)
            || keyboard_input.pressed(KeyCode::Numpad8)
            || keyboard_input.pressed(KeyCode::KeyW)
        {
            direction += Vec3::new(0.0, 1.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::ArrowDown)
            || keyboard_input.pressed(KeyCode::Numpad2)
            || keyboard_input.pressed(KeyCode::KeyS)
        {
            direction += Vec3::new(0.0, -1.0, 0.0);
        }

        if direction.length() > 0.0 {
            direction = direction.normalize();
        }

        // using time.delta_seconds() ensures that the movement is frame-rate independent, meaning the player will move at the same speed regardless of the frame rate.
        // time is the time since the last frame, so multiplying the speed by this value ensures that the player moves at a consistent speed over time.
        transform.translation += direction * PLAYER_SPEED * time.delta_secs();
    }
}

pub fn confine_player_movement(
    mut player_query: Query<&mut Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok(mut player_transform) = player_query.single_mut() {
        let window = window_query.single().unwrap();

        let half_player_size = PLAYER_SIZE / 2.0;
        let x_min = half_player_size;
        let x_max = window.width() - half_player_size;
        let y_min = half_player_size;
        let y_max = window.height() - half_player_size;

        let mut translation = player_transform.translation;

        // Bound player x position
        if translation.x < x_min {
            translation.x = x_min;
        } else if translation.x > x_max {
            translation.x = x_max;
        }

        // Bound player y position
        if translation.y < y_min {
            translation.y = y_min;
        } else if translation.y > y_max {
            translation.y = y_max;
        }

        player_transform.translation = translation;
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
) {
    let window = window_query.single().unwrap();
    let half_enemy_size = ENEMY_SIZE / 2.0;

    let x_min: f32 = 0.0 + half_enemy_size;
    let x_max: f32 = window.width() - half_enemy_size;
    let y_min: f32 = 0.0 + half_enemy_size;
    let y_max: f32 = window.height() - half_enemy_size;

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
        if direction_changed && PLAY_SOUNDS {
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
    let y_max = window.height() - half_enemy_size;

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

pub fn enemy_hit_player(
    mut commands: Commands,
    mut player_query: Query<(Entity, &Transform), With<Player>>,
    enemy_query: Query<&Transform, With<Enemy>>,
    asset_server: Res<AssetServer>,
) {
    if let Ok((player_entity, player_transform)) = player_query.single_mut() {
        for enemy_transform in enemy_query.iter() {
            let distance = player_transform
                .translation
                .distance(enemy_transform.translation);
            let player_radius = PLAYER_SIZE / 2.0;
            let enemy_radius = ENEMY_SIZE / 2.0;

            // collision detection between player and enemy
            if distance < player_radius + enemy_radius {
                println!("Player hit by enemy!, Game Over!");

                if PLAY_SOUNDS {
                    let sound_effect = asset_server.load("audio/explosionCrunch_000.ogg");
                    commands.spawn(AudioPlayer::new(sound_effect));
                }
                commands.entity(player_entity).despawn();
            }
        }
    }
}

pub fn player_hit_star(
    mut commands: Commands,
    mut player_query: Query<&Transform, With<Player>>,
    star_query: Query<(Entity, &Transform), With<Star>>,
    asset_server: Res<AssetServer>,
    mut score: ResMut<Score>,
) {
    if let Ok(player_transform) = player_query.single_mut() {
        for (star_entity, star_transform) in star_query.iter() {
            let distance = player_transform
                .translation
                .distance(star_transform.translation);
            let player_radius = PLAYER_SIZE / 2.0;
            let star_radius = STAR_SIZE / 2.0;

            // collision detection between player and star
            if distance < player_radius + star_radius {
                println!("Player hit by star!");
                score.value += 1;
                if PLAY_SOUNDS {
                    let sound_effect = asset_server.load("audio/laserLarge_000.ogg");
                    commands.spawn(AudioPlayer::new(sound_effect));
                }
                commands.entity(star_entity).despawn();
            }
        }
    }
}

pub fn update_score(score: Res<Score>) {
    if score.is_changed() {
        println!("Score updated: {}", score.value.to_string());
    }
}

pub fn tick_star_spawn_timer(time: Res<Time>, mut star_spawn_timer: ResMut<StarSpawnTimer>) {
    star_spawn_timer.timer.tick(time.delta());
}

pub fn spawn_stars_over_time(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    star_spawn_timer: ResMut<StarSpawnTimer>,
) {
    if star_spawn_timer.timer.is_finished() {
        // timer has finished, spawn a star
        let window = window_query.single().unwrap();
        let mut rng = rand::rng();
        let star_half_size = STAR_SIZE / 2.0;

        let x = rng.random_range(star_half_size..window.width() - star_half_size);
        let y = rng.random_range(star_half_size..window.height() - star_half_size);

        commands.spawn((
            Sprite {
                image: asset_server.load("sprites/star.png"),
                ..default()
            },
            Transform::from_xyz(x, y, 0.0),
            Star {},
        ));
    }
}

pub fn tick_enemy_spawn_timer(time: Res<Time>, mut enemy_spawn_timer: ResMut<EnemySpawnTimer>) {
    enemy_spawn_timer.timer.tick(time.delta());
}

pub fn spawn_enemies_over_time(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    enemy_spawn_timer: ResMut<EnemySpawnTimer>,
) {
    if enemy_spawn_timer.timer.is_finished() {
        // timer has finished, spawn an enemy
        let window = window_query.single().unwrap();
        let mut rng = rand::rng();
        let enemy_half_size = ENEMY_SIZE / 2.0;

        let x = rng.random_range(enemy_half_size..window.width() - enemy_half_size);
        let y = rng.random_range(enemy_half_size..window.height() - enemy_half_size);

        commands.spawn((
            Sprite {
                image: asset_server.load("sprites/ball_red_large.png"),
                ..default()
            },
            Transform::from_xyz(x, y, 0.0),
            Enemy {
                direction: Vec2::new(rng.random(), rng.random()).normalize(),
            },
        ));
    }
}
```

## Step 2
Add exit handler to become active when esc key is hit

```rust
use bevy::app::AppExit;
use bevy::{prelude::*, window::PrimaryWindow};
use rand::prelude::*;

pub const PLAYER_SPEED: f32 = 500.0;
pub const PLAYER_SIZE: f32 = 64.0; // This is the player sprite size.
pub const NUMBER_OF_ENEMIES: usize = 4;
pub const ENEMY_SPEED: f32 = 200.0; // This is the speed at which enemies move.
pub const ENEMY_SIZE: f32 = 64.0; // This is the enemy sprite size.
pub const PLAY_SOUNDS: bool = true;
pub const NUMBER_OF_STARS: usize = 10;
pub const STAR_SIZE: f32 = 30.0; // This is the star sprite size.
pub const STAR_SPAWN_TIME: f32 = 1.0; // This is the time in seconds between star spawns.
pub const ENEMY_SPAWN_TIME: f32 = 5.0; // This is the time in seconds between star spawns.

// Components

#[derive(Component)]
pub struct Player {}

#[derive(Component)]
pub struct Enemy {
    pub direction: Vec2, // This is the direction in which the enemy is moving.
}
#[derive(Component)]
pub struct Star {}

// Resources

#[derive(Resource)]
pub struct Score {
    value: u32,
}

// default trait implementation for Score
impl Default for Score {
    fn default() -> Self {
        Score { value: 0 }
    }
}

#[derive(Resource)]
pub struct StarSpawnTimer {
    timer: Timer,
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
    timer: Timer,
}

// default trait implementation for EnemySpawnTimer
impl Default for EnemySpawnTimer {
    fn default() -> Self {
        EnemySpawnTimer {
            timer: Timer::from_seconds(ENEMY_SPAWN_TIME, TimerMode::Repeating),
        }
    }
}

// main entry point

pub fn ep60() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<Score>() // turbo fish syntax
        .init_resource::<StarSpawnTimer>()
        .init_resource::<EnemySpawnTimer>()
        .add_systems(Startup, spawn_camera)
        .add_systems(Startup, spawn_player)
        .add_systems(Startup, spawn_enemies)
        .add_systems(Startup, spawn_stars)
        .add_systems(Update, player_movement)
        .add_systems(Update, confine_player_movement)
        .add_systems(Update, enemy_movement)
        .add_systems(Update, update_enemy_detection)
        .add_systems(Update, confine_enemy_movement)
        .add_systems(Update, enemy_hit_player)
        .add_systems(Update, player_hit_star)
        .add_systems(Update, update_score)
        .add_systems(Update, tick_star_spawn_timer)
        .add_systems(Update, spawn_stars_over_time)
        .add_systems(Update, tick_enemy_spawn_timer)
        .add_systems(Update, spawn_enemies_over_time)
        .add_systems(Update, exit_game)
        .run();
}

// spawning resources

pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.single().unwrap();

    commands.spawn((
        Sprite {
            image: asset_server.load("sprites/ball_blue_large.png"),
            ..default()
        },
        Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        Player {},
    ));
}

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.single().unwrap();

    commands.spawn((
        Camera2d { ..default() },
        Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
    ));
}

pub fn spawn_enemies(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.single().unwrap();
    let mut rng = rand::rng();
    let enemy_half_size = ENEMY_SIZE / 2.0;
    for _ in 0..NUMBER_OF_ENEMIES {
        let x = rng.random_range(enemy_half_size..window.width() - enemy_half_size);
        let y = rng.random_range(enemy_half_size..window.height() - enemy_half_size);

        commands.spawn((
            Sprite {
                image: asset_server.load("sprites/ball_red_large.png"),
                ..default()
            },
            Transform::from_xyz(x, y, 0.0),
            Enemy {
                direction: Vec2::new(rng.random(), rng.random()).normalize(),
            },
        ));
    }
}

pub fn spawn_stars(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.single().unwrap();
    let mut rng = rand::rng();
    let star_half_size = STAR_SIZE / 2.0;
    for _ in 0..NUMBER_OF_STARS {
        let x = rng.random_range(star_half_size..window.width() - star_half_size);
        let y = rng.random_range(star_half_size..window.height() - star_half_size);

        commands.spawn((
            Sprite {
                image: asset_server.load("sprites/star.png"),
                ..default()
            },
            Transform::from_xyz(x, y, 0.0),
            Star {},
        ));
    }
}

// system functions

pub fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = player_query.single_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::ArrowLeft)
            || keyboard_input.pressed(KeyCode::Numpad4)
            || keyboard_input.pressed(KeyCode::KeyA)
        {
            direction += Vec3::new(-1.0, 0.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::ArrowRight)
            || keyboard_input.pressed(KeyCode::Numpad6)
            || keyboard_input.pressed(KeyCode::KeyD)
        {
            direction += Vec3::new(1.0, 0.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::ArrowUp)
            || keyboard_input.pressed(KeyCode::Numpad8)
            || keyboard_input.pressed(KeyCode::KeyW)
        {
            direction += Vec3::new(0.0, 1.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::ArrowDown)
            || keyboard_input.pressed(KeyCode::Numpad2)
            || keyboard_input.pressed(KeyCode::KeyS)
        {
            direction += Vec3::new(0.0, -1.0, 0.0);
        }

        if direction.length() > 0.0 {
            direction = direction.normalize();
        }

        // using time.delta_seconds() ensures that the movement is frame-rate independent, meaning the player will move at the same speed regardless of the frame rate.
        // time is the time since the last frame, so multiplying the speed by this value ensures that the player moves at a consistent speed over time.
        transform.translation += direction * PLAYER_SPEED * time.delta_secs();
    }
}

pub fn confine_player_movement(
    mut player_query: Query<&mut Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok(mut player_transform) = player_query.single_mut() {
        let window = window_query.single().unwrap();

        let half_player_size = PLAYER_SIZE / 2.0;
        let x_min = half_player_size;
        let x_max = window.width() - half_player_size;
        let y_min = half_player_size;
        let y_max = window.height() - half_player_size;

        let mut translation = player_transform.translation;

        // Bound player x position
        if translation.x < x_min {
            translation.x = x_min;
        } else if translation.x > x_max {
            translation.x = x_max;
        }

        // Bound player y position
        if translation.y < y_min {
            translation.y = y_min;
        } else if translation.y > y_max {
            translation.y = y_max;
        }

        player_transform.translation = translation;
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
) {
    let window = window_query.single().unwrap();
    let half_enemy_size = ENEMY_SIZE / 2.0;

    let x_min: f32 = 0.0 + half_enemy_size;
    let x_max: f32 = window.width() - half_enemy_size;
    let y_min: f32 = 0.0 + half_enemy_size;
    let y_max: f32 = window.height() - half_enemy_size;

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
        if direction_changed && PLAY_SOUNDS {
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
    let y_max = window.height() - half_enemy_size;

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

pub fn enemy_hit_player(
    mut commands: Commands,
    mut player_query: Query<(Entity, &Transform), With<Player>>,
    enemy_query: Query<&Transform, With<Enemy>>,
    asset_server: Res<AssetServer>,
) {
    if let Ok((player_entity, player_transform)) = player_query.single_mut() {
        for enemy_transform in enemy_query.iter() {
            let distance = player_transform
                .translation
                .distance(enemy_transform.translation);
            let player_radius = PLAYER_SIZE / 2.0;
            let enemy_radius = ENEMY_SIZE / 2.0;

            // collision detection between player and enemy
            if distance < player_radius + enemy_radius {
                println!("Player hit by enemy!, Game Over!");

                if PLAY_SOUNDS {
                    let sound_effect = asset_server.load("audio/explosionCrunch_000.ogg");
                    commands.spawn(AudioPlayer::new(sound_effect));
                }
                commands.entity(player_entity).despawn();
            }
        }
    }
}

pub fn player_hit_star(
    mut commands: Commands,
    mut player_query: Query<&Transform, With<Player>>,
    star_query: Query<(Entity, &Transform), With<Star>>,
    asset_server: Res<AssetServer>,
    mut score: ResMut<Score>,
) {
    if let Ok(player_transform) = player_query.single_mut() {
        for (star_entity, star_transform) in star_query.iter() {
            let distance = player_transform
                .translation
                .distance(star_transform.translation);
            let player_radius = PLAYER_SIZE / 2.0;
            let star_radius = STAR_SIZE / 2.0;

            // collision detection between player and star
            if distance < player_radius + star_radius {
                println!("Player hit by star!");
                score.value += 1;
                if PLAY_SOUNDS {
                    let sound_effect = asset_server.load("audio/laserLarge_000.ogg");
                    commands.spawn(AudioPlayer::new(sound_effect));
                }
                commands.entity(star_entity).despawn();
            }
        }
    }
}

pub fn update_score(score: Res<Score>) {
    if score.is_changed() {
        println!("Score updated: {}", score.value.to_string());
    }
}

pub fn tick_star_spawn_timer(time: Res<Time>, mut star_spawn_timer: ResMut<StarSpawnTimer>) {
    star_spawn_timer.timer.tick(time.delta());
}

pub fn spawn_stars_over_time(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    star_spawn_timer: ResMut<StarSpawnTimer>,
) {
    if star_spawn_timer.timer.is_finished() {
        // timer has finished, spawn a star
        let window = window_query.single().unwrap();
        let mut rng = rand::rng();
        let star_half_size = STAR_SIZE / 2.0;

        let x = rng.random_range(star_half_size..window.width() - star_half_size);
        let y = rng.random_range(star_half_size..window.height() - star_half_size);

        commands.spawn((
            Sprite {
                image: asset_server.load("sprites/star.png"),
                ..default()
            },
            Transform::from_xyz(x, y, 0.0),
            Star {},
        ));
    }
}

pub fn tick_enemy_spawn_timer(time: Res<Time>, mut enemy_spawn_timer: ResMut<EnemySpawnTimer>) {
    enemy_spawn_timer.timer.tick(time.delta());
}

pub fn spawn_enemies_over_time(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    enemy_spawn_timer: ResMut<EnemySpawnTimer>,
) {
    if enemy_spawn_timer.timer.is_finished() {
        // timer has finished, spawn an enemy
        let window = window_query.single().unwrap();
        let mut rng = rand::rng();
        let enemy_half_size = ENEMY_SIZE / 2.0;

        let x = rng.random_range(enemy_half_size..window.width() - enemy_half_size);
        let y = rng.random_range(enemy_half_size..window.height() - enemy_half_size);

        commands.spawn((
            Sprite {
                image: asset_server.load("sprites/ball_red_large.png"),
                ..default()
            },
            Transform::from_xyz(x, y, 0.0),
            Enemy {
                direction: Vec2::new(rng.random(), rng.random()).normalize(),
            },
        ));
    }
}

pub fn exit_game(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut app_exit_event_writer: MessageWriter<AppExit>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        app_exit_event_writer.write(AppExit::Success);
    }
}

```

## Bevy events
Events are used to send data between systems.

Events are just rust structures which
- can be empty
- can contain data

Events are used by MessageWriter.write

### Behaviour
Depending on how systems are run the following behaviour occurs.

- system A runs before system B
    - system A sends an event and system B receives the event in the same frame

- sysem B runs before system A
    - system A sends an event and system B receives the event in the following frame

Events only exist from the time they are send in a frame to the end of the next frame

## Step 3
Generate gameover event

```rust
use bevy::app::AppExit;
use bevy::{prelude::*, window::PrimaryWindow};
use rand::prelude::*;

pub const PLAYER_SPEED: f32 = 500.0;
pub const PLAYER_SIZE: f32 = 64.0; // This is the player sprite size.
pub const NUMBER_OF_ENEMIES: usize = 4;
pub const ENEMY_SPEED: f32 = 200.0; // This is the speed at which enemies move.
pub const ENEMY_SIZE: f32 = 64.0; // This is the enemy sprite size.
pub const PLAY_SOUNDS: bool = true;
pub const NUMBER_OF_STARS: usize = 10;
pub const STAR_SIZE: f32 = 30.0; // This is the star sprite size.
pub const STAR_SPAWN_TIME: f32 = 1.0; // This is the time in seconds between star spawns.
pub const ENEMY_SPAWN_TIME: f32 = 5.0; // This is the time in seconds between star spawns.

// Components

#[derive(Component)]
pub struct Player {}

#[derive(Component)]
pub struct Enemy {
    pub direction: Vec2, // This is the direction in which the enemy is moving.
}
#[derive(Component)]
pub struct Star {}

// Resources

#[derive(Resource)]
pub struct Score {
    value: u32,
}

// default trait implementation for Score
impl Default for Score {
    fn default() -> Self {
        Score { value: 0 }
    }
}

#[derive(Resource)]
pub struct StarSpawnTimer {
    timer: Timer,
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
    timer: Timer,
}

// default trait implementation for EnemySpawnTimer
impl Default for EnemySpawnTimer {
    fn default() -> Self {
        EnemySpawnTimer {
            timer: Timer::from_seconds(ENEMY_SPAWN_TIME, TimerMode::Repeating),
        }
    }
}

#[derive(Message)]
pub struct GameOver {
    pub score: u32,
}
// main entry point

pub fn ep60() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<Score>() // turbo fish syntax
        .init_resource::<StarSpawnTimer>()
        .init_resource::<EnemySpawnTimer>()
        .add_message::<GameOver>()
        .add_systems(Startup, spawn_camera)
        .add_systems(Startup, spawn_player)
        .add_systems(Startup, spawn_enemies)
        .add_systems(Startup, spawn_stars)
        .add_systems(Update, player_movement)
        .add_systems(Update, confine_player_movement)
        .add_systems(Update, enemy_movement)
        .add_systems(Update, update_enemy_detection)
        .add_systems(Update, confine_enemy_movement)
        .add_systems(Update, enemy_hit_player)
        .add_systems(Update, player_hit_star)
        .add_systems(Update, update_score)
        .add_systems(Update, tick_star_spawn_timer)
        .add_systems(Update, spawn_stars_over_time)
        .add_systems(Update, tick_enemy_spawn_timer)
        .add_systems(Update, spawn_enemies_over_time)
        .add_systems(Update, exit_game)
        .add_systems(Update, handle_game_over)
        .run();
}

// spawning resources

pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.single().unwrap();

    commands.spawn((
        Sprite {
            image: asset_server.load("sprites/ball_blue_large.png"),
            ..default()
        },
        Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        Player {},
    ));
}

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.single().unwrap();

    commands.spawn((
        Camera2d { ..default() },
        Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
    ));
}

pub fn spawn_enemies(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.single().unwrap();
    let mut rng = rand::rng();
    let enemy_half_size = ENEMY_SIZE / 2.0;
    for _ in 0..NUMBER_OF_ENEMIES {
        let x = rng.random_range(enemy_half_size..window.width() - enemy_half_size);
        let y = rng.random_range(enemy_half_size..window.height() - enemy_half_size);

        commands.spawn((
            Sprite {
                image: asset_server.load("sprites/ball_red_large.png"),
                ..default()
            },
            Transform::from_xyz(x, y, 0.0),
            Enemy {
                direction: Vec2::new(rng.random(), rng.random()).normalize(),
            },
        ));
    }
}

pub fn spawn_stars(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.single().unwrap();
    let mut rng = rand::rng();
    let star_half_size = STAR_SIZE / 2.0;
    for _ in 0..NUMBER_OF_STARS {
        let x = rng.random_range(star_half_size..window.width() - star_half_size);
        let y = rng.random_range(star_half_size..window.height() - star_half_size);

        commands.spawn((
            Sprite {
                image: asset_server.load("sprites/star.png"),
                ..default()
            },
            Transform::from_xyz(x, y, 0.0),
            Star {},
        ));
    }
}

// system functions

pub fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = player_query.single_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::ArrowLeft)
            || keyboard_input.pressed(KeyCode::Numpad4)
            || keyboard_input.pressed(KeyCode::KeyA)
        {
            direction += Vec3::new(-1.0, 0.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::ArrowRight)
            || keyboard_input.pressed(KeyCode::Numpad6)
            || keyboard_input.pressed(KeyCode::KeyD)
        {
            direction += Vec3::new(1.0, 0.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::ArrowUp)
            || keyboard_input.pressed(KeyCode::Numpad8)
            || keyboard_input.pressed(KeyCode::KeyW)
        {
            direction += Vec3::new(0.0, 1.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::ArrowDown)
            || keyboard_input.pressed(KeyCode::Numpad2)
            || keyboard_input.pressed(KeyCode::KeyS)
        {
            direction += Vec3::new(0.0, -1.0, 0.0);
        }

        if direction.length() > 0.0 {
            direction = direction.normalize();
        }

        // using time.delta_seconds() ensures that the movement is frame-rate independent, meaning the player will move at the same speed regardless of the frame rate.
        // time is the time since the last frame, so multiplying the speed by this value ensures that the player moves at a consistent speed over time.
        transform.translation += direction * PLAYER_SPEED * time.delta_secs();
    }
}

pub fn confine_player_movement(
    mut player_query: Query<&mut Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok(mut player_transform) = player_query.single_mut() {
        let window = window_query.single().unwrap();

        let half_player_size = PLAYER_SIZE / 2.0;
        let x_min = half_player_size;
        let x_max = window.width() - half_player_size;
        let y_min = half_player_size;
        let y_max = window.height() - half_player_size;

        let mut translation = player_transform.translation;

        // Bound player x position
        if translation.x < x_min {
            translation.x = x_min;
        } else if translation.x > x_max {
            translation.x = x_max;
        }

        // Bound player y position
        if translation.y < y_min {
            translation.y = y_min;
        } else if translation.y > y_max {
            translation.y = y_max;
        }

        player_transform.translation = translation;
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
) {
    let window = window_query.single().unwrap();
    let half_enemy_size = ENEMY_SIZE / 2.0;

    let x_min: f32 = 0.0 + half_enemy_size;
    let x_max: f32 = window.width() - half_enemy_size;
    let y_min: f32 = 0.0 + half_enemy_size;
    let y_max: f32 = window.height() - half_enemy_size;

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
        if direction_changed && PLAY_SOUNDS {
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
    let y_max = window.height() - half_enemy_size;

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

pub fn enemy_hit_player(
    mut commands: Commands,
    mut game_over_event_writer: MessageWriter<GameOver>,
    mut player_query: Query<(Entity, &Transform), With<Player>>,
    enemy_query: Query<&Transform, With<Enemy>>,
    asset_server: Res<AssetServer>,
    score: Res<Score>,
) {
    if let Ok((player_entity, player_transform)) = player_query.single_mut() {
        for enemy_transform in enemy_query.iter() {
            let distance = player_transform
                .translation
                .distance(enemy_transform.translation);
            let player_radius = PLAYER_SIZE / 2.0;
            let enemy_radius = ENEMY_SIZE / 2.0;

            // collision detection between player and enemy
            if distance < player_radius + enemy_radius {
                println!("Player hit by enemy!, Game Over!");

                if PLAY_SOUNDS {
                    let sound_effect = asset_server.load("audio/explosionCrunch_000.ogg");
                    commands.spawn(AudioPlayer::new(sound_effect));
                }
                commands.entity(player_entity).despawn();
                game_over_event_writer.write(GameOver { score: score.value });
            }
        }
    }
}

pub fn player_hit_star(
    mut commands: Commands,
    mut player_query: Query<&Transform, With<Player>>,
    star_query: Query<(Entity, &Transform), With<Star>>,
    asset_server: Res<AssetServer>,
    mut score: ResMut<Score>,
) {
    if let Ok(player_transform) = player_query.single_mut() {
        for (star_entity, star_transform) in star_query.iter() {
            let distance = player_transform
                .translation
                .distance(star_transform.translation);
            let player_radius = PLAYER_SIZE / 2.0;
            let star_radius = STAR_SIZE / 2.0;

            // collision detection between player and star
            if distance < player_radius + star_radius {
                println!("Player hit by star!");
                score.value += 1;
                if PLAY_SOUNDS {
                    let sound_effect = asset_server.load("audio/laserLarge_000.ogg");
                    commands.spawn(AudioPlayer::new(sound_effect));
                }
                commands.entity(star_entity).despawn();
            }
        }
    }
}

pub fn update_score(score: Res<Score>) {
    if score.is_changed() {
        println!("Score updated: {}", score.value.to_string());
    }
}

pub fn tick_star_spawn_timer(time: Res<Time>, mut star_spawn_timer: ResMut<StarSpawnTimer>) {
    star_spawn_timer.timer.tick(time.delta());
}

pub fn spawn_stars_over_time(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    star_spawn_timer: ResMut<StarSpawnTimer>,
) {
    if star_spawn_timer.timer.is_finished() {
        // timer has finished, spawn a star
        let window = window_query.single().unwrap();
        let mut rng = rand::rng();
        let star_half_size = STAR_SIZE / 2.0;

        let x = rng.random_range(star_half_size..window.width() - star_half_size);
        let y = rng.random_range(star_half_size..window.height() - star_half_size);

        commands.spawn((
            Sprite {
                image: asset_server.load("sprites/star.png"),
                ..default()
            },
            Transform::from_xyz(x, y, 0.0),
            Star {},
        ));
    }
}

pub fn tick_enemy_spawn_timer(time: Res<Time>, mut enemy_spawn_timer: ResMut<EnemySpawnTimer>) {
    enemy_spawn_timer.timer.tick(time.delta());
}

pub fn spawn_enemies_over_time(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    enemy_spawn_timer: ResMut<EnemySpawnTimer>,
) {
    if enemy_spawn_timer.timer.is_finished() {
        // timer has finished, spawn an enemy
        let window = window_query.single().unwrap();
        let mut rng = rand::rng();
        let enemy_half_size = ENEMY_SIZE / 2.0;

        let x = rng.random_range(enemy_half_size..window.width() - enemy_half_size);
        let y = rng.random_range(enemy_half_size..window.height() - enemy_half_size);

        commands.spawn((
            Sprite {
                image: asset_server.load("sprites/ball_red_large.png"),
                ..default()
            },
            Transform::from_xyz(x, y, 0.0),
            Enemy {
                direction: Vec2::new(rng.random(), rng.random()).normalize(),
            },
        ));
    }
}

pub fn exit_game(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut app_exit_event_writer: MessageWriter<AppExit>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        app_exit_event_writer.write(AppExit::Success);
    }
}

pub fn handle_game_over(
    mut game_over_event_reader: MessageReader<GameOver>,
    mut app_exit_event_writer: MessageWriter<AppExit>,
) {
    for event in game_over_event_reader.read() {
        println!("Game Over! Final Score: {}", event.score.to_string());
        app_exit_event_writer.write(AppExit::Success);
    }
}
```

## Step 4
Register and show high score

```rust
use bevy::app::AppExit;
use bevy::{prelude::*, window::PrimaryWindow};
use rand::prelude::*;

pub const PLAYER_SPEED: f32 = 500.0;
pub const PLAYER_SIZE: f32 = 64.0; // This is the player sprite size.
pub const NUMBER_OF_ENEMIES: usize = 4;
pub const ENEMY_SPEED: f32 = 200.0; // This is the speed at which enemies move.
pub const ENEMY_SIZE: f32 = 64.0; // This is the enemy sprite size.
pub const PLAY_SOUNDS: bool = true;
pub const NUMBER_OF_STARS: usize = 10;
pub const STAR_SIZE: f32 = 30.0; // This is the star sprite size.
pub const STAR_SPAWN_TIME: f32 = 1.0; // This is the time in seconds between star spawns.
pub const ENEMY_SPAWN_TIME: f32 = 5.0; // This is the time in seconds between star spawns.

// Components

#[derive(Component)]
pub struct Player {}

#[derive(Component)]
pub struct Enemy {
    pub direction: Vec2, // This is the direction in which the enemy is moving.
}
#[derive(Component)]
pub struct Star {}

// Resources

#[derive(Resource)]
pub struct Score {
    value: u32,
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
    timer: Timer,
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
    timer: Timer,
}

// default trait implementation for EnemySpawnTimer
impl Default for EnemySpawnTimer {
    fn default() -> Self {
        EnemySpawnTimer {
            timer: Timer::from_seconds(ENEMY_SPAWN_TIME, TimerMode::Repeating),
        }
    }
}

#[derive(Message)]
pub struct GameOver {
    pub score: u32,
}
// main entry point

pub fn ep60() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<Score>() // turbo fish syntax
        .init_resource::<HighScores>()
        .init_resource::<StarSpawnTimer>()
        .init_resource::<EnemySpawnTimer>()
        .add_message::<GameOver>()
        .add_systems(Startup, spawn_camera)
        .add_systems(Startup, spawn_player)
        .add_systems(Startup, spawn_enemies)
        .add_systems(Startup, spawn_stars)
        .add_systems(Update, player_movement)
        .add_systems(Update, confine_player_movement)
        .add_systems(Update, enemy_movement)
        .add_systems(Update, update_enemy_detection)
        .add_systems(Update, confine_enemy_movement)
        .add_systems(Update, enemy_hit_player)
        .add_systems(Update, player_hit_star)
        .add_systems(Update, update_score)
        .add_systems(Update, tick_star_spawn_timer)
        .add_systems(Update, spawn_stars_over_time)
        .add_systems(Update, tick_enemy_spawn_timer)
        .add_systems(Update, spawn_enemies_over_time)
        .add_systems(Update, exit_game)
        .add_systems(Update, handle_game_over)
        .add_systems(Update, update_high_scores)
        .add_systems(Update, high_scores_updated)
        .run();
}

// spawning resources

pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.single().unwrap();

    commands.spawn((
        Sprite {
            image: asset_server.load("sprites/ball_blue_large.png"),
            ..default()
        },
        Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        Player {},
    ));
}

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.single().unwrap();

    commands.spawn((
        Camera2d { ..default() },
        Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
    ));
}

pub fn spawn_enemies(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.single().unwrap();
    let mut rng = rand::rng();
    let enemy_half_size = ENEMY_SIZE / 2.0;
    for _ in 0..NUMBER_OF_ENEMIES {
        let x = rng.random_range(enemy_half_size..window.width() - enemy_half_size);
        let y = rng.random_range(enemy_half_size..window.height() - enemy_half_size);

        commands.spawn((
            Sprite {
                image: asset_server.load("sprites/ball_red_large.png"),
                ..default()
            },
            Transform::from_xyz(x, y, 0.0),
            Enemy {
                direction: Vec2::new(rng.random(), rng.random()).normalize(),
            },
        ));
    }
}

pub fn spawn_stars(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.single().unwrap();
    let mut rng = rand::rng();
    let star_half_size = STAR_SIZE / 2.0;
    for _ in 0..NUMBER_OF_STARS {
        let x = rng.random_range(star_half_size..window.width() - star_half_size);
        let y = rng.random_range(star_half_size..window.height() - star_half_size);

        commands.spawn((
            Sprite {
                image: asset_server.load("sprites/star.png"),
                ..default()
            },
            Transform::from_xyz(x, y, 0.0),
            Star {},
        ));
    }
}

// system functions

pub fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = player_query.single_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::ArrowLeft)
            || keyboard_input.pressed(KeyCode::Numpad4)
            || keyboard_input.pressed(KeyCode::KeyA)
        {
            direction += Vec3::new(-1.0, 0.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::ArrowRight)
            || keyboard_input.pressed(KeyCode::Numpad6)
            || keyboard_input.pressed(KeyCode::KeyD)
        {
            direction += Vec3::new(1.0, 0.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::ArrowUp)
            || keyboard_input.pressed(KeyCode::Numpad8)
            || keyboard_input.pressed(KeyCode::KeyW)
        {
            direction += Vec3::new(0.0, 1.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::ArrowDown)
            || keyboard_input.pressed(KeyCode::Numpad2)
            || keyboard_input.pressed(KeyCode::KeyS)
        {
            direction += Vec3::new(0.0, -1.0, 0.0);
        }

        if direction.length() > 0.0 {
            direction = direction.normalize();
        }

        // using time.delta_seconds() ensures that the movement is frame-rate independent, meaning the player will move at the same speed regardless of the frame rate.
        // time is the time since the last frame, so multiplying the speed by this value ensures that the player moves at a consistent speed over time.
        transform.translation += direction * PLAYER_SPEED * time.delta_secs();
    }
}

pub fn confine_player_movement(
    mut player_query: Query<&mut Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok(mut player_transform) = player_query.single_mut() {
        let window = window_query.single().unwrap();

        let half_player_size = PLAYER_SIZE / 2.0;
        let x_min = half_player_size;
        let x_max = window.width() - half_player_size;
        let y_min = half_player_size;
        let y_max = window.height() - half_player_size;

        let mut translation = player_transform.translation;

        // Bound player x position
        if translation.x < x_min {
            translation.x = x_min;
        } else if translation.x > x_max {
            translation.x = x_max;
        }

        // Bound player y position
        if translation.y < y_min {
            translation.y = y_min;
        } else if translation.y > y_max {
            translation.y = y_max;
        }

        player_transform.translation = translation;
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
) {
    let window = window_query.single().unwrap();
    let half_enemy_size = ENEMY_SIZE / 2.0;

    let x_min: f32 = 0.0 + half_enemy_size;
    let x_max: f32 = window.width() - half_enemy_size;
    let y_min: f32 = 0.0 + half_enemy_size;
    let y_max: f32 = window.height() - half_enemy_size;

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
        if direction_changed && PLAY_SOUNDS {
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
    let y_max = window.height() - half_enemy_size;

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

pub fn enemy_hit_player(
    mut commands: Commands,
    mut game_over_event_writer: MessageWriter<GameOver>,
    mut player_query: Query<(Entity, &Transform), With<Player>>,
    enemy_query: Query<&Transform, With<Enemy>>,
    asset_server: Res<AssetServer>,
    score: Res<Score>,
) {
    if let Ok((player_entity, player_transform)) = player_query.single_mut() {
        for enemy_transform in enemy_query.iter() {
            let distance = player_transform
                .translation
                .distance(enemy_transform.translation);
            let player_radius = PLAYER_SIZE / 2.0;
            let enemy_radius = ENEMY_SIZE / 2.0;

            // collision detection between player and enemy
            if distance < player_radius + enemy_radius {
                println!("Player hit by enemy!, Game Over!");

                if PLAY_SOUNDS {
                    let sound_effect = asset_server.load("audio/explosionCrunch_000.ogg");
                    commands.spawn(AudioPlayer::new(sound_effect));
                }
                commands.entity(player_entity).despawn();
                game_over_event_writer.write(GameOver { score: score.value });
            }
        }
    }
}

pub fn player_hit_star(
    mut commands: Commands,
    mut player_query: Query<&Transform, With<Player>>,
    star_query: Query<(Entity, &Transform), With<Star>>,
    asset_server: Res<AssetServer>,
    mut score: ResMut<Score>,
) {
    if let Ok(player_transform) = player_query.single_mut() {
        for (star_entity, star_transform) in star_query.iter() {
            let distance = player_transform
                .translation
                .distance(star_transform.translation);
            let player_radius = PLAYER_SIZE / 2.0;
            let star_radius = STAR_SIZE / 2.0;

            // collision detection between player and star
            if distance < player_radius + star_radius {
                println!("Player hit by star!");
                score.value += 1;
                if PLAY_SOUNDS {
                    let sound_effect = asset_server.load("audio/laserLarge_000.ogg");
                    commands.spawn(AudioPlayer::new(sound_effect));
                }
                commands.entity(star_entity).despawn();
            }
        }
    }
}

pub fn update_score(score: Res<Score>) {
    if score.is_changed() {
        println!("Score updated: {}", score.value.to_string());
    }
}

pub fn tick_star_spawn_timer(time: Res<Time>, mut star_spawn_timer: ResMut<StarSpawnTimer>) {
    star_spawn_timer.timer.tick(time.delta());
}

pub fn spawn_stars_over_time(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    star_spawn_timer: ResMut<StarSpawnTimer>,
) {
    if star_spawn_timer.timer.is_finished() {
        // timer has finished, spawn a star
        let window = window_query.single().unwrap();
        let mut rng = rand::rng();
        let star_half_size = STAR_SIZE / 2.0;

        let x = rng.random_range(star_half_size..window.width() - star_half_size);
        let y = rng.random_range(star_half_size..window.height() - star_half_size);

        commands.spawn((
            Sprite {
                image: asset_server.load("sprites/star.png"),
                ..default()
            },
            Transform::from_xyz(x, y, 0.0),
            Star {},
        ));
    }
}

pub fn tick_enemy_spawn_timer(time: Res<Time>, mut enemy_spawn_timer: ResMut<EnemySpawnTimer>) {
    enemy_spawn_timer.timer.tick(time.delta());
}

pub fn spawn_enemies_over_time(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    enemy_spawn_timer: ResMut<EnemySpawnTimer>,
) {
    if enemy_spawn_timer.timer.is_finished() {
        // timer has finished, spawn an enemy
        let window = window_query.single().unwrap();
        let mut rng = rand::rng();
        let enemy_half_size = ENEMY_SIZE / 2.0;

        let x = rng.random_range(enemy_half_size..window.width() - enemy_half_size);
        let y = rng.random_range(enemy_half_size..window.height() - enemy_half_size);

        commands.spawn((
            Sprite {
                image: asset_server.load("sprites/ball_red_large.png"),
                ..default()
            },
            Transform::from_xyz(x, y, 0.0),
            Enemy {
                direction: Vec2::new(rng.random(), rng.random()).normalize(),
            },
        ));
    }
}

pub fn exit_game(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut app_exit_event_writer: MessageWriter<AppExit>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        app_exit_event_writer.write(AppExit::Success);
    }
}

pub fn handle_game_over(
    mut game_over_event_reader: MessageReader<GameOver>,
    mut app_exit_event_writer: MessageWriter<AppExit>,
) {
    for event in game_over_event_reader.read() {
        println!("Game Over! Final Score: {}", event.score.to_string());
        app_exit_event_writer.write(AppExit::Success);
    }
}

pub fn update_high_scores(
    mut game_over_event_reader: MessageReader<GameOver>,
    mut high_scores: ResMut<HighScores>,
) {
    for event in game_over_event_reader.read() {
        high_scores.scores.push(("Player".to_string(), event.score));
    }
}

pub fn high_scores_updated(high_scores: Res<HighScores>) {
    if high_scores.is_changed() {
        println!("High Scores: {:?}", high_scores)
    }
}

```