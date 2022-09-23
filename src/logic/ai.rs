use ::function_name::named;
use bevy::prelude::*;
use big_brain::prelude::*;
use big_brain::BigBrainPlugin;
use iyes_loopless::prelude::*;
use rand::{thread_rng, Rng};

use crate::config::Config;

use super::components::Dead;
use super::components::{FoodAmount, Person};
use super::people::Forage;
use super::GameState;

#[derive(Clone, Component, Debug)]
struct MoveNeed;
#[derive(Clone, Component, Debug)]
struct MoveAction;

pub struct AiPlugin;

impl Plugin for AiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(BigBrainPlugin)
            .add_system_set_to_stage(
                BigBrainStage::Actions,
                ConditionSet::new()
                    .run_in_bevy_state(GameState::ProcessLogic)
                    .with_system(move_action_system)
                    .into(),
            )
            .add_system_set_to_stage(
                BigBrainStage::Scorers,
                ConditionSet::new()
                    .run_in_bevy_state(GameState::ProcessLogic)
                    .with_system(move_scorer_system)
                    .into(),
            )
            .add_system(init_brains);
    }
}

#[allow(clippy::type_complexity)]
fn init_brains(
    mut commands: Commands,
    query: Query<Entity, (With<Person>, Without<ThinkerBuilder>, Without<Dead>)>,
) {
    for entity in query.iter() {
        info!("Adding a thinker @{}", entity.id());
        commands.entity(entity).insert(
            Thinker::build()
                .picker(FirstToScore { threshold: 0.8 })
                .when(MoveNeed, MoveAction),
        );
    }
}

#[named]
fn move_action_system(
    mut commands: Commands,
    mut query: Query<(&Actor, &mut ActionState, &MoveAction)>,
) {
    info!("Running {} system", function_name!());
    let mut random = thread_rng();
    for (Actor(actor), state, _move) in query.iter_mut() {
        just_execute(state, || {
            info!("Moving actor @{}", actor.id());
            let dx = random.gen_range(-1..=1) as f32;
            let dy = random.gen_range(-1..=1) as f32;
            commands
                .entity(*actor)
                .insert(super::components::Move { dx, dy })
                .insert(Forage);
        })
    }
}

#[named]
fn move_scorer_system(
    food_amount: Query<&FoodAmount>,
    mut query: Query<(&Actor, &mut Score), With<MoveNeed>>,
    config: Res<Config>,
) {
    info!("Running {} system", function_name!());
    for (Actor(actor), mut score) in query.iter_mut() {
        if let Ok(food) = food_amount.get(*actor) {
            let food_goal = config.ai.food_amount_goal.value as f32;
            let food_threshold = config.ai.food_amount_threshold.value as f32;
            let s = clamp((food_goal - food.0 as f32) / food_goal + food_threshold);
            info!("Move score for actor @{} is {}", actor.id(), s);
            score.set(s);
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
