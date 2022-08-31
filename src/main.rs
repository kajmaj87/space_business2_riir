use bevy::{log::LogSettings, prelude::*, render::texture::ImageSettings};
use bevy_prototype_debug_lines::*;

mod input;
mod logic;
mod rendering;

#[derive(Component)]
struct Name(String);
fn main() {
    App::new()
        .insert_resource(ImageSettings::default_nearest())
        .insert_resource(LogSettings {
            filter: "info,wgpu_core=warn,wgpu_hal=warn,space_business2_riir=info".into(),
            level: bevy::log::Level::DEBUG,
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(DebugLinesPlugin::default())
        .add_plugin(input::plugin::InputPlugin)
        .add_plugin(logic::plugin::LogicPlugin)
        .add_plugin(rendering::plugin::RenderingPlugin)
        .add_system(debug_system)
        .run();
}

fn debug_system(mut lines: ResMut<DebugLines>) {
    let start = Vec3::splat(-89.0);
    let end = Vec3::splat(29.0);
    let duration = 0.1; // Duration of 0 will show the line for 1 frame.
    lines.line(start, end, duration);
}
