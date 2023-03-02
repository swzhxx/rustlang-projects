use std::{
    f32::consts::FRAC_PI_2,
    ops::{Div, Mul},
};

use bevy::{math::f32::Mat4, prelude::*};
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_obj::*;

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
enum AppState {
    Loading,
    Running,
}

const VELOCITY_DECAY: f32 = 0.999;
const ANGULE_DECAY: f32 = 0.98;

fn main() {
    App::new()
        .add_state(AppState::Loading)
        .insert_resource(ClearColor(Color::rgb(0.2, 0.2, 0.2)))
        .insert_resource(WindowDescriptor {
            width: 1280.,
            height: 720.,
            title: "games 103".to_string(),
            resizable: false,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(ObjPlugin)
        .add_startup_system(asset_load)
        .add_startup_system(setup)
        .add_system_set(SystemSet::on_update(AppState::Loading).with_system(is_assert_loaded))
        .add_system_set(
            SystemSet::on_update(AppState::Running)
                .with_system(keyboard_input)
                // .with_system(update_velocity)
                // .with_system(update_angular)
                // .with_system(collision_dectection)
                .with_system(collision_dectection),
        )
        .run();
}

// 质量
#[derive(Component, Default, PartialEq, Debug, Reflect)]
#[reflect(Component)]
struct Mass(f32);
// 速度
#[derive(Component, Default, PartialEq, Debug, Reflect)]
#[reflect(Component)]
struct Velocity(Vec3);
// 角速度
#[derive(Component, Default, PartialEq, Debug, Reflect)]
#[reflect(Component)]
struct AnguleVelocity(Vec3);

#[derive(Component, Reflect)]
#[reflect(Component)]
struct Rigidbody {
    inertial_ref: Mat4,
    gravity: Vec3,
}

impl Default for Rigidbody {
    fn default() -> Self {
        let mut i = Mat4::ZERO.clone();
        i.row(3)[3] = 1.;
        Self {
            inertial_ref: i,
            gravity: Vec3::new(0., -9.8, 0.),
        }
    }
}

impl Rigidbody {
    fn collide_with_plane(&self, P: &Vec3, N: &Vec3, x: &Vec3) -> bool {
        return (x.clone() - P.clone()).dot(N.clone()) < 0.;
    }
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
struct Collision {
    normal: Vec3,
}

#[derive(Component)]
struct CollisionEffectPack {
    entity: Entity,
    velocity: Vec3,
    normal: Vec3,
    Rr: Vec3,
}

struct GameAssets {
    bunny: Handle<Mesh>,
}

fn asset_load(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(GameAssets {
        bunny: asset_server.load("bunny.obj"),
    })
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh = meshes.add(Mesh::from(shape::Plane { size: 50. }));
    // ground
    commands
        .spawn_bundle(PbrBundle {
            mesh: mesh.clone(),
            material: materials.add(StandardMaterial {
                base_color: Color::rgb(0.3, 0.5, 0.3),
                metallic: 0.2,
                perceptual_roughness: 0.5,
                ..default()
            }),
            ..default()
        })
        .insert(Collision {
            normal: Vec3::new(0., 1., 0.),
        });

    // wall
    commands
        .spawn_bundle(PbrBundle {
            mesh: mesh,
            material: materials.add(StandardMaterial {
                base_color: Color::rgb(0.5, 0.5, 0.5),
                metallic: 0.2,
                perceptual_roughness: 0.5,
                ..default()
            }),
            transform: Transform {
                translation: Vec3::new(0., 0., -2.),
                rotation: Quat::from_euler(EulerRot::XYZ, FRAC_PI_2, 0., 0.),
                ..default()
            },
            ..default()
        })
        .insert(Collision {
            normal: Vec3::new(0., 1., 0.),
        });

    // camera
    commands.spawn_bundle(Camera3dBundle {
        transform: Transform::from_xyz(10.0, 10.0, 15.0)
            .looking_at(Vec3::new(0.0, 0.0, -1.0), Vec3::Y),
        ..default()
    });

    // Light
    commands.spawn_bundle(DirectionalLightBundle {
        transform: Transform::from_rotation(Quat::from_euler(
            EulerRot::ZYX,
            0.0,
            1.0,
            -std::f32::consts::FRAC_PI_4,
        )),
        directional_light: DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        ..default()
    });
}

fn is_assert_loaded(
    mut commands: Commands,
    mut app_state: ResMut<State<AppState>>,
    asset_server: Res<AssetServer>,
    game_assets: Res<GameAssets>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    meshes: Res<Assets<Mesh>>,
) {
    match asset_server.get_load_state(game_assets.bunny.clone_untyped().id) {
        bevy::asset::LoadState::Loaded => {
            init_rigid_bunny(commands, asset_server, game_assets, materials, meshes);
            app_state.set(AppState::Running);
        }
        _ => {}
    }
}

fn init_rigid_bunny(
    mut commands: Commands,
    assets_server: Res<AssetServer>,
    game_assets: Res<GameAssets>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    meshes: Res<Assets<Mesh>>,
) {
    let transform =
        Transform::from_translation(Vec3::new(2., 2., 2.)).with_scale(Vec3::new(20., 20., 20.));
    let scale = transform.scale;

    let mut interial_ref: Mat4 = Mat4::ZERO.clone();
    interial_ref.row(3)[3] = 1.;
    if let Some(mesh) = meshes.get(&game_assets.bunny) {
        // 计算质量
        let mass = Mass(mesh.count_vertices() as f32);
        let vertices = get_scaled_vertices(mesh, &scale);
        // println!(" position {:?}", positions);
        // let position = mesh.attributes();

        for i in 0..vertices.len() {
            let _v = vertices[i];
            let diag = _v.length_squared();
            interial_ref.row(0)[0] += diag;
            interial_ref.row(1)[1] += diag;
            interial_ref.row(2)[2] += diag;
            interial_ref.row(0)[1] -= _v.x * _v.x;
            interial_ref.row(0)[2] -= _v.x * _v.y;
            interial_ref.row(0)[3] -= _v.x * _v.z;

            interial_ref.row(1)[1] -= _v.y * _v.x;
            interial_ref.row(1)[2] -= _v.y * _v.y;
            interial_ref.row(1)[3] -= _v.y * _v.z;

            interial_ref.row(2)[1] -= _v.z * _v.x;
            interial_ref.row(2)[2] -= _v.z * _v.y;
            interial_ref.row(2)[3] -= _v.z * _v.z;
        }

        commands
            .spawn_bundle(PbrBundle {
                mesh: game_assets.bunny.clone(),
                material: materials.add(StandardMaterial {
                    base_color: Color::rgb(1., 1., 1.),
                    ..default()
                }),
                transform: transform,
                ..default()
            })
            .insert(Rigidbody {
                inertial_ref: interial_ref,
                ..default()
            })
            .insert(mass);
    }
}

fn keyboard_input(mut commands: Commands, keyboard: Res<Input<KeyCode>>) {}

fn collision_dectection(
    mut commands: Commands,
    rigid_query: Query<(
        Entity,
        &Rigidbody,
        &Transform,
        &Handle<Mesh>,
        &Velocity,
        &AnguleVelocity,
    )>,
    collsion_query: Query<(Entity, &Collision, &Transform)>,
    meshes: Res<Assets<Mesh>>,
) {
    for (rigid_entity, rigid_body, transform, handle_mesh, velocity, angular_velocity) in
        rigid_query.iter()
    {
        let T = transform.translation;
        let R = Mat4::from_quat(transform.rotation);
        let S = transform.scale;
        let i = R * rigid_body.inertial_ref * R.transpose();

        if let Some(mesh) = meshes.get(handle_mesh) {
            let vertices = get_scaled_vertices(mesh, &S);
            for i in vertices.iter() {
                // may be w is not 1. trigger bug
                let xi = T + (R.mul_vec4(i.extend(1.))).truncate();
                let vi = velocity.0
                    + angular_velocity
                        .0
                        .cross((R.mul_vec4(i.extend(1.))).truncate());

                for (_, collsion, c_transform) in collsion_query.iter() {
                    let collsion_R = Mat4::from_quat(c_transform.rotation);
                    let collsion_position = c_transform.translation;
                    let c_normal = (collsion_R * collsion.normal.extend(1.))
                        .truncate()
                        .normalize();

                    let mut num = 0;
                    let mut total_x = Vec3::ZERO.clone();
                    if rigid_body.collide_with_plane(&collsion_position, &c_normal, &i)
                        && vi.dot(c_normal.clone()) < 0.
                    {
                        num += 1;
                        total_x += xi;
                    };

                    if num == 0 {
                        continue;
                    }

                    let x_mean = total_x.div(num as f32);
                    let Rr = x_mean - T;
                    let V = vi + angular_velocity.0.cross(Rr);
                }
                // if rigid_body.collide_with_plane(, N, x)
            }
        }
    }
}

fn update_velocity(mut velocities: Query<(&mut Velocity, &Rigidbody)>, timer: Res<Time>) {
    let delta = timer.delta_seconds();
    for (mut velocity, rigid_body) in velocities.iter_mut() {
        let new_v = velocity.0 + rigid_body.gravity * delta;
        velocity.0 = new_v * VELOCITY_DECAY;
    }
}

fn update_angular(mut angulars: Query<(&mut AnguleVelocity)>, timer: Res<Time>) {
    let delta = timer.delta_seconds();
    for (mut angular) in angulars.iter_mut() {
        angular.0 *= ANGULE_DECAY
    }
}

fn get_scaled_vertices(mesh: &Mesh, scale: &Vec3) -> Vec<Vec3> {
    let vertices = mesh
        .attribute(Mesh::ATTRIBUTE_POSITION)
        .unwrap()
        .as_float3()
        .unwrap();

    vertices
        .iter()
        .map(move |v| {
            let _v = Vec3::new(v[0], v[1], v[2]).mul(scale.clone());
            _v
        })
        .collect()
}
