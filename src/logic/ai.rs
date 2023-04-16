use crate::config::Config;
use crate::debug::components::Performance;
use crate::logic::components::{FoodSource, Lookup};
use crate::logic::measures::VirtualCoords;
use crate::logic::people::{mark_entity_as_dead, Information, Knowledge, MoveTo};
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
pub struct Eat;

#[derive(Clone, Component, Debug, ScorerBuilder)]
struct MoveNeed;

#[derive(Clone, Component, Debug, ActionBuilder)]
struct MoveAction;

#[derive(Clone, Component, Debug, ScorerBuilder)]
struct MissingInfo;

#[derive(Clone, Component, Debug, ActionBuilder)]
struct LookAround;

pub struct AiPlugin;

impl Plugin for AiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(BigBrainPlugin)
            .add_system(eat_action_system.in_set(BigBrainSet::Actions))
            .add_system(hungry_scorer_system.in_set(BigBrainSet::Scorers))
            .add_system(move_action_system.in_set(BigBrainSet::Actions))
            .add_system(move_scorer_system.in_set(BigBrainSet::Scorers))
            .add_system(look_around_action_system.in_set(BigBrainSet::Actions))
            .add_system(missing_info_scorer_system.in_set(BigBrainSet::Scorers))
            .add_system(brain_wash)
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
                // .when(MissingInfo, LookAround)
                .when(MoveNeed, MoveAction),
        );
    }
}

#[measured]
fn brain_wash(mut query: Query<(Entity, &mut Knowledge)>) {
    for (entity, mut knowledge) in query.iter_mut() {
        if !knowledge.infos.is_empty() {
            info!("{} is brain washed", entity.index());
            // remove half of random elements from knowledge
            let mut rng = thread_rng();
            let mut to_remove = knowledge.infos.len() / 2;
            while to_remove > 0 {
                let index = rng.gen_range(0..knowledge.infos.len());
                knowledge.infos.remove(index);
                to_remove -= 1;
            }
        }
    }
}

#[measured]
fn missing_info_scorer_system(
    mut query: Query<(&Actor, &mut Score), With<MissingInfo>>,
    info: Query<&Knowledge>,
) {
    for (Actor(actor), mut score) in query.iter_mut() {
        if let Ok(knowledge) = info.get(*actor) {
            if knowledge.infos.is_empty() {
                score.set(1.0);
                warn!("{} has no info", actor.index());
            } else {
                score.set(0.0);
            }
        } else {
            score.set(1.0);
        }
    }
}

#[measured]
fn look_around_action_system(
    config: Res<Config>,
    food_lookup: Res<Lookup<FoodSource>>,
    mut query: Query<(&Actor, &mut ActionState), With<LookAround>>,
    people: Query<&VirtualCoords>,
    mut commands: Commands,
) {
    for (Actor(actor), state) in query.iter_mut() {
        just_execute(state, || {
            if let Ok(coords) = people.get(*actor) {
                let food = find_food(&food_lookup, &config, coords, config.ai.vision_range.value);
                warn!("{} found {} food sources", actor.index(), food.len());
                commands.entity(*actor).insert(Knowledge { infos: food });
            }
        })
    }
}

fn find_food(
    food_lookup: &Res<Lookup<FoodSource>>,
    config: &Config,
    origin: &VirtualCoords,
    vision_range: u32,
) -> Vec<Information> {
    let mut result = Vec::new();
    // gather GridCoords in a vector using coords, looking up, down, left, right upto vision_range tiles
    let mut coords_to_check = Vec::new();
    for x in origin.x - vision_range as i32..=origin.x + vision_range as i32 {
        coords_to_check.push(VirtualCoords { x, y: origin.y });
    }
    for y in origin.y - vision_range as i32..=origin.y + vision_range as i32 {
        coords_to_check.push(VirtualCoords { x: origin.x, y });
    }
    for coords in coords_to_check {
        if let Some(food) = food_lookup.entities.get(&coords.to_real(config)) {
            result.push(Information {
                entity: *food,
                coords,
            });
        }
    }
    result
}

