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

pub fn translation_update_system(
    mut query: Query<(&GridCoords, &mut Transform), Changed<GridCoords>>,
) {
    for (coords, mut transform) in query.iter_mut() {
        transform.translation = Vec3 {
            x: coords.x * TILE_SIZE + TILE_SIZE / 2.0,
            y: coords.y * TILE_SIZE + TILE_SIZE / 2.0,
            z: transform.translation.z,
        };
    }
}
