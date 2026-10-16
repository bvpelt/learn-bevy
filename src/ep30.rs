use bevy::{prelude::*, window::PrimaryWindow};
use rand::prelude::*;

pub const PLAYER_SPEED: f32 = 500.0;
pub const PLAYER_SIZE: f32 = 64.0; // This is the player sprite size.
pub const NUMBER_OF_ENEMIES: usize = 4;

pub fn ep30() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, spawn_camera)
        .add_systems(Startup, spawn_player)
        .add_systems(Startup, spawn_enemies)
        .add_systems(Update, player_movement)
        .add_systems(Update, confine_player_movement)
        .run();
}

#[derive(Component)]
pub struct Player {}

#[derive(Component)]
pub struct Enemy {}

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

    for _ in 0..NUMBER_OF_ENEMIES {
        let x = rng.random_range(0.0..window.width());
        let y = rng.random_range(0.0..window.height());

        commands.spawn((
            Sprite {
                image: asset_server.load("sprites/ball_red_large.png"),
                ..default()
            },
            Transform::from_xyz(x, y, 0.0),
            Enemy {},
        ));
    }
}

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
