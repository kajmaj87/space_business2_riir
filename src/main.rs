use bevy::{log::LogSettings, prelude::*, render::texture::ImageSettings};

mod debug;
mod input;
mod logic;
mod rendering;
mod stats;

fn main() {
    App::new()
        .insert_resource(ImageSettings::default_nearest())
        .insert_resource(LogSettings {
            filter: "info,wgpu_core=warn,wgpu_hal=warn,space_business2_riir=info".into(),
            level: bevy::log::Level::DEBUG,
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(debug::DebugPlugin)
        .add_plugin(input::InputPlugin)
        .add_plugin(logic::LogicPlugin)
        .add_plugin(stats::StatsPlugin)
        .add_plugin(rendering::RenderingPlugin)
        .run();
}
