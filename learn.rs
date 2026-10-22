==> src/events.rs <==
use bevy::prelude::*;

#[derive(Message)]
pub struct GameOver {
    pub score: u32,
}

#[derive(Message)]
pub struct LevelUp;

==> src/systems.rs <==
use bevy::app::AppExit;
use bevy::{prelude::*, window::PrimaryWindow};

use crate::AppState;
use crate::events::*;
use crate::game::SimulationState;

// spawning resources

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.single().unwrap();

    commands.spawn((
        Camera2d { ..default() },
        Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
    ));
}

// system functions

pub fn transition_to_game_state(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    app_state: Res<State<AppState>>,
    mut next_app_state: ResMut<NextState<AppState>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyG) {
        if app_state.get() != &AppState::Game {
            next_app_state.set(AppState::Game);
            debug!("Entered AppState::Game");
        }
    }
}

pub fn transition_to_main_menu_state(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    app_state: Res<State<AppState>>,
    mut next_app_state: ResMut<NextState<AppState>>,
    mut next_simulation_state: ResMut<NextState<SimulationState>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyM) {
        if app_state.get() != &AppState::MainMenu {
            next_app_state.set(AppState::MainMenu);
            next_simulation_state.set(SimulationState::Paused);
            debug!("Entered AppState::MainMenu");
        }
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
    mut next_app_state: ResMut<NextState<AppState>>,
) {
    for event in game_over_event_reader.read() {
        debug!("Game Over! Final Score: {}", event.score.to_string());

        next_app_state.set(AppState::GameOver);
    }
}

==> src/game/player/systems.rs <==
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use super::components::Player;
use crate::events::GameOver;
use crate::game::HUD_HEIGHT;
use crate::game::enemy::ENEMY_SIZE;
use crate::game::enemy::components::*;
use crate::game::level::resources::Level;
use crate::game::player::SoundState;
use crate::game::score::resources::*;
use crate::game::star::STAR_SIZE;
use crate::game::star::components::*;

pub const PLAYER_SPEED: f32 = 500.0;
pub const PLAYER_SIZE: f32 = 64.0; // This is the player sprite size.

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

pub fn despawn_player(mut commands: Commands, player_query: Query<Entity, With<Player>>) {
    if let Ok(player_entity) = player_query.single() {
        commands.entity(player_entity).despawn();
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
        let y_max = window.height() - HUD_HEIGHT - half_player_size; // stay below the HUD

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

pub fn enemy_hit_player(
    mut commands: Commands,
    mut game_over_event_writer: MessageWriter<GameOver>,
    mut player_query: Query<(Entity, &Transform), With<Player>>,
    enemy_query: Query<&Transform, With<Enemy>>,
    asset_server: Res<AssetServer>,
    score: Res<Score>,

    sound_state: Res<State<SoundState>>,
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
                debug!("Player hits by enemy!, Game Over!");

                if sound_state.get() == &SoundState::SoundOn {
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
    mut level: ResMut<Level>,
    sound_state: Res<State<SoundState>>,
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
                debug!("Player hit by star!");
                score.value += 1;
                level.stars_collected += 1;
                if sound_state.get() == &SoundState::SoundOn {
                    let sound_effect = asset_server.load("audio/laserLarge_000.ogg");
                    commands.spawn(AudioPlayer::new(sound_effect));
                }
                commands.entity(star_entity).despawn();
            }
        }
    }
}

==> src/game/player/mod.rs <==
use bevy::prelude::*;

pub mod components;
mod systems;

use systems::*;

use super::SimulationState;
use crate::AppState;
use crate::game::SoundState;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum PlayerSystemSet {
    Movement,
    Confinement,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            // Configure system sets
            .configure_sets(
                Update,
                PlayerSystemSet::Movement.before(PlayerSystemSet::Confinement),
            )
            // On Enter State
            .add_systems(OnEnter(AppState::Game), spawn_player)
            // Systems
            .add_systems(
                Update,
                (
                    player_movement.in_set(PlayerSystemSet::Movement),
                    confine_player_movement.in_set(PlayerSystemSet::Confinement),
                )
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(SimulationState::Running)),
            )
            .add_systems(
                Update,
                (enemy_hit_player, player_hit_star)
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(SimulationState::Running)),
            )
            // On Exit State
            .add_systems(OnExit(AppState::Game), despawn_player);
    }
}

==> src/game/player/components.rs <==
use bevy::prelude::*;

#[derive(Component)]
pub struct Player {}

==> src/game/level/systems.rs <==
use bevy::prelude::*;

use super::resources::*;
use crate::events::LevelUp;

pub fn reset_level(mut level: ResMut<Level>) {
    *level = Level::default();
}

pub fn check_level_complete(mut level: ResMut<Level>, mut level_up_writer: MessageWriter<LevelUp>) {
    if level.stars_collected >= level.stars_total {
        level.advance();
        debug!("Level up! Now level {}", level.number);
        level_up_writer.write(LevelUp);
    }
}

==> src/game/level/resources.rs <==
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

==> src/game/level/mod.rs <==
use bevy::prelude::*;

pub mod resources;
mod systems;

use resources::*;
use systems::*;

