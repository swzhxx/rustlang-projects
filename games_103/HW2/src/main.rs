use bevy::{
    pbr::wireframe::{Wireframe, WireframeConfig, WireframePlugin},
    prelude::*,
    render::{
        camera::Projection,
        mesh::{Indices, VERTEX_ATTRIBUTE_BUFFER_ID},
        render_resource::PrimitiveTopology,
    },
};
use bevy_inspector_egui::WorldInspectorPlugin;
mod components;
mod systems;
use components::*;
use systems::implict_model;
fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.2, 0.2, 0.2)))
        .insert_resource(WindowDescriptor {
            width: 1280.,
            height: 720.,
            title: "games 103 HW2".to_string(),
            resizable: false,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .add_startup_system(setup_screne)
        .add_system(implict_model)
        .add_plugin(WireframePlugin)
        .run()
}

fn setup_screne(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut wireframe_config: ResMut<WireframeConfig>,
) {
    wireframe_config.global = false;
    commands.spawn_bundle(Camera3dBundle {
        transform: Transform {
            translation: Vec3::new(-52.9, 22.3, -44.8),
            rotation: Quat::from_euler(EulerRot::XYZ, 21.865, 126.956, 0.),
            scale: Vec3::new(1., 1., 1.),
        }
        .looking_at(Vec3::new(0., 0., 0.), Vec3::Y),
        projection: Projection::Perspective(PerspectiveProjection {
            fov: 12.,
            ..default()
        }),
        ..default()
    });

    commands
        .spawn_bundle(PbrBundle {
            transform: Transform {
                translation: Vec3::new(4.5, -5.41331, -13.87507),
                scale: Vec3::new(5., 5., 5.),
                rotation: Quat::from_euler(EulerRot::XYZ, 0., 0., 0.),
            },
            material: materials.add(StandardMaterial {
                base_color: Color::rgb(0.8, 0.0, 0.0),
                metallic: 0.2,
                perceptual_roughness: 0.5,
                ..default()
            }),
            mesh: meshes.add(Mesh::from(shape::Icosphere::default())),
            ..default()
        })
        .insert(Force);

    commands.spawn_bundle(DirectionalLightBundle {
        transform: Transform {
            translation: Vec3::new(-5.89, -0.854, -8.4048),
            rotation: Quat::from_euler(EulerRot::XYZ, -31.744, -38.489, 175.51),
            scale: Vec3::new(1., 1., 1.),
        },
        ..default()
    });

    commands.spawn_bundle(DirectionalLightBundle {
        transform: Transform::from_xyz(10.94, -17.45, -8.268).with_rotation(Quat::from_euler(
            EulerRot::XYZ,
            13.76,
            86.5,
            43.183,
        )),
        ..default()
    });

    let cloth = meshes.add(Mesh::from(Mesh::new(PrimitiveTopology::TriangleList)));
    // init cloth vertices
    let n = 21u32;
    let mut triangles: Vec<u32> = vec![0; ((n - 1) * (n - 1) * 6) as usize];
    let mut X: Vec<Vec3> = vec![Vec3::default(); (n * n) as usize];
    if let Some(mut mesh) = meshes.get_mut(&cloth) {
        let mut UV: Vec<Vec3> = vec![Vec3::default(); (n * n) as usize];

        for j in 0..n as usize {
            for i in 0..n as usize {
                X[j * n as usize + i] = Vec3::new(
                    5. - 10. * i as f32 / (n - 1) as f32,
                    0.,
                    -(5. - 10. * j as f32 / (n - 1) as f32),
                );

                UV[j * n as usize + i] =
                    Vec3::new(i as f32 / (n as f32 - 1.), j as f32 / (n as f32 - 1.), 0.);
            }
        }
        let mut t = 0;
        for j in 0..n - 1 {
            for i in 0..n - 1 {
                triangles[t * 6 + 0] = j * n + i;
                triangles[t * 6 + 1] = j * n + i + 1;
                triangles[t * 6 + 2] = (j + 1) * n + i + 1;
                triangles[t * 6 + 3] = j * n + i;
                triangles[t * 6 + 4] = (j + 1) * n + i + 1;
                triangles[t * 6 + 5] = (j + 1) * n + i;

                t += 1;
            }
        }

        let _X = X
            .iter()
            .map(|item| [item.x, item.y, item.z])
            .collect::<Vec<[f32; 3]>>();
        let _UV = UV
            .iter()
            .map(|item| [item.x, item.y])
            .collect::<Vec<[f32; 2]>>();
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, _UV);
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, _X);

        // mesh.duplicate_vertices();
        mesh.compute_flat_normals();
        mesh.set_indices(Some(Indices::U32(triangles.clone())));
    }
    commands
        .spawn_bundle(PbrBundle {
            material: materials.add(StandardMaterial {
                base_color: Color::rgb(0., 0., 0.3),
                emissive: Color::rgb(0., 0., 0.8),
                ..default()
            }),
            mesh: cloth,
            ..default()
        })
        .insert(Wireframe)
        .insert_bundle(ClothBundle {
            elv: ELV::init(&triangles, &X),
            ..default()
        });
}
