use bevy::{prelude::*, render::camera::Camera};

use crate::config::Config;

use super::tiles::TILE_SIZE;

pub fn init_camera(
    mut query: Query<(&mut Transform, &mut OrthographicProjection), With<Camera>>,
    config: Res<Config>,
) {
    for (mut transform, mut ortho) in query.iter_mut() {
        ortho.scale = config.camera.initial_zoom.value;
        transform.translation = Vec3 {
            x: config.map.size_x.value as f32 * TILE_SIZE / 2.0,
            y: config.map.size_y.value as f32 * TILE_SIZE / 2.0,
            z: transform.translation.z,
        }
    }
}
