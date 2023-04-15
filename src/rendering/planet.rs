use bevy::prelude::*;

use crate::config::Config;
use crate::logic::VirtualCoords;
use crate::{
    logic::components::{Dead, Person},
    rendering::tiles::TILE_SIZE,
};

pub fn death_system(
    mut query: Query<(&Dead, &mut Handle<Image>), Changed<Dead>>,
    asset_server: Res<AssetServer>,
) {
    for (_, mut image) in query.iter_mut() {
        *image = asset_server.load("dead_person.png");
    }
}

#[allow(clippy::type_complexity)]
pub fn missing_sprite_setter_system(
    mut commands: Commands,
    query: Query<(Entity, &VirtualCoords), (With<Person>, Without<Handle<Image>>)>,
    asset_server: Res<AssetServer>,
    config: Res<Config>,
) {
    for (person, coords) in query.iter() {
        let sprite = SpriteBundle {
            texture: asset_server.load("person.png"),
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
        commands.entity(person).insert(sprite);
    }
}

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