use super::SimulationState;
use crate::AppState;
use crate::events::LevelUp;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app
            // Resources
            .init_resource::<Level>()
            // Messages
            .add_message::<LevelUp>()
            // Systems
            .add_systems(
                Update,
                check_level_complete
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(SimulationState::Running)),
            )
            // On Exit State: next game (restart or from main menu) starts at level 1
            .add_systems(OnExit(AppState::Game), reset_level);
    }
}

==> src/game/enemy/systems.rs <==
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

==> src/game/enemy/resources.rs <==
use bevy::prelude::*;

pub const ENEMY_SPAWN_TIME: f32 = 5.0; // This is the time in seconds between star spawns.

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

==> src/game/enemy/mod.rs <==
use bevy::prelude::*;

pub mod components;
pub mod resources;
mod systems;

use crate::{AppState, game::SimulationState};
use resources::*;
use systems::*;

pub const NUMBER_OF_ENEMIES: usize = 4; // Number of enemies at the start of a game (capped by the level maximum).
pub const ENEMY_SPEED: f32 = 200.0; // This is the speed at which enemies move.
pub const ENEMY_SIZE: f32 = 64.0; // This is the enemy sprite size.
pub const ENEMY_SAFE_SPAWN_DISTANCE: f32 = 200.0; // Minimum distance between a new enemy and the player.

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app
            // Resources
            .init_resource::<EnemySpawnTimer>()
            // Startup systems
            .add_systems(OnEnter(AppState::Game), spawn_enemies)
            // Systems
            .add_systems(
                Update,
                (
                    enemy_movement,
                    update_enemy_detection,
                    confine_enemy_movement,
                    tick_enemy_spawn_timer,
                    spawn_enemies_over_time,
                )
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(SimulationState::Running)),
            )
            .add_systems(
                Update,
                reset_enemy_spawn_timer_on_level_up.run_if(in_state(AppState::Game)),
            )
            // Exit State Systems
            .add_systems(OnExit(AppState::Game), despawn_enemies);
    }
}

==> src/game/enemy/components.rs <==
use bevy::prelude::*;

#[derive(Component)]
pub struct Enemy {
    pub direction: Vec2, // This is the direction in which the enemy is moving.
}

==> src/game/systems.rs <==
use bevy::prelude::*;

use crate::game::SimulationState;
use crate::game::SoundState;

pub fn pause_simulation(mut next_simulation_state: ResMut<NextState<SimulationState>>) {
    next_simulation_state.set(SimulationState::Paused);
    debug!("Simulation Paused");
}

pub fn resume_simulation(mut next_simulation_state: ResMut<NextState<SimulationState>>) {
    next_simulation_state.set(SimulationState::Running);
    debug!("Simulation Resumed - Running");
}

pub fn toggle_simulation(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    simulation_state: Res<State<SimulationState>>,
    mut next_simulation_state: ResMut<NextState<SimulationState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        if simulation_state.get() == &SimulationState::Running {
            next_simulation_state.set(SimulationState::Paused);
            debug!("Simulation Paused");
        }
        if simulation_state.get() == &SimulationState::Paused {
            next_simulation_state.set(SimulationState::Running);
            debug!("Simulation Running");
        }
    }
}

pub fn toggle_sound(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    sound_state: Res<State<SoundState>>,
    mut next_sound_state: ResMut<NextState<SoundState>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyS) {
        if sound_state.get() == &SoundState::SoundOff {
            next_sound_state.set(SoundState::SoundOn);
            debug!("Sound on");
        } else {
            next_sound_state.set(SoundState::SoundOff);
            debug!("Sound off");
        }
    }
}

==> src/game/mod.rs <==
pub mod enemy;
pub mod level;
mod player;
pub mod score;
pub mod star;
mod systems;

use enemy::EnemyPlugin;
use level::LevelPlugin;
use player::PlayerPlugin;
use score::ScorePlugin;
use star::StarPlugin;
use systems::*;

use bevy::prelude::*;

use crate::AppState;
use crate::events::GameOver;

/// Height in pixels of the HUD bar at the top of the window.
/// The playfield ends where the HUD starts.
pub const HUD_HEIGHT: f32 = 80.0;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            // Game state
            .init_state::<SimulationState>()
            // Sound state
            .init_state::<SoundState>()
            // Messages
            .add_message::<GameOver>()
            // On Enter System
            .add_systems(OnEnter(AppState::Game), pause_simulation)
            // Plugins
            .add_plugins((
                LevelPlugin,
                EnemyPlugin,
                PlayerPlugin,
                ScorePlugin,
                StarPlugin,
            ))
            // Systems
            .add_systems(
                Update,
                (toggle_simulation, toggle_sound).run_if(in_state(AppState::Game)),
            )
            // On Exit System
            .add_systems(OnExit(AppState::Game), resume_simulation);
    }
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum SimulationState {
    #[default]
    Running,
    Paused,
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum SoundState {
    #[default]
    SoundOff,
    SoundOn,
}

==> src/game/score/systems.rs <==
use bevy::prelude::*;

use super::resources::*;
use crate::events::GameOver;

pub fn insert_score(mut commands: Commands) {
    commands.insert_resource(Score::default()); // insert score with initial value 0
}

pub fn remove_score(mut commands: Commands) {
    commands.remove_resource::<Score>();
}

pub fn update_score(score: Res<Score>) {
    if score.is_changed() {
        debug!("Score updated: {}", score.value.to_string());
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
        debug!("High Scores: {:?}", high_scores)
    }
}

==> src/game/score/resources.rs <==
use bevy::prelude::*;

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

==> src/game/score/mod.rs <==
use bevy::prelude::*;

pub mod resources;
mod systems;

use resources::*;
use systems::*;

use crate::AppState;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app
            // Resources
            .init_resource::<HighScores>()
            // On Enter State
            .add_systems(OnEnter(AppState::Game), insert_score)
            // Systems
            .add_systems(Update, update_score.run_if(in_state(AppState::Game)))
            .add_systems(Update, update_high_scores)
            .add_systems(Update, high_scores_updated)
            // On Exit State
            .add_systems(OnExit(AppState::MainMenu), remove_score);
    }
}

