use crate::config::Config;
use crate::debug::components::Performance;
use crate::logic::people::mark_entity_as_dead;
use bevy::prelude::*;
use big_brain::prelude::*;
use big_brain::BigBrainPlugin;
use macros::measured;
use rand::{thread_rng, Rng};
use std::cmp::max;

use super::components::Dead;
use super::components::{FoodAmount, Hunger, Person};
use super::people::Forage;

#[derive(Clone, Component, Debug, ScorerBuilder)]
struct Hungry;

#[derive(Clone, Component, Debug, ActionBuilder)]
struct Eat;

#[derive(Clone, Component, Debug, ScorerBuilder)]
struct MoveNeed;

#[derive(Clone, Component, Debug, ActionBuilder)]
struct MoveAction;

pub struct AiPlugin;

impl Plugin for AiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(BigBrainPlugin)
            .add_system(eat_action_system.in_set(BigBrainSet::Actions))
            .add_system(hungry_scorer_system.in_set(BigBrainSet::Scorers))
            .add_system(move_action_system.in_set(BigBrainSet::Actions))
            .add_system(move_scorer_system.in_set(BigBrainSet::Scorers))
            .add_system(init_brains);
    }
}

#[allow(clippy::type_complexity)]
pub fn init_brains(
    mut commands: Commands,
    query: Query<Entity, (With<Person>, Without<ThinkerBuilder>, Without<Dead>)>,
) {
    for entity in query.iter() {
        info!("Adding a thinker @{}", entity.index());
        commands.entity(entity).insert(
            Thinker::build()
                .picker(FirstToScore { threshold: 0.8 })
                .when(Hungry, Eat)
                .when(MoveNeed, MoveAction),
        );
    }
}

#[measured]
fn move_action_system(
    mut commands: Commands,
    mut query: Query<(&Actor, &mut ActionState, &MoveAction)>,
) {
    let mut random = thread_rng();
    for (Actor(actor), state, _move) in query.iter_mut() {
        just_execute(state, || {
            // randomize dx, dy as -1, 0, 1 (no diagonal movement)
            let (dx, dy);
            if random.gen_range(0..=1) == 0 {
                // horizontal move
                dx = random.gen_range(-1..=1);
                dy = 0;
            } else {
                // vertical move
                dx = 0;
                dy = random.gen_range(-1..=1);
            }
            commands
                .entity(*actor)
                .insert(super::components::Move { dx, dy })
                .insert(Forage);
        })
    }
}

#[measured]
fn move_scorer_system(
    food_amount: Query<&FoodAmount>,
    mut query: Query<(&Actor, &mut Score), With<MoveNeed>>,
    config: Res<Config>,
) {
    for (Actor(actor), mut score) in query.iter_mut() {
        if let Ok(food) = food_amount.get(*actor) {
            let food_goal = config.ai.food_amount_goal.value;
            let food_threshold = config.ai.food_amount_threshold.value;
            let s = clamp(
                (max(
                    food_goal as i32 - food.apples as i32,
                    food_goal as i32 - food.oranges as i32,
                )) as f32
                    / food_goal as f32
                    + food_threshold,
            );
            score.set(s);
        }
    }
}

#[measured]
fn eat_action_system(
    mut commands: Commands,
    mut hungers: Query<(&mut Hunger, &mut FoodAmount)>,
    mut query: Query<(&Actor, &mut ActionState, &Eat)>,
    config: Res<Config>,
) {
    for (Actor(actor), state, _eat) in query.iter_mut() {
        if let Ok((mut hunger, mut food)) = hungers.get_mut(*actor) {
            just_execute(state, || {
                if hunger.apple > 1.0 && food.apples > 0 {
                    let old_hunger = hunger.apple;
                    hunger.apple -= config.game.hunger_decrease.value;
                    food.apples -= 1;
                    debug!(
                        "Person ate something, food left: {}, hunger for apples was: {}, hunger for apples is: {}",
                        food.apples + food.oranges,
                        old_hunger,
                        hunger.apple
                    );
                } else if hunger.orange > 1.0 && food.oranges > 0 {
                    let old_hunger = hunger.orange;
                    hunger.orange -= config.game.hunger_decrease.value;
                    food.oranges -= 1;
                    debug!(
                        "Person ate something, food left: {}, hunger for oranges was: {}, hunger for oranges is: {}",
                        food.apples + food.oranges,
                        old_hunger,
                        hunger.orange
                    );
                } else {
                    mark_entity_as_dead(*actor, &mut commands, &config);
                    if hunger.orange > 1.0 {
                        info!("Person {} has died of orange hunger", actor.index());
                    } else if hunger.apple > 1.0 {
                        info!("Person {} has died of apple hunger", actor.index());
                    } else {
                        warn!("Person {} has died of unknown reason", actor.index());
                    }
                }
            });
        }
    }
}

#[measured]
fn hungry_scorer_system(
    hungers: Query<&Hunger>,
    mut query: Query<(&Actor, &mut Score), With<Hungry>>,
) {
    for (Actor(actor), mut score) in query.iter_mut() {
        if let Ok(hunger) = hungers.get(*actor) {
            // eat only if hunger is above 1.0, if nothihg to eat entity will die
            let s = if hunger.apple > 1.0 || hunger.orange > 1.0 {
                1.0
            } else {
                0.0
            };
            score.set(s * s);
        }
    }
}

fn just_execute(mut state: Mut<ActionState>, f: impl FnOnce()) {
    match *state {
        ActionState::Requested => {
            *state = ActionState::Executing;
        }
        ActionState::Executing => {
            f();
            *state = ActionState::Success;
        }
        ActionState::Cancelled => {
            *state = ActionState::Failure;
        }
        _ => {}
    }
}

fn clamp(x: f32) -> f32 {
    let s = if x < 1.0 { x } else { 1.0 };
    if s > 0.0 {
        s
    } else {
        0.0
    }
}
