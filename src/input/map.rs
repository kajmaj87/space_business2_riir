use bevy::{
    input::{
        mouse::{MouseScrollUnit, MouseWheel},
        Input,
    },
    math::Vec3,
    prelude::*,
    render::camera::Camera,
};

use crate::config::Config;

const MAX_ZOOM: f32 = 3.0;
const MIN_ZOOM: f32 = 0.25;

// A simple camera system for moving and zooming the camera.
pub fn movement(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut mouse_scroll: EventReader<MouseWheel>,
    mut query: Query<(&mut Transform, &mut OrthographicProjection), With<Camera>>,
    config: Res<Config>,
) {
    for (mut transform, mut ortho) in query.iter_mut() {
        for ev in mouse_scroll.iter() {
            let unit = match ev.unit {
                MouseScrollUnit::Line => "line units",
                MouseScrollUnit::Pixel => "pixel units",
            };
            debug!(
                "Scroll ({}): vertical: {}, horizontal: {}",
                unit, ev.y, ev.x
            );
            ortho.scale += -ev.y * config.camera.zoom_sensitivity.value;
        }
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::A) {
            direction -= Vec3::new(1.0, 0.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::D) {
            direction += Vec3::new(1.0, 0.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::W) {
            direction += Vec3::new(0.0, 1.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::S) {
            direction -= Vec3::new(0.0, 1.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::Z) {
            ortho.scale += config.camera.zoom_speed.value;
        }

        if keyboard_input.pressed(KeyCode::X) {
            ortho.scale -= config.camera.zoom_speed.value;
        }

        if ortho.scale < MIN_ZOOM {
            ortho.scale = MIN_ZOOM;
        }

        if ortho.scale > MAX_ZOOM {
            ortho.scale = MAX_ZOOM;
        }

        let z = transform.translation.z;
        transform.translation += time.delta_seconds() * direction * config.camera.move_speed.value;
        // Important! We need to restore the Z values when moving the camera around.
        // Bevy has a specific camera setup and this can mess with how our layers are shown.
        transform.translation.z = z;
    }
}
