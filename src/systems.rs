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
        match app_state.get() {
            // a player must enter a name before starting a new game
            AppState::MainMenu => {
                next_app_state.set(AppState::NameEntry);
                debug!("Entered AppState::NameEntry");
            }
            // restart: the player already has a name
            AppState::GameOver => {
                next_app_state.set(AppState::Game);
                debug!("Entered AppState::Game");
            }
            _ => {}
        }
    }
}

pub fn transition_to_main_menu_state(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    app_state: Res<State<AppState>>,
    mut next_app_state: ResMut<NextState<AppState>>,
    mut next_simulation_state: ResMut<NextState<SimulationState>>,
) {
    // while typing a name the 'm' key is just a letter
    if app_state.get() == &AppState::NameEntry {
        return;
    }

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
    app_state: Res<State<AppState>>,
    mut next_app_state: ResMut<NextState<AppState>>,
    mut app_exit_event_writer: MessageWriter<AppExit>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        match app_state.get() {
            // Escape on these screens means 'back'
            AppState::NameEntry | AppState::Scoreboard => {
                next_app_state.set(AppState::MainMenu);
            }
            _ => {
                app_exit_event_writer.write(AppExit::Success);
            }
        }
    }
}

pub fn handle_game_over(
    mut game_over_event_reader: MessageReader<GameOver>,
    mut next_app_state: ResMut<NextState<AppState>>,
) {
    for event in game_over_event_reader.read() {
        debug!(
            "Game Over! Final Score: {}, Level: {}",
            event.score, event.level
        );

        next_app_state.set(AppState::GameOver);
    }
}