==> src/game/star/systems.rs <==
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

==> src/game/star/resources.rs <==
use bevy::prelude::*;

pub const STAR_SPAWN_TIME: f32 = 1.0; // This is the time in seconds between star spawns.

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

==> src/game/star/mod.rs <==
use bevy::prelude::*;

pub mod components;
pub mod resources;
mod systems;

use resources::*;
use systems::*;

use super::SimulationState;
use crate::AppState;

// The number of stars per level is defined in game::level::resources
pub const STAR_SIZE: f32 = 30.0; // This is the star sprite size.

pub struct StarPlugin;

impl Plugin for StarPlugin {
    fn build(&self, app: &mut App) {
        app
            // Resources
            .init_resource::<StarSpawnTimer>()
            // On Enter State
            .add_systems(OnEnter(AppState::Game), spawn_stars)
            // Systems
            .add_systems(
                Update,
                (tick_star_spawn_timer, spawn_stars_over_time)
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(SimulationState::Running)),
            )
            .add_systems(
                Update,
                spawn_stars_on_level_up.run_if(in_state(AppState::Game)),
            )
            // Exit State Systems
            .add_systems(OnExit(AppState::Game), despawn_stars);
    }
}

==> src/game/star/components.rs <==
use bevy::prelude::*;

#[derive(Component)]
pub struct Star {}

==> src/main_menu/styles.rs <==
use bevy::prelude::*;

pub const NORMAL_BUTTON_COLOR: Color = Color::srgb(0.15, 0.15, 0.15);
pub const HOVERED_BUTTON_COLOR: Color = Color::srgb(0.25, 0.25, 0.25);
pub const PRESSED_BUTTON_COLOR: Color = Color::srgb(0.35, 0.75, 0.35);

// Function instead of const — `..default()` is a runtime call
pub fn main_menu_style() -> Node {
    Node {
        flex_direction: FlexDirection::Column,
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        row_gap: Val::Px(8.0), // or UiValue::Px(8.0) for one axis
        ..default()
    }
}

pub fn button_style() -> Node {
    Node {
        width: Val::Px(200.0),
        height: Val::Px(80.0),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    }
}

pub fn title_button_style() -> Node {
    Node {
        flex_direction: FlexDirection::Row,
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        width: Val::Px(500.0),
        height: Val::Px(120.0),
        ..default()
    }
}

pub fn image_style() -> Node {
    Node {
        width: Val::Px(64.0),
        height: Val::Px(64.0),
        margin: UiRect {
            left: Val::Px(8.0),
            right: Val::Px(8.0),
            top: Val::Px(8.0),
            bottom: Val::Px(8.0),
        },
        ..default()
    }
}

//pub fn text_style(font_size: f32, asset_server: &AssetServer) -> (TextFont, TextColor, TextLayout) {
pub fn text_style(
    font_size: f32,
    asset_server: &Res<AssetServer>,
) -> (TextFont, TextColor, TextLayout) {
    (
        TextFont {
            font: asset_server.load("fonts/FiraSans-Bold.ttf").into(),
            font_size: FontSize::Px(font_size),
            ..default()
        },
        TextColor(Color::WHITE),
        TextLayout {
            justify: Justify::Center,
            ..default()
        },
    )
}

==> src/main_menu/ui/game_over_menu/styles.rs <==
use bevy::prelude::*;

// Full-screen centered overlay
pub fn game_over_menu_style() -> Node {
    Node {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    }
}

// The centered panel
pub fn game_over_panel_style() -> Node {
    Node {
        flex_direction: FlexDirection::Column,
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        padding: UiRect::all(Val::Px(24.0)),
        row_gap: Val::Px(16.0),
        border: UiRect::all(Val::Px(2.0)),
        border_radius: BorderRadius::all(Val::Px(12.0)),
        ..default()
    }
}

// A menu button
pub fn game_over_button_style() -> Node {
    Node {
        width: Val::Px(220.0),
        height: Val::Px(60.0),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    }
}

// Title text
pub fn game_over_title_text(asset_server: &AssetServer) -> (Text, TextFont, TextColor) {
    (
        Text::new("Game Over"),
        TextFont {
            font: asset_server.load("fonts/FiraSans-Bold.ttf").into(),
            font_size: FontSize::Px(48.0),
            ..default()
        },
        TextColor(Color::srgb(0.1, 0.1, 0.1)),
    )
}

