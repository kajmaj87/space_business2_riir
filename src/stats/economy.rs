use bevy::prelude::*;

use crate::logic::components::FoodAmount;

#[derive(Component)]
pub struct Statistics {
    pub food_history: Vec<u32>,
}
pub fn food_statistics(query: Query<&FoodAmount>, mut stats: ResMut<Statistics>) {
    for food in query.iter() {
        stats.food_history.push(food.0);
    }
}
