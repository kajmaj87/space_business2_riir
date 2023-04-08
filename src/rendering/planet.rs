use bevy::prelude::*;

use crate::{
    logic::components::{Dead, GridCoords, Person},
    rendering::tiles::TILE_SIZE,
};

pub fn death_system(
    mut query: Query<(&Person, &mut Handle<Image>), Changed<Dead>>,
    asset_server: Res<AssetServer>,
) {
    for (_, mut image) in query.iter_mut() {
        *image = asset_server.load("dead_person.png");
    }
}

pub fn missing_sprite_setter_system(
    mut commands: Commands,
    query: Query<Entity, (With<Person>, Without<Handle<Image>>)>,
    asset_server: Res<AssetServer>,
) {
    for person in query.iter() {
        let sprite = SpriteBundle {
            texture: asset_server.load("person.png"),
            transform: Transform {
                translation: Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 2.0,
                },
                ..Default::default()
            },
            ..Default::default()
        };
        commands.entity(person).insert(sprite);
    }
}

pub fn translation_update_system(
    mut query: Query<(&GridCoords, &mut Transform), Changed<GridCoords>>,
) {
    for (coords, mut transform) in query.iter_mut() {
        transform.translation = Vec3 {
            x: coords.x as f32 * TILE_SIZE,
            y: coords.y as f32 * TILE_SIZE,
            z: transform.translation.z,
        };
    }
}
