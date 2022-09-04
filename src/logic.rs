pub mod components;
mod people;
mod planet;

use bevy::prelude::{App, Plugin};

use crate::logic;

pub struct LogicPlugin;

impl Plugin for LogicPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(logic::people::PeoplePlugin)
            .add_system(logic::planet::food_growth);
    }
}
