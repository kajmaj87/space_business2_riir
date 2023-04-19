use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
};
use bevy_egui::{egui, EguiContexts};
use egui_extras::{Column, TableBuilder};
// use bevy_prototype_debug_lines::*;

use crate::debug::components::Performance;

pub fn debug_window(
    mut egui_context: EguiContexts,
    diagnostics: Res<Diagnostics>,
    performance: Res<Performance>,
) {
    if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
        if let Some(average) = fps.average() {
            egui::Window::new("Debug").show(egui_context.ctx_mut(), |ui| {
                ui.label(format!("Rendering @{:.1}fps", average));
                ui.collapsing("Performance Stats", |ui| {
                    TableBuilder::new(ui)
                        .striped(true)
                        .resizable(true)
                        .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                        .column(Column::auto())
                        .column(Column::auto())
                        .column(Column::auto())
                        .column(Column::auto())
                        .column(Column::auto())
                        .column(Column::auto())
                        .column(Column::remainder())
                        .min_scrolled_height(0.0)
                        .header(20.0, |mut header| {
                            header.col(|ui| {
                                ui.strong("System");
                            });
                            header.col(|ui| {
                                ui.strong("Total Time (%)");
                            });
                            header.col(|ui| {
                                ui.strong("Min");
                            });
                            header.col(|ui| {
                                ui.strong("p5");
                            });
                            header.col(|ui| {
                                ui.strong("median");
                            });
                            header.col(|ui| {
                                ui.strong("p95");
                            });
                            header.col(|ui| {
                                ui.strong("max");
                            });
                        })
                        .body(|mut body| {
                            for f in performance.describe_all() {
                                body.row(18.0, |mut row| {
                                    row.col(|ui| {
                                        ui.label(f.name);
                                    });
                                    row.col(|ui| {
                                        ui.label(format!("{:.2}", f.total_duration));
                                    });
                                    row.col(|ui| {
                                        ui.label(format!("{:#?}", f.min));
                                    });
                                    row.col(|ui| {
                                        ui.label(format!("{:#?}", f.p5));
                                    });
                                    row.col(|ui| {
                                        ui.label(format!("{:#?}", f.median));
                                    });
                                    row.col(|ui| {
                                        ui.label(format!("{:#?}", f.p95));
                                    });
                                    row.col(|ui| {
                                        ui.label(format!("{:#?}", f.max));
                                    });
                                });
                            }
                        });
                });
            });
        }
    }
}
//
// pub fn debug_lines(mut lines: ResMut<DebugLines>) {
//     let start = Vec3::splat(-89.0);
//     let end = Vec3::splat(29.0);
//     let duration = 0.1; // Duration of 0 will show the line for 1 frame.
//     lines.line(start, end, duration);
// }
