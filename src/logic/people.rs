use bevy::prelude::*;
use big_brain::thinker::ThinkerBuilder;
use std::time::Instant;

use crate::config::Config;

use super::{
    components::{FoodSource, Name, Ttl},
    planet::FoodAmount,
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
    pub dx: f32,
    pub dy: f32,
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
            .add_system(hunger_system)
            .add_system(move_system)
            .add_system(foraging_system)
            .add_system(breeding_system)
            .add_system(aging_system)
            // we need to despawn enities separately so that no commands use them in wrong moment
            .add_system(cleanup_system.in_base_set(CoreSet::PostUpdate));
    }
}

pub fn init_people(mut commands: Commands, config: Res<Config>) {
    info!("People initialized");
    for _ in 0..config.game.starting_people.value {
        commands.spawn(PersonBundle::default());
    }
}

fn hunger_system(
    mut commands: Commands,
    mut query: Query<(Entity, &Person, &mut Hunger), Without<Dead>>,
    config: Res<Config>,
) {
    for (person, _, mut hunger) in query.iter_mut() {
        hunger.0 += config.game.hunger_increase.value;
        if hunger.0 > 1.0 {
            mark_entity_as_dead(person, &mut commands, &config);
            info!("Person {} has died of hunger ({})", person.index(), hunger.0);
        }
    }
}

fn aging_system(
    mut commands: Commands,
    mut query: Query<(Entity, &Person, &mut Age), Without<Dead>>,
    config: Res<Config>,
) {
    for (person, _, mut age) in query.iter_mut() {
        age.0 += 1;
        if age.0 > config.game.max_person_age.value && config.game.max_person_age.value > 0 {
            mark_entity_as_dead(person, &mut commands, &config);
            info!(
                "Person {} died of old age being {} turns old",
                person.index(),
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

fn move_system(
    mut commands: Commands,
    mut query: Query<(Entity, &Move, &mut GridCoords)>,
    config: Res<Config>,
) {
    for (person, move_component, mut coords) in query.iter_mut() {
        commands.entity(person).remove::<Move>();
        let newx = move_component.dx + coords.x;
        let newy = move_component.dy + coords.y;
        coords.x = newx.rem_euclid(config.map.size_x.value as f32);
        coords.y = newy.rem_euclid(config.map.size_y.value as f32);
    }
}

#[allow(clippy::type_complexity)]
fn foraging_system(
    mut commands: Commands,
    mut people: Query<
        (Entity, &mut FoodAmount, &GridCoords),
        (Changed<Forage>, With<Person>, With<Forage>),
    >,
    mut food_producers: Query<(&mut FoodAmount, &GridCoords), (With<FoodSource>, Without<Person>)>,
) {
    // print size of food_producers and people and their multiplicities

    info!(
        "Iterations: {}, food producers: {}, people: {}",
        food_producers.iter().count() * people.iter().count(),
        food_producers.iter().count(),
        people.iter().count()
    );
    // measure time of the following loop
    let start = Instant::now();
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
    info!("Foraging took: {:?}", start.elapsed());
}

fn breeding_system(
    mut commands: Commands,
    mut people: Query<(&mut FoodAmount, &GridCoords), With<Person>>,
    config: Res<Config>,
) {
    for (mut person_food_amount, coords) in people.iter_mut() {
        if person_food_amount.0 > 2 * config.game.food_for_baby.value {
            info!("I'm having a baby! My food is: {}", person_food_amount.0);
            person_food_amount.0 -= config.game.food_for_baby.value;
            commands.spawn(PersonBundle {
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
