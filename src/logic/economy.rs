use bevy::prelude::*;
use big_brain::thinker::ThinkerBuilder;
use function_name::named;
use iyes_loopless::prelude::*;

use crate::config::Config;

use super::{
    components::{FoodSource, GridCoords, Name, Ttl},
    planet::FoodAmount,
    TurnPhase, TurnStep,
};

pub enum Resource {
    Food,
    Money,
}

#[derive(Component)]
pub struct Job {
    pub payout: u32,
    pub payout_type: Resource,
    pub position: GridCoords,
    pub taken_by: Option<Entity>,
}

#[derive(Component)]
pub struct Workplace {
    pub job_template: Job,
    pub amount_of_jobs: u32,
}

pub fn generate_jobs() {
    todo!();
}
