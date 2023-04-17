use crate::config::Config;
use crate::logic::components::{Age, Dead, FoodAmount, Person};
use crate::logic::people::{Female, Fertile, Male};
use crate::rendering::ui::{
    add_options_grid, create_histogram, create_plot_line, draw_config_value,
};
use bevy::prelude::*;
use bevy_egui::egui::plot::{Corner, Legend, Plot};
use bevy_egui::egui::{Color32, Ui};
use bevy_egui::{egui, EguiContexts};

use crate::stats::economy::Statistics;

pub fn stats_window(
    mut egui_context: EguiContexts,
    stats: Res<Statistics>,
    males_fertile: Query<&Male, With<Fertile>>,
    females_fertile: Query<&Female, With<Fertile>>,
    males_infertile: Query<&Male, Without<Fertile>>,
    females_infertile: Query<&Female, Without<Fertile>>,
    people_wealth: Query<&FoodAmount, With<Person>>,
) {
    egui::Window::new("Stats").show(egui_context.ctx_mut(), |ui| {
        let males = males_fertile.iter().count() + males_infertile.iter().count();
        let females = females_fertile.iter().count() + females_infertile.iter().count();
        ui.label(format!("People: {}", stats.current_people));
        ui.label(format!(
            "Males (fertile): {} ({})",
            males,
            males_fertile.iter().count()
        ));
        ui.label(format!(
            "Females (fertile): {} ({})",
            females,
            females_fertile.iter().count()
        ));
        ui.label(format!("Food: {}", stats.current_food));
        ui.label(format!("Apples: {}", stats.current_apples));
        ui.label(format!("Oranges: {}", stats.current_oranges));
        ui.label(format!(
            "Gini Coefficient: {:.3}",
            calculate_gini_coefficient(
                &people_wealth
                    .iter()
                    .map(|f| f.apples as f64 + f.oranges as f64)
                    .collect::<Vec<_>>()
            )
        ));
    });
}

fn calculate_gini_coefficient(food_quantities: &[f64]) -> f64 {
    let n = food_quantities.len();
    let total_food: f64 = food_quantities.iter().sum();

    // Sort food quantities in ascending order
    let mut sorted_quantities = food_quantities.to_vec();
    sorted_quantities.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let mut cum_proportion_sum = 0.0;
    let mut gini_numerator = 0.0;

    for (i, value) in sorted_quantities.iter().enumerate() {
        let p_i = (i + 1) as f64 / n as f64;
        let c_i_proportion = value / total_food;

        cum_proportion_sum += c_i_proportion;
        gini_numerator += p_i * c_i_proportion;
    }

    1.0 - 2.0 * (gini_numerator - 0.5 * cum_proportion_sum)
}

pub fn food_statistics(
    mut egui_context: EguiContexts,
    stats: Res<Statistics>,
    mut config: ResMut<Config>,
    query: Query<(&Person, &Age), Without<Dead>>,
) {
    egui::Window::new("Plots").show(egui_context.ctx_mut(), |ui| {
        ui.label("Foods and people over time");
        add_options_grid(ui, |ui| {
            draw_config_value(ui, &mut config.ui.plot_time_range);
            draw_config_value(ui, &mut config.ui.age_histogram_bins);
        });
        plot_food_on_planet(&stats, &mut config, ui);
        plot_food_for_people(&stats, &mut config, ui);
        plot_people(&stats, &mut config, ui);
        plot_ages(config, query, ui);
    });
}

fn plot_food_on_planet(stats: &Res<Statistics>, config: &mut ResMut<Config>, ui: &mut Ui) {
    let apple_range = get_range(
        &stats.apple_history_sources,
        config.ui.plot_time_range.value,
    );
    let orange_range = get_range(
        &stats.orange_history_sources,
        config.ui.plot_time_range.value,
    );
    let apples = &stats.apple_history_sources.as_slice()[apple_range..];
    let oranges = &stats.orange_history_sources.as_slice()[orange_range..];
    let apple_line = create_plot_line("Apples", apples).color(Color32::RED);
    let orange_line = create_plot_line("Oranges", oranges).color(Color32::from_rgb(255, 165, 0));
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
}

fn plot_food_for_people(stats: &Res<Statistics>, config: &mut ResMut<Config>, ui: &mut Ui) {
    let apple_range_people =
        get_range(&stats.apple_history_people, config.ui.plot_time_range.value);
    let orange_range_people = get_range(
        &stats.orange_history_people,
        config.ui.plot_time_range.value,
    );
    let apples_people = &stats.apple_history_people.as_slice()[apple_range_people..];
    let oranges_people = &stats.orange_history_people.as_slice()[orange_range_people..];
    let apple_line_people = create_plot_line("Apples (people)", apples_people).color(Color32::RED);
    let orange_line_people =
        create_plot_line("Oranges (people)", oranges_people).color(Color32::from_rgb(255, 165, 0));
    Plot::new("foods_people")
        .view_aspect(2.0)
        .legend(Legend {
            position: Corner::LeftTop,
            ..default()
        })
        .show(ui, |plot_ui| {
            plot_ui.line(apple_line_people);
            plot_ui.line(orange_line_people);
        });
}

fn plot_people(stats: &Res<Statistics>, config: &mut ResMut<Config>, ui: &mut Ui) {
    let people_range = get_range(&stats.people_history, config.ui.plot_time_range.value);
    let people = &stats.people_history.as_slice()[people_range..];
    let people_line = create_plot_line("People", people);
    Plot::new("people")
        .view_aspect(2.0)
        .legend(Legend {
            position: Corner::LeftTop,
            ..default()
        })
        .show(ui, |plot_ui| {
            plot_ui.line(people_line);
        });
}

fn plot_ages(config: ResMut<Config>, query: Query<(&Person, &Age), Without<Dead>>, ui: &mut Ui) {
    let ages = query.iter().map(|(_, age)| age.0).collect::<Vec<_>>();
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
}

fn get_range(vector: &Vec<u32>, last_n: usize) -> usize {
    if vector.len() > last_n {
        vector.len() - last_n
    } else {
        0
    }
}
