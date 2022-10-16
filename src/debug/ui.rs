use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
};
use bevy_egui::{egui, EguiContext};
use bevy_prototype_debug_lines::*;

use crate::config::Config;

pub fn debug_window(
    mut egui_context: ResMut<EguiContext>,
    diagnostics: Res<Diagnostics>,
    config: Res<Config>,
) {
    if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
        if let Some(average) = fps.average() {
            egui::Window::new("Debug").show(egui_context.ctx_mut(), |ui| {
                ui.label(format!("Rendering @{:.1}fps", average));
                ui.label(format!("Theoretical max population: NEED RECALCULATION"))
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
