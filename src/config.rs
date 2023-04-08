use std::fs;

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub const CONFIG_PATH: &str = "./data/config.json";

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct ConfigValue<T> {
    pub value: T,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range: Option<(T, T)>,
}

#[derive(Serialize, Deserialize, Debug, Component)]
pub struct CameraConfig {
    pub move_speed: ConfigValue<f32>,
    pub initial_zoom: ConfigValue<f32>,
    pub zoom_speed: ConfigValue<f32>,
    pub zoom_sensitivity: ConfigValue<f32>,
}

#[derive(Serialize, Deserialize, Debug, Component)]
pub struct GameConfig {
    pub growth: ConfigValue<f32>,
    pub hunger_increase: ConfigValue<f32>,
    pub hunger_decrease: ConfigValue<f32>,
    pub starting_people: ConfigValue<u32>,
    pub max_person_age: ConfigValue<u32>,
    pub person_ttl: ConfigValue<u32>,
    pub food_for_baby: ConfigValue<u32>,
    pub year_length: ConfigValue<u32>,
    pub growing_season_length: ConfigValue<f32>,
}

#[derive(Serialize, Deserialize, Debug, Component)]
pub struct MapConfig {
    pub size_x: ConfigValue<u32>,
    pub size_y: ConfigValue<u32>,
    pub tree_tile_probability: ConfigValue<f32>,
}

#[derive(Serialize, Deserialize, Debug, Component)]
pub struct AiConfig {
    pub food_amount_goal: ConfigValue<u32>,
    pub food_amount_threshold: ConfigValue<f32>,
}

#[derive(Serialize, Deserialize, Debug, Resource)]
pub struct Config {
    pub camera: CameraConfig,
    pub game: GameConfig,
    pub map: MapConfig,
    pub ai: AiConfig,
}

#[derive(Component)]
pub struct GenericConfig {
    pub value: Value,
}

pub struct ConfigPlugin;

impl Plugin for ConfigPlugin {
    fn build(&self, app: &mut App) {
        let data = fs::read_to_string(CONFIG_PATH).expect("Unable to read config file");
        let config: Config = serde_json::from_str(&data).expect("Unable to parse config file");
        debug!("Read configuration: {:?}", config);
        app.insert_resource(config);
    }
}
