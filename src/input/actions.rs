use bevy::prelude::*;

use crate::logic::GameState;

pub fn game_state(
    mut keyboard_input: ResMut<Input<KeyCode>>,
    mut game_state: ResMut<State<GameState>>,
) {
    if keyboard_input.clear_just_pressed(KeyCode::Return) {
        game_state.set(GameState::ProcessLogic).unwrap();
    }
}