// Score line
pub fn final_score_text(asset_server: &AssetServer) -> (Text, TextFont, TextColor) {
    (
        Text::new("Your final score was: 0"), // updated by a system once Score is known
        TextFont {
            font: asset_server.load("fonts/FiraSans-Bold.ttf").into(),
            font_size: FontSize::Px(28.0),
            ..default()
        },
        TextColor(Color::srgb(0.1, 0.1, 0.1)),
    )
}

// Button label
pub fn game_over_button_text(
    label: &str,
    asset_server: &AssetServer,
) -> (Text, TextFont, TextColor) {
    (
        Text::new(label.to_string()),
        TextFont {
            font: asset_server.load("fonts/FiraSans-Bold.ttf").into(),
            font_size: FontSize::Px(28.0),
            ..default()
        },
        TextColor(Color::srgb(0.1, 0.1, 0.1)),
    )
}

pub const NORMAL_BUTTON_COLOR: Color = Color::srgb(0.95, 0.95, 0.95);
pub const HOVERED_BUTTON_COLOR: Color = Color::srgb(0.85, 0.85, 0.85);
pub const PRESSED_BUTTON_COLOR: Color = Color::srgb(0.7, 0.7, 0.7);

==> src/main_menu/ui/game_over_menu/mod.rs <==
mod components;
mod styles;
mod systems;

use bevy::prelude::*;

use crate::AppState;

use systems::interactions::*;
use systems::layout::*;
use systems::updates::update_final_score_text;

pub struct GameOverMenuPlugin;

impl Plugin for GameOverMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            // On Enter
            .add_systems(OnEnter(AppState::GameOver), spawn_game_over_menu)
            // Systems
            .add_systems(
                Update,
                (
                    update_final_score_text,
                    interact_with_restart_button,
                    interact_with_main_menu_button,
                    interact_with_quit_button,
                )
                    .run_if(in_state(AppState::GameOver)),
            )
            // On Exit
            .add_systems(OnExit(AppState::GameOver), despawn_game_over_menu);
    }
}

==> src/main_menu/ui/game_over_menu/systems/layout.rs <==
use bevy::prelude::*;

use crate::main_menu::ui::game_over_menu::components::*;
use crate::main_menu::ui::game_over_menu::styles::*;

pub fn spawn_game_over_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let _menu_entity = build_game_over_menu(&mut commands, &asset_server);
}

pub fn despawn_game_over_menu(
    mut commands: Commands,
    menu_query: Query<Entity, With<GameOverMenu>>,
) {
    if let Ok(menu_entity) = menu_query.single() {
        commands.entity(menu_entity).despawn();
    }
}

pub fn build_game_over_menu(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    commands
        .spawn((
            game_over_menu_style(),
            BackgroundColor(Color::srgb(0.4, 0.4, 0.4).with_alpha(0.8)),
            GameOverMenu,
        ))
        .with_children(|overlay| {
            overlay
                .spawn((
                    game_over_panel_style(),
                    BackgroundColor(Color::srgb(0.97, 0.97, 0.97)),
                    BorderColor::all(Color::srgb(0.7, 0.7, 0.7)),
                ))
                .with_children(|panel| {
                    // Title
                    panel.spawn(game_over_title_text(asset_server));

                    // Final score
                    panel.spawn((final_score_text(asset_server), FinalScoreText));

                    // Restart
                    panel
                        .spawn((
                            game_over_button_style(),
                            BackgroundColor(NORMAL_BUTTON_COLOR),
                            Interaction::None,
                            RestartButton,
                        ))
                        .with_children(|btn| {
                            btn.spawn(game_over_button_text("Restart", asset_server));
                        });

                    // Main Menu
                    panel
                        .spawn((
                            game_over_button_style(),
                            BackgroundColor(NORMAL_BUTTON_COLOR),
                            Interaction::None,
                            MainMenuButton,
                        ))
                        .with_children(|btn| {
                            btn.spawn(game_over_button_text("Main Menu", asset_server));
                        });

                    // Quit
                    panel
                        .spawn((
                            game_over_button_style(),
                            BackgroundColor(NORMAL_BUTTON_COLOR),
                            Interaction::None,
                            QuitButton,
                        ))
                        .with_children(|btn| {
                            btn.spawn(game_over_button_text("Quit", asset_server));
                        });
                });
        })
        .id()
}

==> src/main_menu/ui/game_over_menu/systems/mod.rs <==
pub mod interactions;
pub mod layout;
pub mod updates;

==> src/main_menu/ui/game_over_menu/systems/updates.rs <==
use bevy::prelude::*;

use crate::game::score::resources::Score;
use crate::main_menu::ui::game_over_menu::components::FinalScoreText;

pub fn update_final_score_text(
    score: Res<Score>,
    mut query: Query<&mut Text, With<FinalScoreText>>,
) {
    if let Ok(mut text) = query.single_mut() {
        *text = Text::new(format!("Your final score was: {}", score.value));
    }
}

==> src/main_menu/ui/game_over_menu/systems/interactions.rs <==
use bevy::prelude::*;

use crate::AppState;
use crate::main_menu::ui::game_over_menu::components::*;
use crate::main_menu::ui::game_over_menu::styles::{
    HOVERED_BUTTON_COLOR, NORMAL_BUTTON_COLOR, PRESSED_BUTTON_COLOR,
};

