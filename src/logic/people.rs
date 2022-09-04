use bevy::prelude::*;

use crate::config::Config;

use super::{
    components::{Name, Ttl},
    planet::FoodAmount,
};

#[derive(Component)]
struct Hunger(f32);

#[derive(Component)]
pub struct Person;

#[derive(Component)]
pub struct Dead;

#[derive(Bundle)]
struct PersonBundle {
    name: Name,
    type_marker: Person,
    hunger: Hunger,
    food: FoodAmount,
    #[bundle]
    sprite: SpriteBundle,
}

pub struct PeoplePlugin;

impl Plugin for PeoplePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(init_people)
            .add_system(hunger_system)
            .add_system(eating_system)
            .add_system(cleanup_system);
    }
}

fn init_people(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(PersonBundle {
        name: Name(String::from("Test guy")),
        type_marker: Person,
        hunger: Hunger(0.0),
        food: FoodAmount(3),
        sprite: SpriteBundle {
            texture: asset_server.load("person.png"),
            transform: Transform {
                translation: Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 1.0,
                },
                ..Default::default()
            },
            ..Default::default()
        },
    });
}

fn hunger_system(
    mut commands: Commands,
    mut query: Query<(Entity, &Person, &mut Hunger), Without<Dead>>,
    config: Res<Config>,
) {
    for (person, _, mut hunger) in query.iter_mut() {
        info!("Person hunger value: {}", hunger.0);
        hunger.0 += config.game.hunger_increase.value;
        if hunger.0 > 1.0 {
            commands.entity(person).insert(Dead);
            commands.entity(person).insert(Ttl(600));
            info!("Person hunger value: {}, person has died", hunger.0);
        }
    }
}

// TODO this will be removed when AI is implemented
fn eating_system(mut query: Query<(&mut Hunger, &mut FoodAmount)>, config: Res<Config>) {
    for (mut hunger, mut food) in query.iter_mut() {
        if hunger.0 > config.game.hunger_decrease.value && food.0 > 0 {
            hunger.0 = 0.0;
            food.0 -= 1;
            info!("Person ate something, food left: {}", food.0);
        }
    }
}

fn cleanup_system(mut commands: Commands, mut query: Query<(Entity, &mut Ttl)>) {
    for (entity, mut ttl) in query.iter_mut() {
        if ttl.0 > 0 {
            ttl.0 -= 1;
        } else {
            commands.entity(entity).despawn_recursive();
        }
    }
}
