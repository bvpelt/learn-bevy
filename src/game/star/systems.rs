use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::prelude::*;

use super::STAR_SIZE;
use super::components::Star;
use super::resources::*;
use crate::events::LevelUp;
use crate::game::HUD_HEIGHT;
use crate::game::level::resources::Level;

fn spawn_star(commands: &mut Commands, window: &Window, asset_server: &AssetServer) {
    let mut rng = rand::rng();
    let star_half_size = STAR_SIZE / 2.0;

    let x = rng.random_range(star_half_size..window.width() - star_half_size);
    // stay below the HUD
    let y = rng.random_range(star_half_size..window.height() - HUD_HEIGHT - star_half_size);

    commands.spawn((
        Sprite {
            image: asset_server.load("sprites/star.png"),
            ..default()
        },
        Transform::from_xyz(x, y, 0.0),
        Star {},
    ));
}

/// Start of a level: spawn the initial stars and set the star timer of this level.
fn start_level_stars(
    commands: &mut Commands,
    window: &Window,
    asset_server: &AssetServer,
    level: &mut Level,
    star_spawn_timer: &mut StarSpawnTimer,
) {
    star_spawn_timer.timer = Timer::from_seconds(level.star_spawn_secs, TimerMode::Repeating);

    let count = level.initial_stars();
    level.stars_spawned = count;
    for _ in 0..count {
        spawn_star(commands, window, asset_server);
    }
}

pub fn spawn_stars(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    mut level: ResMut<Level>,
    mut star_spawn_timer: ResMut<StarSpawnTimer>,
) {
    let window = window_query.single().unwrap();
    start_level_stars(
        &mut commands,
        window,
        &asset_server,
        &mut level,
        &mut star_spawn_timer,
    );
}

pub fn spawn_stars_on_level_up(
    mut level_up_reader: MessageReader<LevelUp>,
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    mut level: ResMut<Level>,
    mut star_spawn_timer: ResMut<StarSpawnTimer>,
) {
    if level_up_reader.read().last().is_some() {
        let window = window_query.single().unwrap();
        start_level_stars(
            &mut commands,
            window,
            &asset_server,
            &mut level,
            &mut star_spawn_timer,
        );
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
    star_spawn_timer: Res<StarSpawnTimer>,
    mut level: ResMut<Level>,
) {
    // spawn only when the timer has finished and this level still has stars to spawn
    if star_spawn_timer.timer.is_finished() && level.stars_spawned < level.stars_total {
        let window = window_query.single().unwrap();
        spawn_star(&mut commands, window, &asset_server);
        level.stars_spawned += 1;
    }
}
