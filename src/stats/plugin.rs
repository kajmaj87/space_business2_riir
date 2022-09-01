use bevy::prelude::{App, Plugin};

use crate::stats;

pub struct StatsPlugin;

impl Plugin for StatsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(stats::economy::Statistics {
            food_history: vec![],
        })
        .add_system(stats::economy::food_statistics);
    }
}
