use bevy::{pbr::wireframe::WireframePlugin, prelude::*, winit::WinitSettings};
mod components;
mod controlls;
mod system;
mod ui;
mod utils;
use bevy_mod_picking::DefaultPickingPlugins;
use components::*;
use controlls::{OrbitController, OrbitControllerPlugin};
use nalgebra::Translation;
fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "oii oii".to_string(),
            ..default()
        }),
        ..default()
    }))
    .add_plugins(DefaultPickingPlugins)
    .add_plugin(bevy_egui::EguiPlugin)
    .add_plugin(OrbitControllerPlugin)
    .add_plugin(WireframePlugin)
    .insert_resource(ClearColor(Color::rgb(0.15, 0.15, 0.15)))
    .insert_resource(Msaa::Sample4)
    .insert_resource(AmbientLight {
        brightness: 0.3,
        ..default()
    })
    .insert_resource(WinitSettings::desktop_app())
    .add_startup_system(set_store)
    .add_startup_system(setup)
    .add_system(ui::ui_system)
    .add_system(system::render_obj);
    app.run();
}

fn set_store(mut commands: Commands) {
    commands.spawn(PickedFiles { ..default() });
}
fn setup(mut commands: Commands) {
    commands
        .spawn(Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(0., 5., 2.)),
            ..default()
        })
        .insert(OrbitController::default());
}
