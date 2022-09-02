pub use super::planet::{FoodAmount, FoodSource};
use bevy::prelude::*;

pub const FOOD_GENERATION_PER_FRAME: f32 = 0.01;

#[derive(Component)]
pub struct Name(pub String);
