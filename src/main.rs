#[macro_use]
extern crate enum_display_derive;
use bevy::{log::LogSettings, prelude::*, render::texture::ImageSettings};
use logic::GameState;

mod config;
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
        .add_plugin(config::ConfigPlugin)
        .add_plugin(debug::DebugPlugin)
        .add_plugin(input::InputPlugin)
        .add_plugin(logic::LogicPlugin)
        .add_plugin(stats::StatsPlugin)
        .add_plugin(rendering::RenderingPlugin)
        .add_state(GameState::ProcessLogic)
        .run();
}
