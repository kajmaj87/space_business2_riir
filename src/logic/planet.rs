use bevy::prelude::{debug, trace, Component, Entity, Query};

#[derive(Component)]
pub struct FoodSource(pub f32);

#[derive(Component)]
pub struct FoodAmount(pub u32);

pub fn food_growth(mut query: Query<(Entity, &FoodSource, &mut FoodAmount)>) {
    for (entity, growth_speed, mut food_amount) in query.iter_mut() {
        let r = rand::random::<f32>();
        trace!(
            "Found some growing entities: rand: {} growth: {} food: {}",
            r,
            growth_speed.0,
            food_amount.0
        );
        if r < growth_speed.0 && food_amount.0 < 3 {
            food_amount.0 += 1;
            debug!(
                "Increased food amount for entity {} to total of {}",
                entity.id(),
                food_amount.0
            );
        }
    }
}
