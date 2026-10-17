use bevy::app::AppExit;
use bevy::{prelude::*, window::PrimaryWindow};

use crate::events::*;

// spawning resources

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.single().unwrap();

    commands.spawn((
        Camera2d { ..default() },
        Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
    ));
}

// system functions

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
