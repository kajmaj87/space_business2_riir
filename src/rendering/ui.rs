use std::collections::HashMap;
use std::{fmt::Display, fs};

use bevy::prelude::*;
use bevy_egui::egui::plot::{Bar, BarChart};
use bevy_egui::egui::Color32;
use bevy_egui::{
    egui::{
        self,
        emath::Numeric,
        plot::{Corner, Legend},
        Ui,
    },
    EguiContexts,
};
use egui::plot::{Line, Plot, PlotPoints};

use crate::logic::components::{Age, Dead, Person};
use crate::{
    config::{Config, ConfigValue, CONFIG_PATH},
    stats::components::Statistics,
};

#[derive(PartialEq, Eq, Display)]
pub enum SettingsPanel {
    Camera,
    Game,
    Map,
    Ai,
}

#[derive(Resource)]
pub struct UiState {
    pub open_settings_panel: SettingsPanel,
}

pub fn settings(
    mut egui_context: EguiContexts,
    mut config: ResMut<Config>,
    mut state: ResMut<UiState>,
) {
    egui::Window::new("Config").show(egui_context.ctx_mut(), |ui| {
        ui.collapsing("Instructions", |ui| {
            ui.label("Most of the values you adjust here will take effect immediately.");
            ui.label("You can hover over the option name to see an extended tooltip of what it does.");
            ui.label("If you wish to change the value precisely you can drag the numeric value or double click to edit it.");
            ui.label(format!("If range of the values is too small you can edit the {} file and edit the matching \"range\" entry or you can just remove it completely.", CONFIG_PATH));
        });
        ui.horizontal(|ui| {
            add_settings_panel(ui, &mut state.open_settings_panel, SettingsPanel::Game);
            add_settings_panel(ui, &mut state.open_settings_panel, SettingsPanel::Camera);
            add_settings_panel(ui, &mut state.open_settings_panel, SettingsPanel::Map);
            add_settings_panel(ui, &mut state.open_settings_panel, SettingsPanel::Ai);
            let space_left = ui.available_size() - egui::Vec2 { x: 45.0, y: 0.0 };
            ui.allocate_space(space_left);
            if ui.button("Save").clicked() {
                let file_content = serde_json::to_string_pretty(config.as_ref())
                    .expect("Unable to serialize configuration for saving!");
                fs::write(CONFIG_PATH, file_content).expect("Unable to save config data!");
            }
        });
        ui.separator();
        match state.open_settings_panel {
            SettingsPanel::Camera => add_options_grid(ui, |ui| {
                draw_config_value(ui, &mut config.camera.move_speed);
                draw_config_value(ui, &mut config.camera.initial_zoom);
                draw_config_value(ui, &mut config.camera.zoom_speed);
                draw_config_value(ui, &mut config.camera.zoom_sensitivity);
            }),
            SettingsPanel::Game => add_options_grid(ui, |ui| {
                draw_config_value(ui, &mut config.game.growth);
                draw_config_value(ui, &mut config.game.hunger_increase);
                draw_config_value(ui, &mut config.game.hunger_decrease);
                draw_config_value(ui, &mut config.game.starting_people);
                draw_config_value(ui, &mut config.game.max_person_age);
                draw_config_value(ui, &mut config.game.food_for_baby);
                draw_config_value(ui, &mut config.game.person_ttl);
                draw_config_value(ui, &mut config.game.year_length);
                draw_config_value(ui, &mut config.game.growing_season_length);
            }),
            SettingsPanel::Map => add_options_grid(ui, |ui| {
                draw_config_value(ui, &mut config.map.size_x);
                draw_config_value(ui, &mut config.map.size_y);
                draw_config_value(ui, &mut config.map.tree_tile_probability);
            }),
            SettingsPanel::Ai => add_options_grid(ui, |ui| {
                draw_config_value(ui, &mut config.ai.food_amount_goal);
                draw_config_value(ui, &mut config.ai.food_amount_threshold);
            }),
        }
    });
}

fn add_settings_panel(ui: &mut Ui, value: &mut SettingsPanel, label: SettingsPanel) {
    let text = label.to_string();
    ui.selectable_value(value, label, text);
}

