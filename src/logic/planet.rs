use bevy::prelude::*;

use crate::config::Config;
use crate::logic::people::GridCoords;

#[derive(Component)]
pub struct FoodSource();

#[derive(Component)]
pub struct FoodAmount(pub u32);

#[derive(Component)]
pub struct Time(pub u32);

// This system will increase food amount for all food sources
pub fn food_growth(mut query: Query<(Entity, &FoodSource, &mut FoodAmount, &GridCoords)>, config: Res<Config>, time: Res<Time>) {
    for (entity, _, mut food_amount, coords) in query.iter_mut() {
        let r = rand::random::<f32>();
        trace!(
            "Found some growing entities: rand: {} growth: {} food: {}",
            r,
            config.game.growth.value,
            food_amount.0
        );
        // increase food amount if random number is less than growth rate
        if r < config.game.growth.value && food_amount.0 < 3 && is_in_growing_season(&time, config.map.size_y.value, coords.y, config.game.year_length.value, config.game.growing_season_length.value) {
            food_amount.0 += 1;
            debug!(
                "Increased food amount for entity {} to total of {}",
                entity.id(),
                food_amount.0
            );
        }
    }
}

// this system increases time by 1 every frame
pub fn time_system(mut time: ResMut<Time>) {
    time.0 += 1;
}

// this function checks if food is in growing season
fn is_in_growing_season(time: &Time, planet_height: u32, food_location: f32, year_length: u32, growing_season_length: f32) -> bool {
    info!("Time: {}, year length: {}, planet height: {}, food location.y: {}", time.0, year_length, planet_height, food_location);
    let grow_season_start = (time.0 % year_length) as f32 / year_length as f32;
    let grow_season_end = ((time.0 as f32 + year_length as f32 * growing_season_length) % year_length as f32)  / year_length as f32;
    let normalized_food_location = food_location / planet_height as f32;
    return if grow_season_start < grow_season_end {
        grow_season_start <= normalized_food_location && grow_season_end > normalized_food_location
    } else {
        normalized_food_location > grow_season_start || normalized_food_location < grow_season_end
    };
}


#[cfg(test)]
extern crate quickcheck;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_in_growing_season() {
        let time = Time(0);
        let planet_height = 50;
        let year_length = 100;
        let growing_season_length = 0.10;
        let food_location = 0.0;
        assert_eq!(is_in_growing_season(&time, planet_height, food_location, year_length, growing_season_length), true);
        let food_location = 4.0;
        assert_eq!(is_in_growing_season(&time, planet_height, food_location, year_length, growing_season_length), true);
        let food_location = 5.0;
        assert_eq!(is_in_growing_season(&time, planet_height, food_location, year_length, growing_season_length), false);
        let time = Time(95);
        let food_location = 48.0;
        assert_eq!(is_in_growing_season(&time, planet_height, food_location, year_length, growing_season_length), true);
        let food_location = 0.0;
        assert_eq!(is_in_growing_season(&time, planet_height, food_location, year_length, growing_season_length), true);
        let food_location = 3.0;
        assert_eq!(is_in_growing_season(&time, planet_height, food_location, year_length, growing_season_length), false);
    }

    #[quickcheck]
    fn the_rows_that_grow_should_always_be_equal_to_growing_season_length(time: u32, planet_height: u32, food_location: f32, year_length: u32, growing_season_length: f32) -> bool {
        if check_reasonable_boundaries(planet_height, food_location, year_length, growing_season_length) {
            return true;
        }
        let time = Time(time);
        return (0..planet_height)
            .filter(|&i| is_in_growing_season(
                &time,
                planet_height,
                i as f32,
                year_length,
                growing_season_length,
            ))
            .count() == growing_season_length as usize
    }

    #[quickcheck]
    fn increasing_time_by_one_should_move_growing_season(time: u32, planet_height: u32, food_location: f32, year_length: u32, growing_season_length: f32) -> bool {
        if check_reasonable_boundaries(planet_height, food_location, year_length, growing_season_length) {
            return true;
        }
        return if food_location < planet_height as f32 {
            check_boundary_of_season(planet_height, food_location, food_location+1.0, year_length, growing_season_length, time)
        } else {
            check_boundary_of_season(planet_height, food_location, 0.0, year_length, growing_season_length, time)
        }
    }

    fn check_reasonable_boundaries(planet_height: u32, food_location: f32, year_length: u32, growing_season_length: f32) -> bool {
        if planet_height < 1 || year_length < 1 || growing_season_length > 1.0 || growing_season_length < 0.0 {
            return true;
        }
        if food_location >= planet_height as f32 || food_location < 0.0 {
            return true;
        }
        if planet_height > 1000 || year_length > 1000 {
            return true;
        }
        return false;
    }

    fn check_boundary_of_season(planet_height: u32, food_location: f32, next_food_location: f32, year_length: u32, growing_season_length: f32, time: u32) -> bool {
        let first = is_in_growing_season(&Time(time), planet_height, food_location, year_length, growing_season_length);
        let second = is_in_growing_season(&Time(time), planet_height, next_food_location, year_length, growing_season_length);
        if first && !second {
            is_in_growing_season(&Time(time + 1), planet_height, next_food_location , year_length, growing_season_length)
        } else {
            // we don't care about other cases
            true
        }
    }
}