use bevy::prelude::*;
use big_brain::prelude::*;
use big_brain::BigBrainPlugin;
use rand::{thread_rng, Rng};

use crate::config::Config;

use super::components::{FoodAmount, Hunger, Person};
use super::people::init_people;

#[derive(Clone, Component, Debug)]
struct Hungry;
#[derive(Clone, Component, Debug)]
struct Eat;
#[derive(Clone, Component, Debug)]
struct MoveNeed;
#[derive(Clone, Component, Debug)]
struct MoveAction;

pub struct AiPlugin;

impl Plugin for AiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(BigBrainPlugin)
            // no brain without a body
            .add_startup_system_to_stage(StartupStage::PostStartup, init_brains.after(init_people))
            .add_system_to_stage(BigBrainStage::Actions, eat_action_system)
            .add_system_to_stage(BigBrainStage::Scorers, hungry_scorer_system)
            .add_system_to_stage(BigBrainStage::Actions, move_action_system)
            .add_system_to_stage(BigBrainStage::Scorers, move_scorer_system);
    }
}

pub fn init_brains(mut commands: Commands, query: Query<(Entity, &Person)>) {
    info!("Brains initialized");
    for (entity, _) in query.iter() {
        debug!("Adding a thinker @{}", entity.id());
        commands.entity(entity).insert(
            Thinker::build()
                .picker(FirstToScore { threshold: 0.8 })
                .when(Hungry, Eat)
                .when(MoveNeed, MoveAction),
        );
    }
}

fn move_action_system(
    mut commands: Commands,
    mut query: Query<(&Actor, &mut ActionState, &MoveAction)>,
) {
    let mut random = thread_rng();
    for (Actor(actor), state, _move) in query.iter_mut() {
        just_execute(state, || {
            let dx = random.gen_range(-1..=1) as f32;
            let dy = random.gen_range(-1..=1) as f32;
            commands
                .entity(*actor)
                .insert(super::components::Move { dx, dy });
        })
    }
}

fn move_scorer_system(
    food_amount: Query<&FoodAmount>,
    mut query: Query<(&Actor, &mut Score), With<MoveNeed>>,
) {
    for (Actor(actor), mut score) in query.iter_mut() {
        if let Ok(food) = food_amount.get(*actor) {
            let s = clamp((5 - food.0) as f32 / 3.0 - 0.2);
            score.set(s);
        }
    }
}

fn eat_action_system(
    mut hungers: Query<(&mut Hunger, &mut FoodAmount)>,
    mut query: Query<(&Actor, &mut ActionState, &Eat)>,
    config: Res<Config>,
) {
    for (Actor(actor), state, _eat) in query.iter_mut() {
        if let Ok((mut hunger, mut food)) = hungers.get_mut(*actor) {
            just_execute(state, || {
                if food.0 > 0 {
                    let old_hunger = hunger.0;
                    hunger.0 -= config.game.hunger_decrease.value;
                    food.0 -= 1;
                    info!(
                        "Person ate something, food left: {}, hunger was: {}, hunger is: {}",
                        food.0, old_hunger, hunger.0
                    );
                }
            })
        }
    }
}

fn hungry_scorer_system(
    hungers: Query<&Hunger>,
    mut query: Query<(&Actor, &mut Score), With<Hungry>>,
) {
    for (Actor(actor), mut score) in query.iter_mut() {
        if let Ok(hunger) = hungers.get(*actor) {
            // The score here must be between 0.0 and 1.0.
            let s = clamp(hunger.0);
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
