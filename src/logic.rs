mod ai;
pub mod components;
mod people;
mod planet;

use bevy::prelude::*;

pub struct LogicPlugin;

impl Plugin for LogicPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(self::people::PeoplePlugin)
            .add_plugin(self::ai::AiPlugin)
            .add_system(self::planet::food_growth);
    }
}