#[measured]
fn move_action_system(
    mut commands: Commands,
    // knowledge: Query<&Knowledge>,
    food_lookup: Res<Lookup<FoodSource>>,
    food: Query<&FoodAmount, With<FoodSource>>,
    person: Query<(&FoodAmount, &VirtualCoords), With<Person>>,
    config: Res<Config>,
    mut query: Query<(&Actor, &mut ActionState, &MoveAction)>,
) {
    let mut random = thread_rng();
    warn!("Move action system");
    for (Actor(actor), state, _) in query.iter_mut() {
        // warn!("{} is moving, state is {:?}", actor.index(), state);
        just_execute(state, || {
            let destination = if let Ok((person_food, coords)) = person.get(*actor) {
                let mut best = None;
                let mut best_score = 0.0;
                for info in find_food(&food_lookup, &config, coords, config.ai.vision_range.value) {
                    if let Ok(food_amount) = food.get(info.entity) {
                        let move_vector = VirtualCoords {
                            x: info.coords.x - coords.x,
                            y: info.coords.y - coords.y,
                        };
                        let cost = (move_vector.x.abs() + move_vector.y.abs()) as f32
                            * config.game.hunger_increase.value;
                        let max_food_of_type = config.ai.food_amount_goal.value / 2;
                        let apple_preference = if person_food.apples < max_food_of_type {
                            1.0 - person_food.apples as f32
                                / (1.0 + person_food.oranges as f32 + person_food.apples as f32)
                        } else {
                            0.0
                        };
                        let orange_preference = if person_food.oranges < max_food_of_type {
                            1.0 - apple_preference
                        } else {
                            0.0
                        };
                        let score = apple_preference * food_amount.apples as f32
                            + orange_preference * food_amount.oranges as f32
                            - cost;
                        if score > best_score {
                            best_score = score;
                            best = Some(info.coords);
                        }
                    }
                    warn!("person {} had no knowledge", actor.index());
                }
                warn!("{:?} has best score of {}", best, best_score);
                best
            } else {
                warn!("{} is not a person", actor.index());
                None
            };

            let destination = if let Some(destination) = destination {
                warn!(
                    "{} is moving to best position found {:?}",
                    actor.index(),
                    destination
                );
                destination
            } else {
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
                warn!("{} is moving randomly by ({}, {})", actor.index(), dx, dy);
                if let Ok((_, coords)) = person.get(*actor) {
                    warn!("{} is a person", actor.index());
                    VirtualCoords {
                        x: coords.x + dx,
                        y: coords.y + dy,
                    }
                } else {
                    panic!("{} is not a person", actor.index());
                }
            };
            commands
                .entity(*actor)
                .insert(MoveTo { dest: destination })
                // todo this should be first added after the move is ended
                .insert(Forage);
        })
    }
}

#[measured]
fn move_scorer_system(
    food_amount: Query<&FoodAmount>,
    mut query: Query<(&Actor, &mut Score), With<MoveNeed>>,
    already_moving: Query<(&Actor, &MoveTo)>,
    config: Res<Config>,
) {
    for (Actor(actor), mut score) in query.iter_mut() {
        if already_moving.get(*actor).is_ok() {
            warn!("{} is already moving", actor.index());
            score.set(0.0);
        } else if let Ok(food) = food_amount.get(*actor) {
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
            warn!("{} has score of {} for moving", actor.index(), s);
            score.set(s);
        }
    }
}

#[measured]
pub fn eat_action_system(
    mut commands: Commands,
    mut hungers: Query<(&mut Hunger, &mut FoodAmount)>,
    mut query: Query<(&Actor, &mut ActionState, &Eat)>,
    config: Res<Config>,
) {
    for (Actor(actor), state, _eat) in query.iter_mut() {
        if let Ok((mut hunger, mut food)) = hungers.get_mut(*actor) {
            warn!("{} is eating", actor.index());
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
                        warn!("Person {} has died of orange hunger", actor.index());
                    } else if hunger.apple > 1.0 {
                        warn!("Person {} has died of apple hunger", actor.index());
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
            f();
            *state = ActionState::Success;
        }
        ActionState::Executing => {
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