pub fn interact_with_restart_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<RestartButton>),
    >,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if let Ok((interaction, mut background_color)) = button_query.single_mut() {
        match *interaction {
            Interaction::Pressed => {
                *background_color = PRESSED_BUTTON_COLOR.into();
                next_state.set(AppState::Game); // re-enter Game → OnEnter systems respawn everything
            }
            Interaction::Hovered => *background_color = HOVERED_BUTTON_COLOR.into(),
            Interaction::None => *background_color = NORMAL_BUTTON_COLOR.into(),
        }
    }
}

pub fn interact_with_main_menu_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<MainMenuButton>),
    >,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if let Ok((interaction, mut background_color)) = button_query.single_mut() {
        match *interaction {
            Interaction::Pressed => {
                *background_color = PRESSED_BUTTON_COLOR.into();
                next_state.set(AppState::MainMenu);
            }
            Interaction::Hovered => *background_color = HOVERED_BUTTON_COLOR.into(),
            Interaction::None => *background_color = NORMAL_BUTTON_COLOR.into(),
        }
    }
}

pub fn interact_with_quit_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<QuitButton>),
    >,
    mut app_exit_event_writer: MessageWriter<AppExit>,
) {
    if let Ok((interaction, mut background_color)) = button_query.single_mut() {
        match *interaction {
            Interaction::Pressed => {
                *background_color = PRESSED_BUTTON_COLOR.into();
                app_exit_event_writer.write(AppExit::Success);
            }
            Interaction::Hovered => *background_color = HOVERED_BUTTON_COLOR.into(),
            Interaction::None => *background_color = NORMAL_BUTTON_COLOR.into(),
        }
    }
}

==> src/main_menu/ui/game_over_menu/components.rs <==
use bevy::prelude::*;

#[derive(Component)]
pub struct GameOverMenu;

#[derive(Component)]
pub struct FinalScoreText; // the "Your final score was: X" text

#[derive(Component)]
pub struct RestartButton;

#[derive(Component)]
pub struct MainMenuButton;

#[derive(Component)]
pub struct QuitButton;

==> src/main_menu/ui/hud/styles.rs <==
use bevy::prelude::*;

use crate::game::HUD_HEIGHT;

pub fn hud_menu_style() -> Node {
    Node {
        width: Val::Percent(100.0),
        height: Val::Px(HUD_HEIGHT),
        justify_content: JustifyContent::SpaceBetween, // pushes left & right groups apart
        align_items: AlignItems::Center,
        padding: UiRect::all(Val::Px(8.0)),
        ..default()
    }
}

// A group: [image][number] or [number][image], laid out horizontally
pub fn hud_group_style() -> Node {
    Node {
        flex_direction: FlexDirection::Row,
        align_items: AlignItems::Center,
        column_gap: Val::Px(8.0),
        ..default()
    }
}

// Small wrapper for an icon image
pub fn hud_icon_style() -> Node {
    Node {
        width: Val::Px(32.0),
        height: Val::Px(32.0),
        ..default()
    }
}

pub fn hud_text_style(asset_server: &AssetServer) -> (Text, TextFont, TextColor) {
    (
        Text::new("0"),
        TextFont {
            font: asset_server.load("fonts/FiraSans-Bold.ttf").into(),
            font_size: FontSize::Px(32.0),
            ..default()
        },
        TextColor(Color::srgb(0.1, 0.1, 0.1)),
    )
}

==> src/main_menu/ui/hud/mod.rs <==
mod components;
mod styles;
mod systems;

use crate::AppState;
use bevy::prelude::*;
use systems::layout::*;
use systems::updates::{update_enemy_text, update_level_text, update_score_text};

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app
            // On Enter
            .add_systems(OnEnter(AppState::Game), spawn_hud_menu)
            // Systems
            .add_systems(
                Update,
                (update_score_text, update_level_text, update_enemy_text)
                    .run_if(in_state(AppState::Game)),
            )
            // On Exit
            .add_systems(OnExit(AppState::Game), despawn_hud_menu);
    }
}

==> src/main_menu/ui/hud/systems/layout.rs <==
use bevy::prelude::*;

use crate::main_menu::ui::hud::components::*;
use crate::main_menu::ui::hud::styles::*;

pub fn spawn_hud_menu(mut commands: Commands<'_, '_>, asset_server: Res<AssetServer>) {
    let _hud_menu_entity = build_hud_menu(&mut commands, &asset_server);
}

pub fn despawn_hud_menu(
    mut commands: Commands<'_, '_>,
    hud_menu_query: Query<Entity, With<HudMenu>>,
) {
    if let Ok(hud_menu_entity) = hud_menu_query.single() {
        commands.entity(hud_menu_entity).despawn();
    }
}

pub fn build_hud_menu(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    commands
        .spawn((
            hud_menu_style(),
            BackgroundColor(Color::srgb(0.9, 0.9, 0.9)),
            HudMenu {},
        ))
        .with_children(|hud| {
            // Left group: [star image] [score]
            hud.spawn(hud_group_style()).with_children(|group| {
                group.spawn((
                    hud_icon_style(),
                    ImageNode::new(asset_server.load("sprites/star.png")),
                ));
                group.spawn((hud_text_style(asset_server), ScoreText));
            });

            // Middle group: [Level N]
            hud.spawn(hud_group_style()).with_children(|group| {
                group.spawn((hud_text_style(asset_server), LevelText));
            });

            // Right group: [enemy count] [ball image]
            hud.spawn(hud_group_style()).with_children(|group| {
                group.spawn((hud_text_style(asset_server), EnemyText));
                group.spawn((
                    hud_icon_style(),
                    ImageNode::new(asset_server.load("sprites/ball_red_large.png")),
                ));
            });
        })
        .id()
}

