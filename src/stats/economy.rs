use bevy::prelude::*;

use crate::debug::components::Performance;
use crate::logic::components::{FoodAmount, Person};
use macros::measured;

#[derive(Resource)]
pub struct Statistics {
    pub apple_history: Vec<u32>,
    pub orange_history: Vec<u32>,
    pub people_history: Vec<u32>,
    pub current_food: u32,
    pub current_apples: u32,
    pub current_oranges: u32,
    pub current_people: u32,
}

#[measured]
pub fn food_statistics(
    food: Query<&FoodAmount>,
    people: Query<&Person>,
    mut stats: ResMut<Statistics>,
) {
    let mut apple_sum = 0;
    let mut orange_sum = 0;
    let mut people_sum = 0;
    for food_amount in food.iter() {
        apple_sum += food_amount.apples;
        orange_sum += food_amount.oranges;
    }
    for _ in people.iter() {
        people_sum += 1;
    }
    stats.apple_history.push(apple_sum);
    stats.orange_history.push(orange_sum);
    stats.people_history.push(people_sum);
    stats.current_food = apple_sum + orange_sum;
    stats.current_apples = apple_sum;
    stats.current_oranges = orange_sum;
    stats.current_people = people_sum;
}
