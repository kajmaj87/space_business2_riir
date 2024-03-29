use crate::debug::components::Performance;
use bevy::ecs::query::QueryEntityError;
use bevy::prelude::*;
use macros::measured;
use rand::random;

use crate::config::Config;
use crate::logic::components::Lookup;
use crate::logic::measures::VirtualCoords;
use crate::logic::people::{
    free_neighbouring_coords, occupied_neighbouring_coords, Female, Fertile, Male, Person,
    PersonBundle,
};
use crate::stats::components::{Statistics, Transaction};

use super::planet::FoodAmount;

#[derive(Component, Debug)]
pub struct PeopleInteraction {
    pub a: Entity,
    pub b: Entity,
}

#[measured]
pub fn add_interaction_system(
    config: Res<Config>,
    people: Res<Lookup<Person>>,
    query: Query<(Entity, &Person, &VirtualCoords)>,
    mut commands: Commands,
) {
    for (person, _, coords) in query.iter() {
        let neighbors = occupied_neighbouring_coords(&config, coords, &people);
        if !neighbors.is_empty() {
            for neighbor in neighbors.iter() {
                let interaction = PeopleInteraction {
                    a: person,
                    b: *people.entities.get(&neighbor.to_real(&config)).unwrap(),
                };
                debug!("Interaction added: {:?}", interaction);
                commands.spawn(interaction);
            }
        }
    }
}

#[measured]
pub fn breeding_interaction_system(
    mut commands: Commands,
    mut mothers: Query<(&mut FoodAmount, &VirtualCoords, &Fertile), With<Female>>,
    mut fathers: Query<(&mut FoodAmount, &VirtualCoords, &Fertile), Without<Female>>,
    config: Res<Config>,
    mut lookup: ResMut<Lookup<Person>>,
    interactions: Query<&PeopleInteraction>,
) {
    for interaction in interactions.iter() {
        {
            let father = fathers.get_mut(interaction.a);
            let mother = mothers.get_mut(interaction.b);
            create_baby(&mut commands, &config, &mut lookup, father, mother);
        }
        {
            let father = fathers.get_mut(interaction.b);
            let mother = mothers.get_mut(interaction.a);
            create_baby(&mut commands, &config, &mut lookup, father, mother);
        }
    }
}

fn create_baby(
    commands: &mut Commands,
    config: &Res<Config>,
    lookup: &mut ResMut<Lookup<Person>>,
    father: Result<(Mut<FoodAmount>, &VirtualCoords, &Fertile), QueryEntityError>,
    mother: Result<(Mut<FoodAmount>, &VirtualCoords, &Fertile), QueryEntityError>,
) {
    if let (Ok((mut father_food, _, _)), Ok((mut mother_food, mother_coords, _))) = (father, mother)
    {
        let free_space = free_neighbouring_coords(config, mother_coords, lookup);
        if !free_space.is_empty()
            && father_food.apples + mother_food.apples > config.game.food_for_baby.value
            && father_food.oranges + mother_food.oranges > config.game.food_for_baby.value
        {
            let baby_oranges = father_food.oranges / 2 + mother_food.oranges / 2;
            let baby_apples = father_food.apples / 2 + mother_food.apples / 2;
            father_food.apples -= father_food.apples / 2;
            father_food.oranges -= father_food.oranges / 2;
            mother_food.apples -= mother_food.apples / 2;
            mother_food.oranges -= mother_food.oranges / 2;
            let baby_coords = free_space[random::<usize>() % free_space.len()];
            let mut baby = commands.spawn(PersonBundle {
                food: FoodAmount {
                    apples: baby_apples,
                    oranges: baby_oranges,
                },
                position: baby_coords,
                ..Default::default()
            });
            if random::<u8>() % 2 == 0 {
                baby.insert(Male);
            } else {
                baby.insert(Female);
            }
            lookup
                .entities
                .insert(baby_coords.to_real(config), baby.id());
        }
    }
}

