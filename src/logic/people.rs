use ::function_name::named;
use bevy::prelude::*;
use big_brain::thinker::ThinkerBuilder;
use iyes_loopless::prelude::*;

use crate::config::Config;

use super::{
    components::{FoodSource, Name, Ttl},
    planet::FoodAmount,
    TurnPhase, TurnStep,
};

#[derive(Component)]
pub struct Hunger(pub f32);

#[derive(Component)]
pub struct Person;

#[derive(Component)]
pub struct Age(pub u32);

#[derive(Component)]
pub struct Dead;

#[derive(Component)]
pub struct Move {
    pub x: f32,
    pub y: f32,
}

#[derive(Component)]
pub struct Forage;

// Position and GridPostion are already defined in bevy::prelude
#[derive(Component, PartialEq)]
pub struct GridCoords {
    pub x: f32,
    pub y: f32,
}

#[derive(Bundle)]
struct PersonBundle {
    name: Name,
    type_marker: Person,
    age: Age,
    hunger: Hunger,
    food: FoodAmount,
    position: GridCoords,
}

impl Default for PersonBundle {
    fn default() -> Self {
        PersonBundle {
            name: Name(String::from("Test guy")),
            type_marker: Person,
            age: Age(0),
            hunger: Hunger(0.0),
            food: FoodAmount(3),
            position: GridCoords { x: 5.0, y: 3.0 },
        }
    }
}

pub struct PeoplePlugin;

impl Plugin for PeoplePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(init_people)
            .add_system(
                move_system
                    .run_in_bevy_state((TurnPhase::PreparePlanet, TurnStep::Process))
                    .label("movement"),
            )
            .add_system_set(
                ConditionSet::new()
                    .run_in_bevy_state((TurnPhase::PreparePlanet, TurnStep::Process))
                    .after("movement")
                    .with_system(foraging_system)
                    .with_system(breeding_system)
                    .with_system(aging_system)
                    .into(),
            )
            // we need to despawn enities separately so that no commands use them in wrong moment
            .add_system_to_stage(CoreStage::PostUpdate, cleanup_system);
    }
}

pub fn init_people(mut commands: Commands, config: Res<Config>) {
    info!("People initialized");
    for _ in 0..config.game.starting_people.value {
        commands.spawn_bundle(PersonBundle::default());
    }
}

#[named]
fn aging_system(
    mut commands: Commands,
    mut query: Query<(Entity, &Person, &mut Age), Without<Dead>>,
    config: Res<Config>,
) {
    info!("Running {}", function_name!());
    for (person, _, mut age) in query.iter_mut() {
        age.0 += 1;
        if age.0 > config.game.max_person_age.value && config.game.max_person_age.value > 0 {
            mark_entity_as_dead(person, &mut commands, &config);
            info!(
                "Person {} died of old age being {} turns old",
                person.id(),
                age.0
            );
        }
    }
}

fn mark_entity_as_dead(person: Entity, commands: &mut Commands, config: &Res<Config>) {
    commands
        .entity(person)
        .insert(Dead)
        .insert(Ttl(config.game.person_ttl.value))
        .remove::<ThinkerBuilder>();
}

#[named]
fn move_system(mut commands: Commands, mut query: Query<(Entity, &Move, &mut GridCoords)>) {
    info!("Running {}", function_name!());
    for (person, move_component, mut coords) in query.iter_mut() {
        commands.entity(person).remove::<Move>();
        coords.x = move_component.x;
        coords.y = move_component.y;
    }
}

#[allow(clippy::type_complexity)]
#[named]
fn foraging_system(
    mut commands: Commands,
    mut people: Query<
        (Entity, &mut FoodAmount, &GridCoords),
        (Changed<Forage>, With<Person>, With<Forage>),
    >,
    mut food_producers: Query<(&mut FoodAmount, &GridCoords), (With<FoodSource>, Without<Person>)>,
) {
    info!("Running {}", function_name!());
    for (person, mut person_food_amount, coords) in people.iter_mut() {
        for (mut food_producer_amount, food_coords) in food_producers.iter_mut() {
            if coords == food_coords && food_producer_amount.0 > 0 {
                debug!("Found some food!");
                person_food_amount.0 += 1;
                food_producer_amount.0 -= 1;
                commands.entity(person).remove::<Forage>();
            }
        }
    }
}

#[named]
fn breeding_system(
    mut commands: Commands,
    mut people: Query<(&mut FoodAmount, &GridCoords), With<Person>>,
    config: Res<Config>,
) {
    info!("Running {}", function_name!());
    for (mut person_food_amount, coords) in people.iter_mut() {
        if person_food_amount.0 > 2 * config.game.food_for_baby.value {
            info!("I'm having a baby! My food is: {}", person_food_amount.0);
            person_food_amount.0 -= config.game.food_for_baby.value;
            commands.spawn_bundle(PersonBundle {
                food: FoodAmount(config.game.food_for_baby.value),
                position: GridCoords {
                    x: coords.x,
                    y: coords.y,
                },
                ..Default::default()
            });
        }
    }
}

fn cleanup_system(mut commands: Commands, mut query: Query<(Entity, &mut Ttl)>) {
    for (entity, mut ttl) in query.iter_mut() {
        if ttl.0 > 0 {
            ttl.0 -= 1;
        } else {
            commands.entity(entity).despawn_recursive();
        }
    }
}
