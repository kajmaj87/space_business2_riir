use std::collections::HashMap;
use std::{fmt::Display, fs};

use bevy::prelude::*;
use bevy_egui::egui::plot::{Bar, BarChart};
use bevy_egui::egui::Color32;
use bevy_egui::{
    egui::{self, emath::Numeric, Ui},
    EguiContexts,
};
use egui::plot::{Line, PlotPoints};

use crate::config::{Config, ConfigValue, CONFIG_PATH};
use crate::logic::GeometryType;

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
                draw_bool_config_value(ui, &mut config.game.death_lottery);
                draw_bool_config_value(ui, &mut config.game.trade_allowed);
            }),
            SettingsPanel::Map => add_options_grid(ui, |ui| {
                draw_config_value(ui, &mut config.map.size_x);
                draw_config_value(ui, &mut config.map.size_y);
                draw_geometry_type(ui, &mut config.map.geometry);
                draw_config_value(ui, &mut config.map.apple_tree_tile_probability);
                draw_config_value(ui, &mut config.map.orange_tree_tile_probability);
            }),
            SettingsPanel::Ai => add_options_grid(ui, |ui| {
                draw_config_value(ui, &mut config.ai.food_amount_goal);
                draw_config_value(ui, &mut config.ai.food_amount_threshold);
                draw_config_value(ui, &mut config.ai.vision_range);
            }),
        }
    });
}

fn draw_bool_config_value(ui: &mut Ui, value: &mut ConfigValue<bool>) {
    let label = ui.label(&value.name);
    if let Some(hint) = &value.description {
        label.on_hover_text(hint);
    }
    ui.checkbox(&mut value.value, "");
}

fn add_settings_panel(ui: &mut Ui, value: &mut SettingsPanel, label: SettingsPanel) {
    let text = label.to_string();
    ui.selectable_value(value, label, text);
}

pub fn add_options_grid<R>(ui: &mut Ui, f: impl FnOnce(&mut Ui) -> R) {
    egui::Grid::new("options_grid")
        .num_columns(2)
        .spacing([40.0, 4.0])
        .striped(true)
        .show(ui, f);
}

pub fn draw_config_value<T: Numeric>(ui: &mut Ui, value: &mut ConfigValue<T>) {
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

fn draw_geometry_type(ui: &mut Ui, value: &mut ConfigValue<GeometryType>) {
    let label = ui.label(&value.name);
    if let Some(hint) = &value.description {
        label.on_hover_text(hint);
    }
    egui::ComboBox::from_label("")
        .selected_text(value.value.to_string())
        .show_ui(ui, |ui| {
            ui.set_min_width(120.0);
            ui.selectable_value(&mut value.value, GeometryType::Torus, "Torus");
            ui.selectable_value(&mut value.value, GeometryType::FlatEarth, "Flat Earth");
            ui.selectable_value(
                &mut value.value,
                GeometryType::RingVertical,
                "Ring Vertical",
            );
            ui.selectable_value(
                &mut value.value,
                GeometryType::RingHorizontal,
                "Ring Horizontal",
            );
        });
    ui.end_row();
}

pub fn create_plot_line(name: &str, values: &[u32]) -> Line {
    let stats: PlotPoints = values
        .iter()
        .enumerate()
        .map(|(i, v)| [i as f64, *v as f64])
        .collect();

    Line::new(stats).name(name)
}

pub fn create_histogram(name: &str, values: &[u32], bins: u32) -> BarChart {
    let mut histogram = HashMap::new();
    let max = values.iter().max().unwrap_or(&0);
    let min = values.iter().min().unwrap_or(&0);
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
