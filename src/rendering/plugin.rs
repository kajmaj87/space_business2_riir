use bevy::prelude::{App, Plugin, StartupStage};
use bevy_ecs_tilemap::prelude::*;
use bevy_egui::EguiPlugin;

use super::{
    camera::init_camera,
    tiles::{randomize_tiles, setup_tiles},
};

pub struct RenderingPlugin;

impl Plugin for RenderingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(TilemapPlugin)
            .add_plugin(EguiPlugin)
            .add_startup_system(setup_tiles)
            .add_startup_system_to_stage(StartupStage::PostStartup, randomize_tiles)
            .add_startup_system_to_stage(StartupStage::PostStartup, init_camera)
            .add_system(super::tiles::update_food_tiles)
            .add_system(super::ui::food_statistics);
    }
}
