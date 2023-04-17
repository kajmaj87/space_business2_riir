mod camera;
mod planet;
mod tiles;
pub mod ui;

use crate::stats;
use bevy::app::StartupSet;
use bevy::prelude::{App, IntoSystemConfig, Plugin};
use bevy_ecs_tilemap::prelude::*;
use bevy_egui::EguiPlugin;

pub struct RenderingPlugin;

impl Plugin for RenderingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(TilemapPlugin)
            .add_plugin(EguiPlugin)
            .add_startup_system(tiles::setup_tiles)
            .add_startup_system(tiles::randomize_tiles.in_base_set(StartupSet::PostStartup))
            .add_startup_system(camera::init_camera.in_base_set(StartupSet::PostStartup))
            .add_system(tiles::update_food_tiles)
            .insert_resource(ui::UiState {
                open_settings_panel: ui::SettingsPanel::Game,
            })
            .add_system(ui::settings)
            .add_system(stats::ui::food_statistics)
            .add_system(planet::death_system)
            .add_system(planet::missing_sprite_setter_system)
            .add_system(planet::translation_update_system);
    }
}
