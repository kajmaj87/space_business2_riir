use crate::debug::components::Performance;
use bevy::prelude::*;
use big_brain::thinker::ThinkerBuilder;
use macros::measured;
use std::collections::HashMap;

use crate::config::Config;
use crate::logic::components::FoodLookup;
use crate::logic::planet::FoodType;

use super::{
    components::{FoodSource, Name, Ttl},
    planet::FoodAmount,
};

#[derive(Component)]
pub struct Hunger {
    pub apple: f32,
    pub orange: f32,
}

#[derive(Component)]
pub struct Person;

#[derive(Component)]
pub struct Age(pub u32);

#[derive(Component)]
pub struct Dead;

#[derive(Component)]
pub struct Move {
    pub dx: i32,
    pub dy: i32,
}

#[derive(Component)]
pub struct Forage;

// Position and GridPostion are already defined in bevy::prelude
#[derive(Component, PartialEq, Eq, Hash, Copy, Clone)]
pub struct GridCoords {
    pub x: u32,
    pub y: u32,
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
            hunger: Hunger {
                apple: 0.0,
                orange: 0.0,
            },
            food: FoodAmount {
                apples: 3,
                oranges: 3,
            },
            position: GridCoords { x: 5, y: 3 },
        }
    }
}

pub struct PeoplePlugin;

impl Plugin for PeoplePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(init_people)
            .insert_resource(FoodLookup {
                food: HashMap::new(),
            })
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

#[measured]
fn hunger_system(mut query: Query<(&Person, &mut Hunger), Without<Dead>>, config: Res<Config>) {
    for (_, mut hunger) in query.iter_mut() {
        hunger.apple += config.game.hunger_increase.value;
        hunger.orange += config.game.hunger_increase.value;
    }
}

#[measured]
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

pub fn mark_entity_as_dead(person: Entity, commands: &mut Commands, config: &Res<Config>) {
    commands
        .entity(person)
        .insert(Dead)
        .insert(Ttl(config.game.person_ttl.value))
        .remove::<ThinkerBuilder>();
}

#[measured]
fn move_system(
    mut commands: Commands,
    mut query: Query<(Entity, &Move, &mut GridCoords)>,
    config: Res<Config>,
) {
    for (person, move_component, mut coords) in query.iter_mut() {
        commands.entity(person).remove::<Move>();
        coords.x = add_modulo(move_component.dx, coords.x, config.map.size_x.value);
        coords.y = add_modulo(move_component.dy, coords.y, config.map.size_y.value);
    }
}

fn add_modulo(a: i32, b: u32, z: u32) -> u32 {
    let a = a.rem_euclid(z as i32) as u32;
    let sum = a.wrapping_add(b);
    sum % z
}

#[measured]
#[allow(clippy::type_complexity)]
fn foraging_system(
    mut commands: Commands,
    mut people: Query<
        (Entity, &mut FoodAmount, &GridCoords),
        (Changed<Forage>, With<Person>, With<Forage>),
    >,
    mut food_producers: Query<(&mut FoodAmount, &GridCoords, &FoodSource), Without<Person>>,
    food_lookup: Res<FoodLookup>,
) {
    for (person, mut person_food_amount, coords) in people.iter_mut() {
        if let Some(food) = food_lookup.food.get(coords) {
            if let Ok((mut food_amount, _, source)) = food_producers.get_mut(*food) {
                debug!("Found some food!");
                match source.0 {
                    FoodType::Apple => {
                        if food_amount.apples > 0 {
                            person_food_amount.apples += 1;
                            food_amount.apples -= 1;
                        }
                    }
                    FoodType::Orange => {
                        if food_amount.oranges > 0 {
                            person_food_amount.oranges += 1;
                            food_amount.oranges -= 1;
                        }
                    }
                }
                commands.entity(person).remove::<Forage>();
            }
        }
    }
}

#[measured]
fn breeding_system(
    mut commands: Commands,
    mut people: Query<(&mut FoodAmount, &GridCoords), With<Person>>,
    config: Res<Config>,
) {
    for (mut person_food_amount, coords) in people.iter_mut() {
        if person_food_amount.apples + person_food_amount.oranges
            > 2 * config.game.food_for_baby.value
        {
            info!(
                "I'm having a baby! My food is: {}",
                person_food_amount.apples + person_food_amount.oranges
            );
            let baby_oranges = person_food_amount.oranges / 2;
            let baby_apples = person_food_amount.apples / 2;
            person_food_amount.apples -= baby_apples;
            person_food_amount.oranges -= baby_oranges;
            commands.spawn(PersonBundle {
                food: FoodAmount {
                    apples: baby_apples,
                    oranges: baby_oranges,
                },
                position: GridCoords {
                    x: coords.x,
                    y: coords.y,
                },
                ..Default::default()
            });
        }
    }
}

#[measured]
fn cleanup_system(mut commands: Commands, mut query: Query<(Entity, &mut Ttl)>) {
    for (entity, mut ttl) in query.iter_mut() {
        if ttl.0 > 0 {
            ttl.0 -= 1;
        } else {
            commands.entity(entity).despawn_recursive();
        }
    }
}
