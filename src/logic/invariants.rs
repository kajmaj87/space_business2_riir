use crate::debug::components::Performance;
use bevy::prelude::*;
use macros::measured;

use crate::config::Config;
use crate::logic::ai::eat_action_system;
use crate::logic::components::{Dead, Lookup};
use crate::logic::measures::VirtualCoords;
use crate::logic::people::{aging_system, breeding_system, move_system, Person};

pub struct InvariantsPlugin;

impl Plugin for InvariantsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(
            one_person_per_space_check
                .before(move_system)
                .before(breeding_system),
        )
        .add_system(
            person_lookup_has_correct_amount_of_people
                .after(aging_system)
                .after(eat_action_system),
        );
    }
}

#[measured]
fn person_lookup_has_correct_amount_of_people(
    alive: Query<(Entity, &Person, &VirtualCoords)>,
    dead: Query<(Entity, &Dead, &VirtualCoords)>,
    person_lookup: Res<Lookup<Person>>,
) {
    let total = alive.iter().count() + dead.iter().count();
    if person_lookup.entities.len() != total {
        panic!(
            "Person lookup has wrong amount of people. Lookup vs query: {} != {}",
            person_lookup.entities.len(),
            total
        );
    }
}
#[measured]
fn one_person_per_space_check(
    config: Res<Config>,
    query: Query<(Entity, &Person, &VirtualCoords)>,
    person_lookup: Res<Lookup<Person>>,
) {
    for (person, _, coords) in query.iter() {
        if let Some(other_person) = person_lookup.entities.get(&coords.to_real(&config)) {
            if *other_person != person {
                panic!(
                    "Two people in one place! {} and {} at {:?}",
                    person.index(),
                    other_person.index(),
                    coords
                );
            }
        }
    }
}
