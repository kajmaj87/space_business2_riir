use crate::debug::components::Performance;
use bevy::prelude::*;
use big_brain::thinker::ThinkerBuilder;
use macros::measured;
use rand::random;
use rand::seq::IteratorRandom;
use std::collections::HashMap;

use crate::config::Config;
use crate::logic::components::Lookup;
use crate::logic::measures::{RealCoords, VirtualCoords};
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
pub struct Male;

#[derive(Component)]
pub struct Female;

#[derive(Component)]
pub struct Fertile;

#[derive(Component)]
pub struct Age(pub u32);

#[derive(Component)]
pub struct Dead;

#[derive(Component)]
pub struct MoveTo {
    pub dest: VirtualCoords,
}

#[derive(Component)]
pub struct Forage;

pub struct Information {
    pub entity: Entity,
    pub coords: VirtualCoords,
}

#[derive(Component)]
pub struct Knowledge {
    pub infos: Vec<Information>,
}

#[derive(Bundle)]
pub struct PersonBundle {
    pub name: Name,
    pub type_marker: Person,
    pub age: Age,
    pub hunger: Hunger,
    pub food: FoodAmount,
    pub position: VirtualCoords,
    pub knowledge: Knowledge,
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
            position: VirtualCoords { x: 5, y: 3 },
            knowledge: Knowledge { infos: Vec::new() },
        }
    }
}

pub struct PeoplePlugin;

impl Plugin for PeoplePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(init_people)
            .insert_resource(Lookup::<FoodSource> {
                entities: HashMap::new(),
                default: None,
            })
            .insert_resource(Lookup::<Person> {
                entities: HashMap::new(),
                default: None,
            })
            .add_system(hunger_system)
            .add_system(move_system)
            .add_system(foraging_system)
            .add_system(aging_system)
            .add_system(fertility_system)
            // we need to despawn enities separately so that no commands use them in wrong moment
            .add_system(cleanup_system.in_base_set(CoreSet::PostUpdate));
    }
}

