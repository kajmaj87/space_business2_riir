mod ai;
pub mod components;
mod interactions;
pub(crate) mod invariants;
mod measures;
mod people;
mod planet;

pub use self::measures::{GeometryType, RealCoords, VirtualCoords};

use bevy::prelude::*;

pub struct LogicPlugin;

impl Plugin for LogicPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(people::PeoplePlugin)
            .add_plugin(ai::AiPlugin)
            .insert_resource(planet::Time(0))
            .add_system(planet::time_system)
            .add_system(planet::food_growth)
            .add_system(interactions::add_interaction_system.in_base_set(CoreSet::First))
            .add_system(interactions::breeding_interaction_system)
            .add_system(interactions::cleanup_interactions_system.in_base_set(CoreSet::PostUpdate));
    }
}
