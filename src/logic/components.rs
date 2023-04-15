pub use super::people::{Age, Dead, Hunger, MoveTo, Person};
pub use super::planet::{FoodAmount, FoodSource, FoodType};
use crate::logic::measures::RealCoords;
use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Component)]
pub struct Name(pub String);

/// This component marks entities for cleanup, they will be despowned after passed amount of fps
#[derive(Component)]
pub struct Ttl(pub u32);

#[derive(Resource)]
pub struct Lookup<T> {
    pub entities: HashMap<RealCoords, Entity>,
    pub default: Option<T>,
}