==> src/main_menu/ui/hud/systems/mod.rs <==
pub mod interactions;
pub mod layout;
pub mod updates;

==> src/main_menu/ui/hud/systems/updates.rs <==
use bevy::prelude::*;

use crate::game::enemy::components::Enemy;
use crate::game::level::resources::Level;
use crate::game::score::resources::Score;
use crate::main_menu::ui::hud::components::*;

pub fn update_score_text(score: Res<Score>, mut query: Query<&mut Text, With<ScoreText>>) {
    if score.is_changed() {
        debug!("Update score stars: {}", score.value);
        if let Ok(mut text) = query.single_mut() {
            *text = Text::new(score.value.to_string());
        }
    }
}

pub fn update_level_text(level: Res<Level>, mut query: Query<&mut Text, With<LevelText>>) {
    if let Ok(mut text) = query.single_mut() {
        *text = Text::new(format!("Level {}", level.number));
    }
}

pub fn update_enemy_text(
    enemies: Query<(), With<Enemy>>, // count live enemies
    mut query: Query<&mut Text, With<EnemyText>>,
) {
    if let Ok(mut text) = query.single_mut() {
        *text = Text::new(enemies.iter().count().to_string());
    }
}

==> src/main_menu/ui/hud/systems/interactions.rs <==

==> src/main_menu/ui/hud/components.rs <==
use bevy::prelude::*;

#[derive(Component)]
pub struct HudMenu {}

#[derive(Component)]
pub struct ScoreText; // star counter (left)

#[derive(Component)]
pub struct LevelText; // current level (middle)

#[derive(Component)]
pub struct EnemyText;

==> src/main_menu/ui/mod.rs <==
mod game_over_menu;
mod hud;
mod pause_menu;

use game_over_menu::GameOverMenuPlugin;
use hud::HudPlugin;
use pause_menu::PauseMenuPlugin;

use bevy::prelude::*;

pub struct GameUIPlugin;

impl Plugin for GameUIPlugin {
    fn build(&self, app: &mut App) {
        app
            // Plugins
            .add_plugins((HudPlugin, PauseMenuPlugin, GameOverMenuPlugin));
    }
}

==> src/main_menu/ui/pause_menu/styles.rs <==
use bevy::prelude::*;

// Full-screen dim overlay
pub fn pause_menu_style() -> Node {
    Node {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        justify_content: JustifyContent::Center, // center panel horizontally
        align_items: AlignItems::Center,         // center panel vertically
        ..default()
    }
}

// The centered white panel
pub fn pause_panel_style() -> Node {
    Node {
        flex_direction: FlexDirection::Column,
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        padding: UiRect::all(Val::Px(24.0)),
        row_gap: Val::Px(16.0),
        border: UiRect::all(Val::Px(2.0)), // border width (needed to see it)
        border_radius: BorderRadius::all(Val::Px(12.0)), // <-- moved here
        ..default()
    }
}

// A menu button
pub fn pause_button_style() -> Node {
    Node {
        width: Val::Px(220.0),
        height: Val::Px(60.0),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    }
}

// Button label text
pub fn pause_button_text(label: &str, asset_server: &AssetServer) -> (Text, TextFont, TextColor) {
    (
        Text::new(label.to_string()),
        TextFont {
            font: asset_server.load("fonts/FiraSans-Bold.ttf").into(),
            font_size: FontSize::Px(28.0),
            ..default()
        },
        TextColor(Color::srgb(0.1, 0.1, 0.1)),
    )
}

// Title text
pub fn pause_title_text(asset_server: &AssetServer) -> (Text, TextFont, TextColor) {
    (
        Text::new("Pause Menu"),
        TextFont {
            font: asset_server.load("fonts/FiraSans-Bold.ttf").into(),
            font_size: FontSize::Px(48.0),
            ..default()
        },
        TextColor(Color::srgb(0.1, 0.1, 0.1)),
    )
}

pub const NORMAL_BUTTON_COLOR: Color = Color::srgb(0.95, 0.95, 0.95);
pub const HOVERED_BUTTON_COLOR: Color = Color::srgb(0.85, 0.85, 0.85);
pub const PRESSED_BUTTON_COLOR: Color = Color::srgb(0.7, 0.7, 0.7);

==> src/main_menu/ui/pause_menu/mod.rs <==
mod components;
mod styles;
mod systems;

use bevy::prelude::*;

use crate::game::SimulationState;

use systems::interactions::*;
use systems::layout::*;

pub struct PauseMenuPlugin;

impl Plugin for PauseMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            // On Enter
            .add_systems(OnEnter(SimulationState::Paused), spawn_pause_menu)
            // Systems
            .add_systems(
                Update,
                (
                    interact_with_resume_button,
                    interact_with_main_button,
                    interact_with_quit_button,
                )
                    .run_if(in_state(SimulationState::Paused)), // <-- in_state()
            )
            // On Exit
            .add_systems(OnExit(SimulationState::Paused), despawn_pause_menu);
    }
}

