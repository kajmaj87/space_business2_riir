use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};

pub fn food_statistics(mut egui_context: ResMut<EguiContext>) {
    egui::Window::new("Hello").show(egui_context.ctx_mut(), |ui| {
        ui.label("world");
    });
}
