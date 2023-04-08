#[macro_use]
extern crate enum_display_derive;
#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;

use bevy::log::LogPlugin;
use bevy::prelude::*;

mod config;
mod debug;
mod input;
mod logic;
mod rendering;
mod stats;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(LogPlugin {
                    filter: "info,wgpu_core=warn,wgpu_hal=warn,space_business2_riir=info".into(),
                    level: bevy::log::Level::DEBUG,
                }),
        )
        .add_plugin(config::ConfigPlugin)
        .add_plugin(debug::DebugPlugin)
        .add_plugin(input::InputPlugin)
        .add_plugin(logic::LogicPlugin)
        .add_plugin(stats::StatsPlugin)
        .add_plugin(rendering::RenderingPlugin)
        .run();
}