==> src/main_menu/ui/pause_menu/systems/layout.rs <==
use bevy::prelude::*;

use crate::main_menu::ui::pause_menu::components::*;
use crate::main_menu::ui::pause_menu::styles::*;

pub fn spawn_pause_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let _pause_menu_entity = build_pause_menu(&mut commands, &asset_server);
}

pub fn despawn_pause_menu(
    mut commands: Commands,
    pause_menu_query: Query<Entity, With<PauseMenu>>,
) {
    if let Ok(pause_menu_entity) = pause_menu_query.single() {
        commands.entity(pause_menu_entity).despawn();
    }
}

pub fn build_pause_menu(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    commands
        .spawn((
            pause_menu_style(),
            BackgroundColor(Color::srgb(0.4, 0.4, 0.4).with_alpha(0.8)), // dim overlay
            PauseMenu {},
        ))
        .with_children(|overlay| {
            // Centered white panel
            overlay
                .spawn((
                    pause_panel_style(),
                    BackgroundColor(Color::srgb(0.97, 0.97, 0.97)),
                    BorderColor::all(Color::srgb(0.7, 0.7, 0.7)),
                ))
                .with_children(|panel| {
                    // Title
                    panel.spawn(pause_title_text(asset_server));

                    // Resume
                    panel
                        .spawn((
                            pause_button_style(),
                            BackgroundColor(NORMAL_BUTTON_COLOR),
                            Interaction::None,
                            ResumeButton,
                        ))
                        .with_children(|btn| {
                            btn.spawn(pause_button_text("Resume", asset_server));
                        });

                    // Main Menu
                    panel
                        .spawn((
                            pause_button_style(),
                            BackgroundColor(NORMAL_BUTTON_COLOR),
                            Interaction::None,
                            MainButton,
                        ))
                        .with_children(|btn| {
                            btn.spawn(pause_button_text("Main Menu", asset_server));
                        });

                    // Quit
                    panel
                        .spawn((
                            pause_button_style(),
                            BackgroundColor(NORMAL_BUTTON_COLOR),
                            Interaction::None,
                            QuitButton,
                        ))
                        .with_children(|btn| {
                            btn.spawn(pause_button_text("Quit", asset_server));
                        });
                });
        })
        .id()
}

==> src/main_menu/ui/pause_menu/systems/mod.rs <==
pub mod interactions;
pub mod layout;

==> src/main_menu/ui/pause_menu/systems/interactions.rs <==
use bevy::prelude::*;

use crate::AppState;
use crate::game::SimulationState;
use crate::main_menu::ui::pause_menu::components::*;
use crate::main_menu::ui::pause_menu::styles::{
    HOVERED_BUTTON_COLOR, NORMAL_BUTTON_COLOR, PRESSED_BUTTON_COLOR,
};

pub fn interact_with_resume_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<ResumeButton>),
    >,
    mut next_state: ResMut<NextState<SimulationState>>,
) {
    if let Ok((interaction, mut background_color)) = button_query.single_mut() {
        match *interaction {
            Interaction::Pressed => {
                *background_color = PRESSED_BUTTON_COLOR.into();
                next_state.set(SimulationState::Running);
            }
            Interaction::Hovered => *background_color = HOVERED_BUTTON_COLOR.into(),
            Interaction::None => *background_color = NORMAL_BUTTON_COLOR.into(),
        }
    }
}

pub fn interact_with_main_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<MainButton>),
    >,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if let Ok((interaction, mut background_color)) = button_query.single_mut() {
        match *interaction {
            Interaction::Pressed => {
                *background_color = PRESSED_BUTTON_COLOR.into();
                next_state.set(AppState::MainMenu);
            }
            Interaction::Hovered => *background_color = HOVERED_BUTTON_COLOR.into(),
            Interaction::None => *background_color = NORMAL_BUTTON_COLOR.into(),
        }
    }
}

pub fn interact_with_quit_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<QuitButton>),
    >,
    mut app_exit_event_writer: MessageWriter<AppExit>,
) {
    if let Ok((interaction, mut background_color)) = button_query.single_mut() {
        match *interaction {
            Interaction::Pressed => {
                *background_color = PRESSED_BUTTON_COLOR.into();
                app_exit_event_writer.write(AppExit::Success);
            }
            Interaction::Hovered => *background_color = HOVERED_BUTTON_COLOR.into(),
            Interaction::None => *background_color = NORMAL_BUTTON_COLOR.into(),
        }
    }
}

==> src/main_menu/ui/pause_menu/components.rs <==
use bevy::prelude::*;

#[derive(Component)]
pub struct PauseMenu;

#[derive(Component)]
pub struct ResumeButton;

#[derive(Component)]
pub struct MainButton;

#[derive(Component)]
pub struct QuitButton;

==> src/main_menu/mod.rs <==
mod components;
mod styles;
mod systems;
mod ui;

use crate::AppState;
use bevy::prelude::*;

