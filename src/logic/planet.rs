use crate::debug::components::Performance;
use bevy::prelude::*;
use bevy::utils::HashMap;
use macros::measured;

use crate::config::Config;
use crate::logic::measures::{RealCoords, VirtualCoords};

#[derive(Component)]
pub enum FoodType {
    Apple,
    Orange,
}

#[derive(Component)]
pub struct FoodSource(pub FoodType);

#[derive(Component, Debug)]
pub struct FoodAmount {
    pub apples: u32,
    pub oranges: u32,
}

#[derive(Resource)]
pub struct FoodLookup {
    pub food: HashMap<RealCoords, Entity>,
}

#[derive(Resource)]
pub struct TotalTicks(pub u32);

// This system will increase food amount for all food sources
#[measured]
pub fn food_growth(
    mut query: Query<(Entity, &FoodSource, &mut FoodAmount, &VirtualCoords)>,
    config: Res<Config>,
    time: Res<TotalTicks>,
) {
    for (_, source, mut food_amount, coords) in query.iter_mut() {
        let r = rand::random::<f32>();
        let food = match source.0 {
            FoodType::Apple => food_amount.apples,
            FoodType::Orange => food_amount.oranges,
        };
        // increase food amount if random number is less than growth rate
        if r < config.game.growth.value
            && food < 3
            && is_in_growing_season(
                &time,
                config.map.size_y.value,
                coords.to_real(&config).y,
                config.game.year_length.value,
                config.game.growing_season_length.value,
            )
        {
            match source.0 {
                FoodType::Apple => food_amount.apples += 1,
                FoodType::Orange => food_amount.oranges += 1,
            }
        }
    }
}

// this system increases time by 1 every frame
#[measured]
pub fn time_system(mut time: ResMut<TotalTicks>) {
    time.0 += 1;
}

// this function checks if food is in growing season
fn is_in_growing_season(
    time: &TotalTicks,
    planet_height: u32,
    food_location: u32,
    year_length: u32,
    growing_season_length: f32,
) -> bool {
    info!(
        "Time: {}, year length: {}, planet height: {}, food location.y: {}",
        time.0, year_length, planet_height, food_location
    );
    let grow_season_start = (time.0 % year_length) as f32 / year_length as f32;
    let grow_season_end = ((time.0 as f32 + year_length as f32 * growing_season_length)
        % year_length as f32)
        / year_length as f32;
    let normalized_food_location = food_location as f32 / planet_height as f32;
    if grow_season_start < grow_season_end {
        grow_season_start <= normalized_food_location && grow_season_end > normalized_food_location
    } else {
        normalized_food_location > grow_season_start || normalized_food_location < grow_season_end
    }
}

#[cfg(test)]
extern crate quickcheck;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_in_growing_season() {
        let time = TotalTicks(0);
        let planet_height = 50;
        let year_length = 100;
        let growing_season_length = 0.10;
        let food_location = 0;
        assert_eq!(
            is_in_growing_season(
                &time,
                planet_height,
                food_location,
                year_length,
                growing_season_length
            ),
            true
        );
        let food_location = 4;
        assert_eq!(
            is_in_growing_season(
                &time,
                planet_height,
                food_location,
                year_length,
                growing_season_length
            ),
            true
        );
        let food_location = 5;
        assert_eq!(
            is_in_growing_season(
                &time,
                planet_height,
                food_location,
                year_length,
                growing_season_length
            ),
            false
        );
        let time = TotalTicks(95);
        let food_location = 48;
        assert_eq!(
            is_in_growing_season(
                &time,
                planet_height,
                food_location,
                year_length,
                growing_season_length
            ),
            true
        );
        let food_location = 0;
        assert_eq!(
            is_in_growing_season(
                &time,
                planet_height,
                food_location,
                year_length,
                growing_season_length
            ),
            true
        );
        let food_location = 3;
        assert_eq!(
            is_in_growing_season(
                &time,
                planet_height,
                food_location,
                year_length,
                growing_season_length
            ),
            false
        );
    }

    #[quickcheck]
    fn the_rows_that_grow_should_always_be_equal_to_growing_season_length(
        time: u32,
        planet_height: u32,
        food_location: u32,
        year_length: u32,
        growing_season_length: f32,
    ) -> bool {
        if check_reasonable_boundaries(
            planet_height,
            food_location,
            year_length,
            growing_season_length,
        ) {
            return true;
        }
        let time = TotalTicks(time);
        return (0..planet_height)
            .filter(|&i| {
                is_in_growing_season(&time, planet_height, i, year_length, growing_season_length)
            })
            .count()
            == growing_season_length as usize;
    }

    #[quickcheck]
    fn increasing_time_by_one_should_move_growing_season(
        time: u32,
        planet_height: u32,
        food_location: u32,
        year_length: u32,
        growing_season_length: f32,
    ) -> bool {
        if check_reasonable_boundaries(
            planet_height,
            food_location,
            year_length,
            growing_season_length,
        ) {
            return true;
        }
        return if food_location < planet_height {
            check_boundary_of_season(
                planet_height,
                food_location,
                food_location + 1,
                year_length,
                growing_season_length,
                time,
            )
        } else {
            check_boundary_of_season(
                planet_height,
                food_location,
                0,
                year_length,
                growing_season_length,
                time,
            )
        };
    }

    fn check_reasonable_boundaries(
        planet_height: u32,
        food_location: u32,
        year_length: u32,
        growing_season_length: f32,
    ) -> bool {
        if planet_height < 1
            || year_length < 1
            || growing_season_length > 1.0
            || growing_season_length < 0.0
        {
            return true;
        }
        if food_location > planet_height || food_location < 0 {
            return true;
        }
        if planet_height > 1000 || year_length > 1000 {
            return true;
        }
        return false;
    }

    fn check_boundary_of_season(
        planet_height: u32,
        food_location: u32,
        next_food_location: u32,
        year_length: u32,
        growing_season_length: f32,
        time: u32,
    ) -> bool {
        let first = is_in_growing_season(
            &TotalTicks(time),
            planet_height,
            food_location,
            year_length,
            growing_season_length,
        );
        let second = is_in_growing_season(
            &TotalTicks(time),
            planet_height,
            next_food_location,
            year_length,
            growing_season_length,
        );
        if first && !second {
            is_in_growing_season(
                &TotalTicks(time + 1),
                planet_height,
                next_food_location,
                year_length,
                growing_season_length,
            )
        } else {
            // we don't care about other cases
            true
        }
    }
}
