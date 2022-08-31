use bevy::prelude::{App, Plugin, StartupStage};
use bevy_ecs_tilemap::prelude::*;

use super::{
    camera::init_camera,
    tiles::{randomize_tiles, setup_tiles, update_food_tiles},
};

pub struct RenderingPlugin;

impl Plugin for RenderingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(TilemapPlugin)
            .add_startup_system(setup_tiles)
            .add_startup_system_to_stage(StartupStage::PostStartup, randomize_tiles)
            .add_startup_system_to_stage(StartupStage::PostStartup, init_camera)
            .add_system(update_food_tiles);
    }
}
