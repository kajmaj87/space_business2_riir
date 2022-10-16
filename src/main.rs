#[macro_use]
extern crate enum_display_derive;
use bevy::{log::LogSettings, prelude::*, render::texture::ImageSettings};
use logic::{GameState, TurnPhase, TurnStep};

mod config;
mod debug;
mod input;
mod logic;
mod rendering;
mod stats;

fn main() {
    App::new()
        .insert_resource(ImageSettings::default_nearest())
        .insert_resource(LogSettings {
            filter: "info,wgpu_core=warn,wgpu_hal=warn,space_business2_riir=info".into(),
            level: bevy::log::Level::DEBUG,
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(config::ConfigPlugin)
        .add_plugin(debug::DebugPlugin)
        .add_plugin(input::InputPlugin)
        .add_plugin(logic::LogicPlugin)
        .add_plugin(stats::StatsPlugin)
        .add_plugin(rendering::RenderingPlugin)
        .add_state((TurnPhase::PreparePlanet, TurnStep::Process))
        .add_system_to_stage(CoreStage::Last, state_transision)
        .run();
}

fn state_transision(
    mut game_state: ResMut<State<GameState>>,
    mut keyboard_input: ResMut<Input<KeyCode>>,
    time: Res<Time>,
) {
    use TurnStep::*;
    let current_state = game_state.current().clone();
    let new_state = match current_state {
        s @ (phase, WaitForInput) => {
            if keyboard_input.clear_just_pressed(KeyCode::Return) {
                (next_phase(phase), next_step(WaitForInput, time))
            } else {
                s
            }
        }
        (phase, step) => (phase, next_step(step, time)),
    };
    if current_state != new_state {
        info!("Changing state from {:?} to {:?}", current_state, new_state);
        game_state.set(new_state).unwrap();
    }
}

fn next_phase(phase: TurnPhase) -> TurnPhase {
    use TurnPhase::*;
    match phase {
        PreparePlanet => GenerateJobs,
        GenerateJobs => PreparePlanet,
        _ => PreparePlanet,
    }
}

fn next_step(step: TurnStep, time: Res<Time>) -> TurnStep {
    use TurnStep::*;
    match step {
        Process => Animate(0), // amount of microsecond that the animation shood take
        Animate(microseconds_left) => {
            let delta = (time.delta_seconds() * 1_000_000.0) as u32;
            if microseconds_left > delta {
                Animate(microseconds_left - delta)
            } else {
                WaitForInput
            }
        }
        WaitForInput => Process,
    }
}
