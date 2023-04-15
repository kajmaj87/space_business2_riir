mod ai;
pub mod components;
mod measures;
mod people;
mod planet;

pub use self::measures::{GeometryType, RealCoords, VirtualCoords};

use bevy::prelude::*;

pub struct LogicPlugin;

impl Plugin for LogicPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(self::people::PeoplePlugin)
            .add_plugin(self::ai::AiPlugin)
            .insert_resource(planet::Time(0))
            .add_system(self::planet::food_growth)
            .add_system(self::planet::time_system);
    }
}