fn add_options_grid<R>(ui: &mut Ui, f: impl FnOnce(&mut Ui) -> R) {
    egui::Grid::new("options_grid")
        .num_columns(2)
        .spacing([40.0, 4.0])
        .striped(true)
        .show(ui, f);
}

fn draw_config_value<T: Numeric>(ui: &mut Ui, value: &mut ConfigValue<T>) {
    let label = ui.label(&value.name);
    if let Some(hint) = &value.description {
        label.on_hover_text(hint);
    }
    if let Some((min, max)) = value.range {
        ui.add(egui::Slider::new(&mut value.value, min..=max));
    } else {
        ui.add(egui::DragValue::new(&mut value.value).speed(0.1));
    }
    ui.end_row();
}

pub fn food_statistics(
    mut egui_context: EguiContexts,
    stats: Res<Statistics>,
    mut config: ResMut<Config>,
    query: Query<(&Person, &Age), Without<Dead>>,
) {
    let apple_range = get_range(&stats.apple_history, config.ui.plot_time_range.value);
    let orange_range = get_range(&stats.orange_history, config.ui.plot_time_range.value);
    let people_range = get_range(&stats.people_history, config.ui.plot_time_range.value);

    let apples = &stats.apple_history.as_slice()[apple_range..];
    let oranges = &stats.orange_history.as_slice()[orange_range..];
    let people = &stats.people_history.as_slice()[people_range..];
    let apple_line = create_plot_line("Apples", apples).color(Color32::RED);
    let orange_line = create_plot_line("Oranges", oranges).color(Color32::from_rgb(255, 165, 0));
    let people_line = create_plot_line("People", people);
    let ages = query.iter().map(|(_, age)| age.0).collect::<Vec<_>>();
    egui::Window::new("Plots").show(egui_context.ctx_mut(), |ui| {
        ui.label("Foods and people over time");
        add_options_grid(ui, |ui| {
            draw_config_value(ui, &mut config.ui.plot_time_range);
            draw_config_value(ui, &mut config.ui.age_histogram_bins);
        });
        Plot::new("foods")
            .view_aspect(2.0)
            .legend(Legend {
                position: Corner::LeftTop,
                ..default()
            })
            .show(ui, |plot_ui| {
                plot_ui.line(apple_line);
                plot_ui.line(orange_line);
            });
        Plot::new("people")
            .view_aspect(2.0)
            .legend(Legend {
                position: Corner::LeftTop,
                ..default()
            })
            .show(ui, |plot_ui| {
                plot_ui.line(people_line);
            });
        Plot::new("ages")
            .view_aspect(2.0)
            .legend(Legend {
                position: Corner::LeftTop,
                ..default()
            })
            .show(ui, |plot_ui| {
                plot_ui.bar_chart(create_histogram(
                    "Ages",
                    &ages,
                    config.ui.age_histogram_bins.value,
                ));
            });
    });
}

fn get_range(vector: &Vec<u32>, last_n: usize) -> usize {
    if vector.len() > last_n {
        vector.len() - last_n
    } else {
        0
    }
}

fn create_plot_line(name: &str, values: &[u32]) -> Line {
    let stats: PlotPoints = values
        .iter()
        .enumerate()
        .map(|(i, v)| [i as f64, *v as f64])
        .collect();

    Line::new(stats).name(name)
}

fn create_histogram(name: &str, values: &[u32], bins: u32) -> BarChart {
    let mut histogram = HashMap::new();
    let max = values.iter().max().unwrap();
    let min = values.iter().min().unwrap();
    let range = max - min + 1;
    let bin_width = (range as f64 / bins as f64).ceil() as u32;
    for &value in values {
        *histogram.entry((value - min) / bin_width).or_insert(0) += 1;
    }
    let histogram: Vec<Bar> = histogram
        .into_iter()
        .map(|(bin, count)| {
            Bar::new((bin * bin_width + min) as f64, count as f64).width(bin_width as f64)
        })
        .collect();
    BarChart::new(histogram)
        .color(Color32::LIGHT_BLUE)
        .name(name)
}
