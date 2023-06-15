use bevy::{pbr::wireframe::WireframePlugin, prelude::*, winit::WinitSettings};
mod components;
mod controlls;
mod system;
mod ui;
mod utils;
use bevy_mod_picking::{DefaultPickingPlugins, PickingCameraBundle, PickingPluginsState};
use components::*;
use controlls::{OrbitController, OrbitControllerPlugin};

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
    .add_system(system::render_obj)
    .add_system(system::pick_events.in_base_set(CoreSet::PostUpdate));
    app.run();
}

fn set_store(
    mut commands: Commands,
    mut resource_picking_plugin_state: ResMut<PickingPluginsState>,
) {
    commands.spawn(PickedFiles { ..default() });
    resource_picking_plugin_state.enable_highlighting = false;
}
fn setup(mut commands: Commands) {
    commands
        .spawn(Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(0., 5., 2.)),
            ..default()
        })
        .insert(OrbitController::default())
        .insert(PickingCameraBundle::default());
}

// use bevy::prelude::*;

// use bevy_mod_picking::{DefaultPickingPlugins, PickableBundle, PickingCameraBundle, PickingEvent};

// fn main() {
//     App::new()
//         .add_plugins(DefaultPlugins)
//         .add_plugins(DefaultPickingPlugins) // <- Adds picking, interaction, and highlighting
//         .add_startup_system(setup)
//         .add_system(print_events.in_base_set(CoreSet::PostUpdate))
//         .run();
// }

// pub fn print_events(mut events: EventReader<PickingEvent>) {
//     for event in events.iter() {
//         match event {
//             PickingEvent::Selection(e) => info!("A selection event happened: {:?}", e),
//             PickingEvent::Hover(e) => info!("Egads! A hover event!? {:?}", e),
//             PickingEvent::Clicked(e) => info!("Gee Willikers, it's a click! {:?}", e),
//         }
//     }
// }

// fn setup(
//     mut commands: Commands,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<StandardMaterial>>,
// ) {
//     commands.spawn((
//         PbrBundle {
//             mesh: meshes.add(Mesh::from(shape::Plane::from_size(5.0))),
//             material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
//             ..Default::default()
//         },
//         PickableBundle::default(), // <- Makes the mesh pickable.
//     ));
//     commands.spawn((
//         PbrBundle {
//             mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
//             material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
//             transform: Transform::from_xyz(0.0, 0.5, 0.0),
//             ..Default::default()
//         },
//         PickableBundle::default(), // <- Makes the mesh pickable.
//     ));
//     commands.spawn(PointLightBundle {
//         point_light: PointLight {
//             intensity: 1500.0,
//             shadows_enabled: true,
//             ..Default::default()
//         },
//         transform: Transform::from_xyz(4.0, 8.0, 4.0),
//         ..Default::default()
//     });
//     commands.spawn((
//         Camera3dBundle {
//             transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
//             ..Default::default()
//         },
//         PickingCameraBundle::default(), // <- Sets the camera to use for picking.
//     ));
// }