use crate::debug::components::Performance;
use bevy::prelude::*;
use macros::measured;

use crate::config::Config;
use crate::logic::components::{Dead, Lookup};
use crate::logic::interactions::PeopleInteraction;
use crate::logic::measures::VirtualCoords;
use crate::logic::people::Person;

pub struct InvariantsPlugin;

impl Plugin for InvariantsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(inv_one_person_per_space_check.in_base_set(CoreSet::PreUpdate))
            .add_system(
                inv_person_lookup_has_correct_amount_of_people.in_base_set(CoreSet::PreUpdate),
            )
            .add_system(inv_no_self_interactions_allowed.in_base_set(CoreSet::Last))
            .add_system(inv_no_interactions_at_start.in_base_set(CoreSet::First));
    }
}

#[measured]
fn inv_person_lookup_has_correct_amount_of_people(
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
fn inv_one_person_per_space_check(
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

#[measured]
fn inv_no_self_interactions_allowed(query: Query<&PeopleInteraction>) {
    for interaction in query.iter() {
        if interaction.a == interaction.b {
            panic!("Self interaction detected: {:?}", interaction);
        }
    }
}

#[measured]
fn inv_no_interactions_at_start(query: Query<&PeopleInteraction>) {
    for interaction in query.iter() {
        panic!("Interaction detected at start: {:?}", interaction);
    }
}
