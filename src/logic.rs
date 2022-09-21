mod ai;
pub mod components;
mod people;
mod planet;

use bevy::prelude::*;
use iyes_loopless::prelude::*;

pub struct LogicPlugin;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    ProcessLogic,
    WaitForInput,
}

fn turn_end_system(mut game_state: ResMut<State<GameState>>) {
    info!("Going back to WaitForInput state");
    game_state.set(GameState::WaitForInput).unwrap();
}

impl Plugin for LogicPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(self::people::PeoplePlugin)
            .add_plugin(self::ai::AiPlugin)
            .add_system_set(
                SystemSet::on_update(GameState::ProcessLogic)
                    .with_system(self::planet::food_growth),
            )
            .add_system_to_stage(
                CoreStage::PostUpdate,
                turn_end_system.run_not_in_bevy_state(GameState::WaitForInput),
            );
    }
}
