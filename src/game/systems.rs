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
