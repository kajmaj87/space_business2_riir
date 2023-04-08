pub mod components;
mod economy;
mod ui;

use bevy::prelude::{App, Plugin};

pub struct StatsPlugin;

impl Plugin for StatsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(economy::Statistics {
            food_history: vec![],
            people_history: vec![],
            current_food: 0,
            current_people: 0,
        })
        .add_system(economy::food_statistics)
        .add_system(ui::stats_window);
    }
}
