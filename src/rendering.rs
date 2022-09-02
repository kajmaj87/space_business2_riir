mod camera;
mod tiles;
mod ui;

use bevy::prelude::{App, Plugin, StartupStage};
use bevy_ecs_tilemap::prelude::*;
use bevy_egui::EguiPlugin;

pub struct RenderingPlugin;

impl Plugin for RenderingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(TilemapPlugin)
            .add_plugin(EguiPlugin)
            .add_startup_system(tiles::setup_tiles)
            .add_startup_system_to_stage(StartupStage::PostStartup, tiles::randomize_tiles)
            .add_startup_system_to_stage(StartupStage::PostStartup, camera::init_camera)
            .add_system(tiles::update_food_tiles)
            .add_system(ui::food_statistics);
    }
}
