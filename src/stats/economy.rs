use bevy::prelude::*;

use crate::logic::components::FoodAmount;

#[derive(Component)]
pub struct Statistics {
    pub food_history: Vec<u32>,
}
pub fn food_statistics(query: Query<&FoodAmount>, mut stats: ResMut<Statistics>) {
    let mut sum = 0;
    for food in query.iter() {
        sum += food.0;
    }
    stats.food_history.push(sum);
}
