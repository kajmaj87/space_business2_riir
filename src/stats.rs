pub mod components;
mod economy;
pub mod ui;

use bevy::prelude::{App, Plugin};

pub struct StatsPlugin;

impl Plugin for StatsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(economy::Statistics {
            apple_history_sources: vec![],
            orange_history_sources: vec![],
            apple_history_people: vec![],
            orange_history_people: vec![],
            people_history: vec![],
            trade_history: vec![],
            current_food: 0,
            current_apples: 0,
            current_oranges: 0,
            current_people: 0,
        })
        .add_system(economy::food_statistics)
        .add_system(ui::stats_window);
    }
}
