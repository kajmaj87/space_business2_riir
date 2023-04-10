use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use crate::stats::economy::Statistics;

pub fn stats_window(mut egui_context: EguiContexts, stats: Res<Statistics>) {
    egui::Window::new("Stats").show(egui_context.ctx_mut(), |ui| {
        ui.label(format!("Current people: {}", stats.current_people));
        ui.label(format!("Current food: {}", stats.current_food));
        ui.label(format!("Current apples: {}", stats.current_apples));
        ui.label(format!("Current oranges: {}", stats.current_oranges));
    });
}
