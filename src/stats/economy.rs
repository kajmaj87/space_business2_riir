use bevy::prelude::*;

use crate::debug::components::Performance;
use crate::logic::components::{FoodAmount, FoodSource, Person};
use macros::measured;

#[derive(Resource)]
pub struct Statistics {
    pub apple_history_sources: Vec<u32>,
    pub orange_history_sources: Vec<u32>,
    pub apple_history_people: Vec<u32>,
    pub orange_history_people: Vec<u32>,
    pub people_history: Vec<u32>,
    pub current_food: u32,
    pub current_apples: u32,
    pub current_oranges: u32,
    pub current_people: u32,
}

#[measured]
pub fn food_statistics(
    food_in_sources: Query<&FoodAmount, With<FoodSource>>,
    food_in_people: Query<&FoodAmount, With<Person>>,
    people: Query<&Person>,
    mut stats: ResMut<Statistics>,
) {
    let mut apple_sum_sources = 0;
    let mut orange_sum_sources = 0;
    let mut apple_sum_people = 0;
    let mut orange_sum_people = 0;
    let mut people_sum = 0;
    for food_amount in food_in_sources.iter() {
        apple_sum_sources += food_amount.apples;
        orange_sum_sources += food_amount.oranges;
    }
    for food_amount in food_in_people.iter() {
        apple_sum_people += food_amount.apples;
        orange_sum_people += food_amount.oranges;
    }
    for _ in people.iter() {
        people_sum += 1;
    }
    stats.apple_history_sources.push(apple_sum_sources);
    stats.orange_history_sources.push(orange_sum_sources);
    stats.apple_history_people.push(apple_sum_people);
    stats.orange_history_people.push(orange_sum_people);
    stats.people_history.push(people_sum);
    stats.current_food = apple_sum_sources + orange_sum_sources;
    stats.current_apples = apple_sum_sources;
    stats.current_oranges = orange_sum_sources;
    stats.current_people = people_sum;
}
