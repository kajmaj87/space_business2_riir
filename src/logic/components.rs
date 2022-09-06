pub use super::people::{Dead, Hunger, Person};
pub use super::planet::{FoodAmount, FoodSource};
use bevy::prelude::*;

#[derive(Component)]
pub struct Name(pub String);

/// This component marks entities for cleanup, they will be despowned after passed amount of fps
#[derive(Component)]
pub struct Ttl(pub u32);