#[measured]
pub fn trade_interaction_system(
    query: Query<&PeopleInteraction>,
    people: Query<(&Person, &mut FoodAmount)>,
    mut stats: ResMut<Statistics>,
    config: Res<Config>,
) {
    if !config.game.trade_allowed.value {
        return;
    }
    for interaction in query.iter() {
        let (a, b) = (people.get(interaction.a), people.get(interaction.b));
        if let (Ok((_, a_food)), Ok((_, b_food))) = (a, b) {
            // mrs < 1 means agent a is poor in oranges and rich in apples
            // mrs > 1 means agent a is rich in oranges and poor in apples
            let mrs_a = calculate_marginal_rate_of_substitution(a_food.apples, a_food.oranges);
            let mrs_b = calculate_marginal_rate_of_substitution(b_food.apples, b_food.oranges);
            if mrs_a < 1.0 && mrs_b > 1.0 {
                trade_apples_for_oranges(a_food, b_food, &mut stats);
            } else if mrs_a > 1.0 && mrs_b < 1.0 {
                trade_apples_for_oranges(b_food, a_food, &mut stats);
            }
        }
    }
}

fn trade_apples_for_oranges(
    a_food: &FoodAmount,
    b_food: &FoodAmount,
    stats: &mut ResMut<Statistics>,
) {
    let u_a = calculate_utility(a_food.apples, a_food.oranges);
    let u_b = calculate_utility(b_food.apples, b_food.oranges);
    let apples_to_trade = if a_food.apples > b_food.apples {
        (a_food.apples - b_food.apples) / 2
    } else {
        1
    };
    let oranges_to_trade = if a_food.oranges < b_food.oranges {
        (b_food.oranges - a_food.oranges) / 2
    } else {
        1
    };
    if calculate_utility(
        a_food.apples - apples_to_trade,
        a_food.oranges + oranges_to_trade,
    ) > u_a
        && calculate_utility(
            b_food.apples + apples_to_trade,
            b_food.oranges - oranges_to_trade,
        ) > u_b
    {
        debug!(
            "Trade accepted for {} o/a, A: {:?} -> {:?}, B: {:?} -> {:?}",
            oranges_to_trade as f32 / apples_to_trade as f32,
            a_food,
            FoodAmount {
                apples: a_food.apples - apples_to_trade,
                oranges: a_food.oranges + oranges_to_trade,
            },
            b_food,
            FoodAmount {
                apples: b_food.apples + apples_to_trade,
                oranges: b_food.oranges - oranges_to_trade,
            }
        );
        if let Some(transactions) = stats.trade_history.last_mut() {
            transactions.push(Transaction {
                apples: apples_to_trade,
                oranges: oranges_to_trade,
            });
        } else {
            stats.trade_history.push(vec![Transaction {
                apples: apples_to_trade,
                oranges: oranges_to_trade,
            }]);
        }
    }
}

fn calculate_utility(apples: u32, oranges: u32) -> f32 {
    ((apples as f32) * (oranges as f32)).sqrt()
}

fn calculate_apple_marginal_utility(apples: u32, oranges: u32) -> f32 {
    calculate_utility(apples + 1, oranges) - calculate_utility(apples, oranges)
}

fn calculate_orange_marginal_utility(apples: u32, oranges: u32) -> f32 {
    calculate_utility(apples, oranges + 1) - calculate_utility(apples, oranges)
}

fn calculate_marginal_rate_of_substitution(apples: u32, oranges: u32) -> f32 {
    calculate_apple_marginal_utility(apples, oranges)
        / calculate_orange_marginal_utility(apples, oranges)
}

#[measured]
pub fn cleanup_interactions_system(
    query: Query<(Entity, &PeopleInteraction)>,
    mut commands: Commands,
) {
    for (entity, _) in query.iter() {
        commands.entity(entity).despawn();
    }
}
