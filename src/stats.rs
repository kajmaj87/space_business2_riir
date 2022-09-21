pub mod components;
mod economy;

use bevy::prelude::{App, CoreStage, Plugin};
use iyes_loopless::prelude::*;

use crate::{logic::GameState, stats};

pub struct StatsPlugin;

impl Plugin for StatsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(stats::economy::Statistics {
            food_history: vec![],
            people_history: vec![],
        })
        .add_system_to_stage(
            CoreStage::PostUpdate,
            stats::economy::food_statistics.run_in_bevy_state(GameState::ProcessLogic),
        );
    }
}