pub fn init_people(
    mut commands: Commands,
    config: Res<Config>,
    mut lookup: ResMut<Lookup<Person>>,
) {
    info!("People initialized");
    let people_to_spawn =
        if config.game.starting_people.value > config.map.size_x.value * config.map.size_y.value {
            warn!(
                "Too many people for the map size, spawning only {} people",
                config.map.size_x.value * config.map.size_y.value
            );
            config.map.size_x.value * config.map.size_y.value
        } else {
            config.game.starting_people.value
        };
    while lookup.entities.len() < people_to_spawn as usize {
        let x = random::<u32>() % config.map.size_x.value;
        let y = random::<u32>() % config.map.size_y.value;
        if lookup.entities.get(&RealCoords { x, y }).is_none() {
            let mut person = commands.spawn(PersonBundle {
                position: VirtualCoords {
                    x: x as i32,
                    y: y as i32,
                },
                age: Age(random::<u32>() % config.game.max_person_age.value),
                ..default()
            });
            if random::<u8>() % 2 == 0 {
                person.insert(Male);
            } else {
                person.insert(Female);
            }
            lookup.entities.insert(RealCoords { x, y }, person.id());
            trace!(
                "Person spawned at {}, {}. Lookup size: {}",
                x,
                y,
                lookup.entities.len()
            );
        }
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
pub fn aging_system(
    mut commands: Commands,
    mut query: Query<(Entity, &Person, &mut Age), Without<Dead>>,
    config: Res<Config>,
) {
    for (person, _, mut age) in query.iter_mut() {
        age.0 += 1;
        if age.0 > config.game.max_person_age.value && config.game.max_person_age.value > 0 {
            mark_entity_as_dead(person, &mut commands, &config);
            debug!(
                "Person {} died of old age being {} turns old",
                person.index(),
                age.0
            );
        }
    }
}

#[measured]
fn fertility_system(
    mut commands: Commands,
    males: Query<&Male>,
    females: Query<&Female>,
    query: Query<(Entity, &Person, &Age)>,
    config: Res<Config>,
) {
    for (person, _, age) in query.iter() {
        if let Ok(_male) = males.get(person) {
            if is_in_male_fertile_age(age.0, &config) {
                commands.entity(person).insert(Fertile);
            } else {
                commands.entity(person).remove::<Fertile>();
            }
        }
        if let Ok(_female) = females.get(person) {
            if is_in_female_fertile_age(age.0, &config) {
                commands.entity(person).insert(Fertile);
            } else {
                commands.entity(person).remove::<Fertile>();
            }
        }
    }
}

fn is_in_male_fertile_age(age: u32, config: &Config) -> bool {
    (config.game.min_fertile_age_male.value * config.game.max_person_age.value as f32)
        < (age as f32)
        && (age as f32)
            < (config.game.max_fertile_age_male.value * config.game.max_person_age.value as f32)
}

fn is_in_female_fertile_age(age: u32, config: &Config) -> bool {
    (config.game.min_fertile_age_female.value * config.game.max_person_age.value as f32)
        < (age as f32)
        && (age as f32)
            < (config.game.max_fertile_age_female.value * config.game.max_person_age.value as f32)
}

pub fn mark_entity_as_dead(person: Entity, commands: &mut Commands, config: &Res<Config>) {
    commands
        .entity(person)
        .insert(Dead)
        .insert(Ttl(config.game.person_ttl.value))
        .remove::<Person>()
        .remove::<ThinkerBuilder>();
}

#[measured]
pub fn move_system(
    mut commands: Commands,
    mut query: Query<(Entity, &mut MoveTo, &VirtualCoords)>,
    config: Res<Config>,
    mut person_lookup: ResMut<Lookup<Person>>,
) {
    for (person, move_component, coords) in query.iter_mut() {
        let move_vector = VirtualCoords {
            x: move_component.dest.x - coords.to_real(&config).x as i32,
            y: move_component.dest.y - coords.to_real(&config).y as i32,
        };
        let (delta_x, delta_y) =
            // horizontal move
            if move_vector.x > 0 {
                (1, 0)
            } else if move_vector.x < 0 {
                (-1, 0)
            } else if move_vector.y > 0 {
                (0, 1)
            } else if move_vector.y < 0 {
                (0, -1)
            } else {
                (0, 0)
            };
        if delta_x == 0 && delta_y == 0 {
            commands.entity(person).remove::<MoveTo>();
        }
        trace!(
            "Person {} moved from {:?} by {:?} (move vector {:?})",
            person.index(),
            coords,
            (delta_x, delta_y),
            move_vector
        );
        let new_position = VirtualCoords {
            x: coords.x + delta_x,
            y: coords.y + delta_y,
        };
        if person_lookup
            .entities
            .get(&new_position.to_real(&config))
            .is_none()
        {
            commands.entity(person).insert(new_position);
            person_lookup
                .entities
                .insert(new_position.to_real(&config), person);
            person_lookup.entities.remove(&coords.to_real(&config));
        } else {
            debug!(
                "Person {} tried to move to {:?} but there is already someone there",
                person.index(),
                new_position
            );
        }
    }
}

#[measured]
#[allow(clippy::type_complexity)]
fn foraging_system(
    mut commands: Commands,
    mut people: Query<
        (Entity, &mut FoodAmount, &VirtualCoords),
        (Changed<Forage>, With<Person>, With<Forage>),
    >,
    mut food_producers: Query<(&mut FoodAmount, &VirtualCoords, &FoodSource), Without<Person>>,
    food_lookup: Res<Lookup<FoodSource>>,
    config: Res<Config>,
) {
    for (person, mut person_food_amount, coords) in people.iter_mut() {
        if let Some(food) = food_lookup.entities.get(&coords.to_real(&config)) {
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

pub fn free_neighbouring_coords(
    config: &Res<Config>,
    coords: &VirtualCoords,
    lookup: &ResMut<Lookup<Person>>,
) -> Vec<VirtualCoords> {
    let mut result = Vec::new();
    for dx in -1..=1 {
        for dy in -1..=1 {
            if dx == 0 && dy == 0 {
                continue;
            }
            let new_position = VirtualCoords {
                x: coords.x + dx,
                y: coords.y + dy,
            };
            // those two can be the same in flat earth or rings
            let real_position = new_position.to_real(config);
            let origin_position = coords.to_real(config);
            if real_position != origin_position
                && lookup.entities.get(&new_position.to_real(config)).is_none()
            {
                result.push(new_position);
            }
        }
    }
    result
}

pub fn occupied_neighbouring_coords(
    config: &Res<Config>,
    coords: &VirtualCoords,
    lookup: &Res<Lookup<Person>>,
) -> Vec<VirtualCoords> {
    let mut result = Vec::new();
    for dx in -1..=1 {
        for dy in -1..=1 {
            if dx == 0 && dy == 0 {
                continue;
            }
            let new_position = VirtualCoords {
                x: coords.x + dx,
                y: coords.y + dy,
            };
            // those two can be the same in flat earth or rings
            let real_position = new_position.to_real(config);
            let origin_position = coords.to_real(config);
            if real_position != origin_position
                && lookup.entities.get(&new_position.to_real(config)).is_some()
            {
                result.push(new_position);
            }
        }
    }
    debug!("Occupied neighbours: {:?}, I'm at: {:?}", result, coords);
    result
}

#[measured]
#[allow(clippy::type_complexity)]
fn cleanup_system(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Ttl)>,
    query_person: Query<(&Dead, &VirtualCoords, &FoodAmount)>,
    mut lottery_person: Query<(&Person, &mut FoodAmount), Without<Dead>>,
    mut people: ResMut<Lookup<Person>>,
    config: Res<Config>,
) {
    for (entity, mut ttl) in query.iter_mut() {
        if ttl.0 > 0 {
            ttl.0 -= 1;
        } else {
            if let Ok((_, coords, food_to_inherit)) = query_person.get(entity) {
                debug!("Person {} died, removing from coords", entity.index());
                let mut rng = rand::thread_rng();
                people.entities.remove(&coords.to_real(&config));
                if config.game.death_lottery.value {
                    if let Some((_, mut winner_food)) = lottery_person.iter_mut().choose(&mut rng) {
                        winner_food.apples += food_to_inherit.apples;
                        winner_food.oranges += food_to_inherit.oranges;
                    }
                }
            }
            commands.entity(entity).despawn_recursive();
        }
    }
}