use crate::main_menu::systems::interactions::*;
use crate::main_menu::ui::GameUIPlugin;
use systems::layout::*;
pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            // Plugins
            .add_plugins(GameUIPlugin)
            // On Enter State
            .add_systems(OnEnter(AppState::MainMenu), spawn_main_menu)
            // Systems
            .add_systems(
                Update,
                (interact_with_play_button, interact_with_quit_button)
                    .run_if(in_state(AppState::MainMenu)),
            )
            // On Exit State
            .add_systems(OnExit(AppState::MainMenu), despawn_main_menu);
    }
}

==> src/main_menu/systems/layout.rs <==
use bevy::prelude::*;

use crate::main_menu::components::*;
use crate::main_menu::styles::*;

pub fn spawn_main_menu(mut commands: Commands<'_, '_>, asset_server: Res<AssetServer>) {
    let _main_menu_entity = build_main_menu(&mut commands, &asset_server);
}

pub fn despawn_main_menu(
    mut commands: Commands<'_, '_>,
    main_menu_query: Query<Entity, With<MainMenu>>,
) {
    if let Ok(main_menu_entity) = main_menu_query.single() {
        commands.entity(main_menu_entity).despawn();
    }
}

pub fn build_main_menu(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    let main_menu_entity = commands
        .spawn((main_menu_style(), MainMenu {}))
        .with_children(|parent| {
            // Title
            parent
                .spawn((title_button_style(),)) // <-- spawn closed here
                .with_children(|title| {
                    // <-- .with_children chained on it
                    // Image 1
                    title.spawn((
                        image_style(),
                        ImageNode::new(asset_server.load("sprites/ball_blue_large.png")),
                    ));

                    // Text
                    title.spawn((Text::new("Ball Game"), text_style(64.0, &asset_server)));

                    // Image 2
                    title.spawn((
                        image_style(),
                        ImageNode::new(asset_server.load("sprites/ball_red_large.png")),
                    ));
                });
            // Play
            parent
                .spawn((
                    button_style(),
                    BackgroundColor(NORMAL_BUTTON_COLOR),
                    Interaction::None,
                    PlayButton {},
                ))
                .with_children(|button| {
                    button.spawn((
                        Text::new("Play"), // text content
                        text_style(32.0, &asset_server),
                    ));
                });
            // Quit
            parent
                .spawn((
                    button_style(),
                    BackgroundColor(NORMAL_BUTTON_COLOR),
                    Interaction::None,
                    QuitButton {},
                ))
                .with_children(|button| {
                    button.spawn((
                        Text::new("Quit"), // text content
                        text_style(32.0, &asset_server),
                    ));
                });
        })
        .id();

    return main_menu_entity;
}

==> src/main_menu/systems/mod.rs <==
pub mod interactions;
pub mod layout;

==> src/main_menu/systems/interactions.rs <==
use bevy::prelude::*;

use crate::AppState;
use crate::main_menu::components::*;
use crate::main_menu::styles::{HOVERED_BUTTON_COLOR, NORMAL_BUTTON_COLOR, PRESSED_BUTTON_COLOR};

pub fn interact_with_play_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<PlayButton>),
    >,
    mut app_state_next_state: ResMut<NextState<AppState>>,
) {
    if let Ok((interaction, mut background_color)) = button_query.single_mut() {
        match *interaction {
            Interaction::Pressed => {
                *background_color = PRESSED_BUTTON_COLOR.into();
                app_state_next_state.set(AppState::Game);
            }
            Interaction::Hovered => {
                *background_color = HOVERED_BUTTON_COLOR.into();
            }
            Interaction::None => {
                *background_color = NORMAL_BUTTON_COLOR.into();
            }
        }
    }
}

pub fn interact_with_quit_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<QuitButton>),
    >,
    mut game_over_event_writer: MessageWriter<AppExit>,
) {
    if let Ok((interaction, mut background_color)) = button_query.single_mut() {
        match *interaction {
            Interaction::Pressed => {
                *background_color = PRESSED_BUTTON_COLOR.into();
                game_over_event_writer.write(AppExit::Success);
            }
            Interaction::Hovered => {
                *background_color = HOVERED_BUTTON_COLOR.into();
            }
            Interaction::None => {
                *background_color = NORMAL_BUTTON_COLOR.into();
            }
        }
    }
}

==> src/main_menu/components.rs <==
use bevy::prelude::*;

#[derive(Component)]
pub struct MainMenu {}

#[derive(Component)]
pub struct PlayButton {}

#[derive(Component)]
pub struct QuitButton {}

==> src/resources.rs <==
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

==> src/main.rs <==
pub mod events;
mod game;
mod main_menu;
mod systems;

use game::GamePlugin;
use main_menu::MainMenuPlugin;

use systems::*;

use bevy::prelude::*;

fn main() {
    App::new()
        // Bevy Plugins
        .add_plugins(DefaultPlugins)
        // State
        .init_state::<AppState>()
        // My Plugins
        .add_plugins(MainMenuPlugin)
        .add_plugins(GamePlugin)
        // Startup system
        .add_systems(Startup, spawn_camera)
        // Systems
        .add_systems(Update, transition_to_game_state)
        .add_systems(Update, transition_to_main_menu_state)
        .add_systems(Update, exit_game)
        .add_systems(Update, handle_game_over)
        .run();
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    Game,
    GameOver,
}
