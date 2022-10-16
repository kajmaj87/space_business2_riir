use bevy::prelude::*;
use rand::{thread_rng, Rng};

use crate::config::Config;

#[derive(Component)]
pub struct FoodSource();

#[derive(Component)]
pub struct FoodAmount(pub u32);

pub fn food_growth(mut query: Query<(Entity, &FoodSource, &mut FoodAmount)>, config: Res<Config>) {
    info!("Running food_growth system");
    for (entity, _, mut food_amount) in query.iter_mut() {
        let mut r = rand::thread_rng();
        let roll = r.gen_range(1..=6);
        trace!(
            "Found some growing entities: rand: {} growth: {} food: {}",
            roll,
            config.game.growth.value,
            food_amount.0
        );
        if roll >= config.game.growth.value && food_amount.0 < 3 {
            food_amount.0 += 1;
            debug!(
                "Increased food amount for entity {} to total of {}",
                entity.id(),
                food_amount.0
            );
        }
    }
}
