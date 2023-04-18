use crate::debug::components::Performance;
use bevy::prelude::*;

use crate::config::Config;
use crate::logic::people::{Child, Female, Fertile, Male, Old};
use crate::logic::VirtualCoords;
use crate::{
    logic::components::{Dead, Person},
    rendering::tiles::TILE_SIZE,
};
use macros::measured;

#[measured]
pub fn death_system(
    mut query: Query<(Entity, &Dead, &mut Handle<Image>), Added<Dead>>,
    child: Query<&Child>,
    asset_server: Res<AssetServer>,
) {
    for (entity, _, mut image) in query.iter_mut() {
        if child.get(entity).is_ok() {
            *image = asset_server.load("dead_child.png");
        } else {
            *image = asset_server.load("dead_person.png");
        }
    }
}

#[allow(clippy::type_complexity)]
#[allow(clippy::too_many_arguments)]
#[measured]
pub fn render_aging_system(
    mut changed_males: Query<
        (Entity, &Person, &mut Handle<Image>),
        (With<Male>, Or<(Added<Child>, Added<Fertile>, Added<Old>)>),
    >,
    mut changed_females: Query<
        (Entity, &Person, &mut Handle<Image>),
        (
            Without<Male>,
            Or<(Added<Child>, Added<Fertile>, Added<Old>)>,
        ),
    >,
    asset_server: Res<AssetServer>,
    child: Query<&Child>,
    granny: Query<&Female, With<Old>>,
    grandpa: Query<&Male, With<Old>>,
    female: Query<&Female, Without<Old>>,
    male: Query<&Male, Without<Old>>,
) {
    for (entity, _, image) in changed_males.iter_mut() {
        update_person_img(
            &asset_server,
            &child,
            &granny,
            &grandpa,
            &female,
            &male,
            entity,
            image,
        );
    }
    for (entity, _, image) in changed_females.iter_mut() {
        update_person_img(
            &asset_server,
            &child,
            &granny,
            &grandpa,
            &female,
            &male,
            entity,
            image,
        );
    }
}

#[allow(clippy::too_many_arguments)]
fn update_person_img(
    asset_server: &Res<AssetServer>,
    child: &Query<&Child>,
    granny: &Query<&Female, With<Old>>,
    grandpa: &Query<&Male, With<Old>>,
    female: &Query<&Female, Without<Old>>,
    male: &Query<&Male, Without<Old>>,
    person: Entity,
    mut image: Mut<Handle<Image>>,
) {
    if granny.get(person).is_ok() {
        *image = asset_server.load("granny.png");
    }
    if grandpa.get(person).is_ok() {
        *image = asset_server.load("grandpa.png");
    }
    if male.get(person).is_ok() {
        *image = asset_server.load("male.png");
    }
    if female.get(person).is_ok() {
        *image = asset_server.load("female.png");
    }
    if child.get(person).is_ok() {
        *image = asset_server.load("child.png");
    }
}

#[allow(clippy::too_many_arguments)]
fn update_person_texture(
    asset_server: &Res<AssetServer>,
    child: &Query<&Child>,
    granny: &Query<&Female, With<Old>>,
    grandpa: &Query<&Male, With<Old>>,
    female: &Query<&Female, Without<Old>>,
    male: &Query<&Male, Without<Old>>,
    person: Entity,
    sprite: &mut SpriteBundle,
) {
    if granny.get(person).is_ok() {
        sprite.texture = asset_server.load("granny.png");
    }
    if grandpa.get(person).is_ok() {
        sprite.texture = asset_server.load("grandpa.png");
    }
    if male.get(person).is_ok() {
        sprite.texture = asset_server.load("male.png");
    }
    if female.get(person).is_ok() {
        sprite.texture = asset_server.load("female.png");
    }
    if child.get(person).is_ok() {
        sprite.texture = asset_server.load("child.png");
    }
}

#[allow(clippy::type_complexity)]
#[allow(clippy::too_many_arguments)]
#[measured]
pub fn missing_sprite_setter_system(
    mut commands: Commands,
    query: Query<(Entity, &VirtualCoords), (With<Person>, Without<Handle<Image>>)>,
    asset_server: Res<AssetServer>,
    child: Query<&Child>,
    granny: Query<&Female, With<Old>>,
    grandpa: Query<&Male, With<Old>>,
    female: Query<&Female, Without<Old>>,
    male: Query<&Male, Without<Old>>,
    config: Res<Config>,
) {
    for (person, coords) in query.iter() {
        let mut sprite = SpriteBundle {
            transform: Transform {
                translation: Vec3 {
                    x: coords.to_real(&config).x as f32 * TILE_SIZE,
                    y: coords.to_real(&config).y as f32 * TILE_SIZE,
                    z: 2.0,
                },
                ..Default::default()
            },
            ..Default::default()
        };
        update_person_texture(
            &asset_server,
            &child,
            &granny,
            &grandpa,
            &female,
            &male,
            person,
            &mut sprite,
        );
        commands.entity(person).insert(sprite);
    }
}

#[measured]
pub fn translation_update_system(
    mut query: Query<(&VirtualCoords, &mut Transform), Changed<VirtualCoords>>,
    config: Res<Config>,
) {
    for (coords, mut transform) in query.iter_mut() {
        transform.translation = Vec3 {
            x: coords.to_real(&config).x as f32 * TILE_SIZE,
            y: coords.to_real(&config).y as f32 * TILE_SIZE,
            z: transform.translation.z,
        };
    }
}
