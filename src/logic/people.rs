use bevy::prelude::*;

use crate::config::Config;

use super::{
    components::{Name, Ttl},
    planet::FoodAmount,
};

#[derive(Component)]
pub struct Hunger(pub f32);

#[derive(Component)]
pub struct Person;

#[derive(Component)]
pub struct Dead;

// Position and GridPostion are already defined in bevy::prelude
#[derive(Component)]
pub struct GridCoords {
    pub x: f32,
    pub y: f32,
}

#[derive(Bundle)]
struct PersonBundle {
    name: Name,
    type_marker: Person,
    hunger: Hunger,
    food: FoodAmount,
    position: GridCoords,
    #[bundle]
    sprite: SpriteBundle,
}

pub struct PeoplePlugin;

impl Plugin for PeoplePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(init_people)
            .add_system(hunger_system)
            .add_system(cleanup_system);
    }
}

pub fn init_people(mut commands: Commands, asset_server: Res<AssetServer>) {
    info!("People initialized");
    commands.spawn_bundle(PersonBundle {
        name: Name(String::from("Test guy")),
        type_marker: Person,
        hunger: Hunger(0.0),
        food: FoodAmount(3),
        position: GridCoords { x: 0.0, y: 0.0 },
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
        hunger.0 += config.game.hunger_increase.value;
        if hunger.0 > 1.0 {
            commands.entity(person).insert(Dead);
            commands
                .entity(person)
                .insert(Ttl(config.game.person_ttl.value));
            info!("Person hunger value: {}, person has died", hunger.0);
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
