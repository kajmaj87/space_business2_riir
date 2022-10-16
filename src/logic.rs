mod ai;
pub mod components;
mod economy;
mod people;
mod planet;

use bevy::prelude::*;
use iyes_loopless::prelude::*;

pub struct LogicPlugin;

#[derive(Clone, Eq, PartialEq, Debug, Hash, Copy)]
pub enum TurnStep {
    Process,
    Animate(u32), //time in microseconds, cannot use f32 because its not Hash
    WaitForInput,
}
#[derive(Clone, Eq, PartialEq, Debug, Hash, Copy)]
pub enum TurnPhase {
    PreparePlanet,
    GenerateJobs,
    TakeJobs,
    GotoMarket,
    Eat,
    DegradeStuff,
}

pub type GameState = (TurnPhase, TurnStep);

impl Plugin for LogicPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(self::people::PeoplePlugin)
            .add_plugin(self::ai::AiPlugin)
            .add_system_set(
                SystemSet::on_update((TurnPhase::GenerateJobs, TurnStep::Process))
                    .with_system(self::economy::generate_jobs),
            )
            .add_system_set(
                SystemSet::on_update((TurnPhase::PreparePlanet, TurnStep::Process))
                    .with_system(self::planet::food_growth),
            );
    }
}
