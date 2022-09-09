use bevy::prelude::*;

use crate::config::Config;

use super::{
    components::{FoodSource, Name, Ttl},
    planet::FoodAmount,
};

#[derive(Component)]
pub struct Hunger(pub f32);

#[derive(Component)]
pub struct Person;

#[derive(Component)]
pub struct Dead;

#[derive(Component)]
pub struct Move {
    pub dx: f32,
    pub dy: f32,
}

#[derive(Component)]
pub struct Forage;

// Position and GridPostion are already defined in bevy::prelude
#[derive(Component, PartialEq)]
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
            .add_system(move_system)
            .add_system(foraging_system)
            .add_system(cleanup_system);
    }
}

pub fn init_people(mut commands: Commands, asset_server: Res<AssetServer>, config: Res<Config>) {
    info!("People initialized");
    for _ in 0..config.game.starting_people.value {
        commands.spawn_bundle(PersonBundle {
            name: Name(String::from("Test guy")),
            type_marker: Person,
            hunger: Hunger(0.0),
            food: FoodAmount(3),
            position: GridCoords { x: 5.0, y: 3.0 },
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

fn move_system(
    mut commands: Commands,
    mut query: Query<(Entity, &Move, &mut GridCoords)>,
    config: Res<Config>,
) {
    for (person, move_component, mut coords) in query.iter_mut() {
        commands.entity(person).remove::<Move>();
        let newx = move_component.dx + coords.x;
        let newy = move_component.dy + coords.y;
        if 0.0 <= newx && newx <= config.map.size_x.value as f32 - 1.0 {
            coords.x = newx;
        }
        if 0.0 <= newy && newy <= config.map.size_y.value as f32 - 1.0 {
            coords.y = newy;
        }
    }
}

#[allow(clippy::type_complexity)]
fn foraging_system(
    mut commands: Commands,
    mut people: Query<
        (Entity, &mut FoodAmount, &GridCoords),
        (Changed<Forage>, With<Person>, With<Forage>),
    >,
    mut food_producers: Query<(&mut FoodAmount, &GridCoords), (With<FoodSource>, Without<Person>)>,
) {
    for (person, mut person_food_amount, coords) in people.iter_mut() {
        for (mut food_producer_amount, food_coords) in food_producers.iter_mut() {
            if coords == food_coords && food_producer_amount.0 > 0 {
                info!("Found some food!");
                person_food_amount.0 += 1;
                food_producer_amount.0 -= 1;
                commands.entity(person).remove::<Forage>();
            }
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
