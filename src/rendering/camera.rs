use bevy::{prelude::*, render::camera::Camera};

pub fn init_camera(mut query: Query<&mut OrthographicProjection, With<Camera>>) {
    for mut ortho in query.iter_mut() {
        ortho.scale = 0.5;
    }
}
