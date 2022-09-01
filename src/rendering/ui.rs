use bevy::prelude::*;
use bevy_egui::{
    egui::{
        self,
        plot::{Corner, Legend},
    },
    EguiContext,
};
use egui::plot::{Line, Plot, PlotPoints};

use crate::stats::components::Statistics;

pub fn food_statistics(mut egui_context: ResMut<EguiContext>, stats: Res<Statistics>) {
    let sin: PlotPoints = (0..1000)
        .map(|i| {
            let x = i as f64 * 0.01;
            [x, x.sin()]
        })
        .collect();
    let stats: PlotPoints = stats
        .food_history
        .into_iter()
        .enumerate()
        .map(|(i, v)| [i as f64, v as f64])
        .collect();
    let line = Line::new(sin).name("Apples");
    egui::Window::new("Hello").show(egui_context.ctx_mut(), |ui| {
        ui.label("world");
        Plot::new("my_plot")
            .view_aspect(2.0)
            .legend(Legend {
                position: Corner::RightBottom,
                ..default()
            })
            .show(ui, |plot_ui| plot_ui.line(line));
    });
}
