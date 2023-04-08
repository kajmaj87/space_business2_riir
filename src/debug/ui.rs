use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
};
use bevy_egui::{egui, EguiContexts};
use bevy_prototype_debug_lines::*;

use crate::config::Config;

pub fn debug_window(
    mut egui_context: EguiContexts,
    diagnostics: Res<Diagnostics>,
    config: Res<Config>,
) {
    if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
        if let Some(average) = fps.average() {
            egui::Window::new("Debug").show(egui_context.ctx_mut(), |ui| {
                ui.label(format!("Rendering @{:.1}fps", average));
                ui.label(format!(
                    "Theoretical max population: {:.1}",
                    config.map.size_x.value as f32
                        * config.map.size_y.value as f32
                        * config.map.tree_tile_probability.value
                        * config.game.growth.value
                        * config.game.hunger_decrease.value
                        / config.game.hunger_increase.value
                ))
            });
        }
    }
}

pub fn debug_lines(mut lines: ResMut<DebugLines>) {
    let start = Vec3::splat(-89.0);
    let end = Vec3::splat(29.0);
    let duration = 0.1; // Duration of 0 will show the line for 1 frame.
    lines.line(start, end, duration);
}
