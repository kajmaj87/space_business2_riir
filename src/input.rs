mod actions;
mod map;

use bevy::prelude::*;

use crate::{input, logic::GameState};

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(input::map::movement).add_system_set(
            SystemSet::on_update(GameState::WaitForInput).with_system(input::actions::game_state),
        );
    }
}
