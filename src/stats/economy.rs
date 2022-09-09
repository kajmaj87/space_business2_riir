use bevy::prelude::*;

use crate::logic::components::{FoodAmount, Person};

#[derive(Component)]
pub struct Statistics {
    pub food_history: Vec<u32>,
    pub people_history: Vec<u32>,
}
pub fn food_statistics(
    food: Query<&FoodAmount>,
    people: Query<&Person>,
    mut stats: ResMut<Statistics>,
) {
    let mut food_sum = 0;
    let mut people_sum = 0;
    for food_amount in food.iter() {
        food_sum += food_amount.0;
    }
    for _ in people.iter() {
        people_sum += 1;
    }
    stats.food_history.push(food_sum);
    stats.people_history.push(people_sum);
}
