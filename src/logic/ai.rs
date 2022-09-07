use bevy::prelude::*;
use big_brain::prelude::*;
use big_brain::BigBrainPlugin;

use crate::config::Config;

use super::components::{FoodAmount, Hunger, Person};
use super::people::init_people;

#[derive(Clone, Component, Debug)]
pub struct Hungry;
#[derive(Clone, Component, Debug)]
pub struct Eat;

pub struct AiPlugin;

impl Plugin for AiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(BigBrainPlugin)
            // no brain without a body
            .add_startup_system_to_stage(StartupStage::PostStartup, init_brains.after(init_people))
            .add_system_to_stage(BigBrainStage::Actions, eat_action_system)
            .add_system_to_stage(BigBrainStage::Scorers, hungry_scorer_system);
    }
}

pub fn init_brains(mut commands: Commands, query: Query<(Entity, &Person)>) {
    info!("Brains initialized");
    for (entity, _) in query.iter() {
        debug!("Adding a thinker @{}", entity.id());
        commands.entity(entity).insert(
            Thinker::build()
                .picker(FirstToScore { threshold: 0.8 })
                .when(Hungry, Eat),
        );
    }
}

fn eat_action_system(
    mut hungers: Query<(&mut Hunger, &mut FoodAmount)>,
    mut query: Query<(&Actor, &mut ActionState, &Eat)>,
    config: Res<Config>,
) {
    for (Actor(actor), mut state, _eat) in query.iter_mut() {
        if let Ok((mut hunger, mut food)) = hungers.get_mut(*actor) {
            match *state {
                ActionState::Requested => {
                    *state = ActionState::Executing;
                }
                ActionState::Executing => {
                    if food.0 > 0 {
                        let old_hunger = hunger.0;
                        hunger.0 -= config.game.hunger_decrease.value;
                        food.0 -= 1;
                        info!(
                            "Person ate something, food left: {}, hunger was: {}, hunger is: {}",
                            food.0, old_hunger, hunger.0
                        );
                    }
                    *state = ActionState::Success;
                }
                ActionState::Cancelled => {
                    *state = ActionState::Failure;
                }
                _ => {}
            }
        }
    }
}

pub fn hungry_scorer_system(
    hungers: Query<&Hunger>,
    mut query: Query<(&Actor, &mut Score), With<Hungry>>,
) {
    for (Actor(actor), mut score) in query.iter_mut() {
        if let Ok(hunger) = hungers.get(*actor) {
            // The score here must be between 0.0 and 1.0.
            let s = if hunger.0 < 1.0 { hunger.0 } else { 1.0 };
            let s = if s > 0.0 { s } else { 0.0 };
            score.set(s * s);
        }
    }
}
