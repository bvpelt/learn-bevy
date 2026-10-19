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
    mut commands: Commands<'_, '_>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    app_state: Res<State<AppState>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyG) {
        if app_state.get() != &AppState::Game {
            commands.insert_resource(NextState::Pending(AppState::Game));
            println!("Entered AppState::Game");
        }
    }
}

pub fn transition_to_main_menu_state(
    mut commands: Commands<'_, '_>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    app_state: Res<State<AppState>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyM) {
        if app_state.get() != &AppState::MainMenu {
            commands.insert_resource(NextState::Pending(AppState::MainMenu));
            commands.insert_resource(NextState::Pending(SimulationState::Paused));
            println!("Entered AppState::MainMenu");
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
    mut commands: Commands,
    mut game_over_event_reader: MessageReader<GameOver>,
) {
    for event in game_over_event_reader.read() {
        println!("Game Over! Final Score: {}", event.score.to_string());
        commands.insert_resource(NextState::Pending(AppState::GameOver));
    }
}
