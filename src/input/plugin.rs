use bevy::prelude::{App, Plugin};

use crate::input;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(input::map::movement);
    }
}
