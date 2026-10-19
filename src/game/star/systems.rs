use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::prelude::*;

use super::components::Star;
use super::resources::*;
use super::{NUMBER_OF_STARS, STAR_SIZE};

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

pub fn despawn_stars(mut commands: Commands, star_query: Query<Entity, With<Star>>) {
    for star_entity in star_query {
        commands.entity(star_entity).despawn();
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
