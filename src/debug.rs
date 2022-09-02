mod ui;

use bevy::{
    diagnostic::FrameTimeDiagnosticsPlugin,
    prelude::{App, Plugin},
};
use bevy_prototype_debug_lines::DebugLinesPlugin;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(FrameTimeDiagnosticsPlugin::default())
            .add_plugin(DebugLinesPlugin::default())
            .add_system(ui::debug_window)
            .add_system(ui::debug_lines);
    }
}
