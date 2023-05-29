use bevy::{
    pbr::wireframe::{Wireframe, WireframePlugin},
    prelude::{shape::Quad, *},
    winit::WinitSettings,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use controlls::{OrbitController, OrbitControllerPlugin};
mod components;
mod controlls;
mod field;
mod utils;
mod system;
fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::rgb(0., 0., 0.)))
        .insert_resource(Msaa::Sample4)
        .insert_resource(AmbientLight {
            brightness: 0.3,
            ..default()
        })
        .insert_resource(WinitSettings::desktop_app())
        .add_plugin(WireframePlugin)
        .add_startup_system(setup)
        .add_plugin(OrbitControllerPlugin)
        .add_plugin(WorldInspectorPlugin::new());
    app.run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_translation(Vec3::new(0., 13., 0.))
            .looking_at(Vec3::new(0., 0., 0.), Vec3::Z),
        ..default()
    });
    // .insert(OrbitController::default());

    let plane = meshes.add(Mesh::from(shape::Plane {
        size: 10.,
        subdivisions: 100,
    }));

    // let plane = meshes.add(Mesh::from(Quad {
    //     size: Vec2::new(5., 5.),
    //     flip: false,
    // }));

    commands
        .spawn(PbrBundle {
            mesh: plane,

            material: materials.add(StandardMaterial {
                base_color: Color::Rgba {
                    red: 0.8,
                    green: 0.8,
                    blue: 0.8,
                    alpha: 1.,
                },
                double_sided: true,
                ..default()
            }),
            ..default()
        })
        .insert(Wireframe);
}
