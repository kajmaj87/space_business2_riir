use crate::config::Config;
use crate::logic::components::{Age, Dead, FoodAmount, FoodSource, FoodType, Person};
use crate::logic::people::{Female, Fertile, Male};
use crate::rendering::ui::{
    add_options_grid, create_histogram, create_plot_line, create_plot_line_f64, draw_config_value,
};
use crate::stats::components::Transaction;
use bevy::prelude::*;
use bevy_egui::egui::plot::{Corner, Legend, Plot, PlotPoints, Points};
use bevy_egui::egui::{Color32, Ui};
use bevy_egui::{egui, EguiContexts};

use crate::stats::economy::Statistics;

#[allow(clippy::too_many_arguments)]
pub fn stats_window(
    mut egui_context: EguiContexts,
    stats: Res<Statistics>,
    total_entities: Query<Entity>,
    males_fertile: Query<&Male, With<Fertile>>,
    females_fertile: Query<&Female, With<Fertile>>,
    males_infertile: Query<&Male, Without<Fertile>>,
    females_infertile: Query<&Female, Without<Fertile>>,
    people_wealth: Query<&FoodAmount, With<Person>>,
    food_sources: Query<(&FoodSource, &FoodAmount)>,
    people: Query<(&Person, &Age)>,
    config: Res<Config>,
) {
    egui::Window::new("Stats").show(egui_context.ctx_mut(), |ui| {
        let males = males_fertile.iter().count() + males_infertile.iter().count();
        let females = females_fertile.iter().count() + females_infertile.iter().count();
        ui.label(format!("Total entities: {}", total_entities.iter().count()));
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
        let mut growing_apple_trees = 0;
        let mut growing_orange_trees = 0;
        food_sources
            .iter()
            .for_each(|(food_source, food_amount)| match food_source.0 {
                FoodType::Apple => {
                    if food_amount.apples < 3 {
                        growing_apple_trees += 1
                    } else {
                    }
                }
                FoodType::Orange => {
                    if food_amount.oranges < 3 {
                        growing_orange_trees += 1
                    } else {
                    }
                }
            });
        ui.label(format!(
            "Average apple growth: {:.1}",
            growing_apple_trees as f32
                * config.game.growth.value
                * config.game.growing_season_length.value
        ));
        ui.label(format!(
            "Average orange growth: {:.1}",
            growing_orange_trees as f32
                * config.game.growth.value
                * config.game.growing_season_length.value
        ));
        ui.label(format!(
            "Average consumption of each food: {:.1}",
            people.iter().count() as f32 * config.game.hunger_increase.value
        ));
        ui.label(format!(
            "Average age: {:.0}",
            people.iter().map(|(_, age)| { age.0 }).sum::<u32>() as f32
                / people.iter().count() as f32
        ));
        ui.label(format!(
            "Average apple trade volume in last 100 ticks: {:.0}",
            get_range(&stats.trade_history, 100)
                .iter()
                .map(|t| { t.apples })
                .sum::<u32>() as f32
                / get_range(&stats.trade_history, 100).len() as f32
        ));
        ui.label(format!(
            "Average orange trade volume in last 100 ticks: {:.0}",
            get_range(&stats.trade_history, 100)
                .iter()
                .map(|t| { t.oranges })
                .sum::<u32>() as f32
                / get_range(&stats.trade_history, 100).len() as f32
        ));
        ui.label(format!(
            "Average orange trade price in last 100 ticks: {:.2}",
            get_range(&stats.trade_history, 100)
                .iter()
                .map(|t| { t.apples as f32 / t.oranges as f32 })
                .sum::<f32>()
                / get_range(&stats.trade_history, 100).len() as f32
        ));
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
        plot_ages(&mut config, query, ui);
        plot_transactions(&mut config, &stats.trade_history, ui);
        plot_orange_price(&mut config, &stats.trade_history, ui);
    });
}

fn plot_food_on_planet(stats: &Res<Statistics>, config: &mut ResMut<Config>, ui: &mut Ui) {
    let apples = get_range(
        &stats.apple_history_sources,
        config.ui.plot_time_range.value,
    );
    let oranges = get_range(
        &stats.orange_history_sources,
        config.ui.plot_time_range.value,
    );
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
    let apples_people = get_range(&stats.apple_history_people, config.ui.plot_time_range.value);
    let oranges_people = get_range(
        &stats.orange_history_people,
        config.ui.plot_time_range.value,
    );
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
    let people = get_range(&stats.people_history, config.ui.plot_time_range.value);
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

fn plot_ages(
    config: &mut ResMut<Config>,
    query: Query<(&Person, &Age), Without<Dead>>,
    ui: &mut Ui,
) {
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

fn plot_transactions(_config: &mut ResMut<Config>, transactions: &Vec<Transaction>, ui: &mut Ui) {
    Plot::new("transactions")
        .view_aspect(2.0)
        .legend(Legend {
            position: Corner::LeftTop,
            ..default()
        })
        .show(ui, |plot_ui| {
            let points: PlotPoints = get_range(transactions, 1000)
                .iter()
                .map(|transaction| {
                    // random number between -1.0 and 1.0
                    let random_x = rand::random::<f64>() * 2.0 - 1.0;
                    let random_y = rand::random::<f64>() * 2.0 - 1.0;
                    [
                        transaction.apples as f64 + random_x,
                        transaction.oranges as f64 + random_y,
                    ]
                })
                .collect();
            plot_ui.points(Points::new(points));
        });
}

fn plot_orange_price(config: &mut ResMut<Config>, transactions: &Vec<Transaction>, ui: &mut Ui) {
    let price = get_range(transactions, config.ui.plot_time_range.value)
        .iter()
        .map(|t| t.apples as f64 / t.oranges as f64)
        .filter(|p| p.is_finite())
        .collect::<Vec<_>>();
    let price_line = create_plot_line_f64("Price", price.as_slice());
    Plot::new("price")
        .view_aspect(2.0)
        .legend(Legend {
            position: Corner::LeftTop,
            ..default()
        })
        .show(ui, |plot_ui| {
            plot_ui.line(price_line);
        });
}

fn get_range<T>(vector: &Vec<T>, last_n: usize) -> &[T] {
    let range = if vector.len() > last_n {
        vector.len() - last_n
    } else {
        0
    };
    &vector.as_slice()[range..]
}
