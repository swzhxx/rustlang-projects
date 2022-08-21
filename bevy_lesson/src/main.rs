use std::sync::Arc;

use bevy::prelude::*;
use bevy_inspector_egui::prelude::*;
fn main() {
    // println!("Hello, world!");

    App::new()
        .add_plugins(DefaultPlugins)
        
        .add_plugin(bevy::diagnostic::FrameTimeDiagnosticsPlugin)
        .add_plugin(bevy::diagnostic::EntityCountDiagnosticsPlugin)
        .run();
}
