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
const MU: f32 = 0.5;

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
        .insert_resource(Restitution(0.5))
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(ObjPlugin)
        .add_startup_system(asset_load)
        .add_startup_system(setup)
        .add_system_set(SystemSet::on_update(AppState::Loading).with_system(is_assert_loaded))
        .add_system_set(
            SystemSet::on_update(AppState::Running)
                .with_system(keyboard_input)
                .with_system(update_velocity)
                .with_system(update_angular.after(update_velocity))
                .with_system(collision_dectection.after(update_angular))
                .with_system(collision_effect.after(collision_dectection))
                .with_system(update_transform.after(collision_effect)),
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

#[derive(Default, PartialEq, Debug)]
struct Restitution(f32);

impl Default for Rigidbody {
    fn default() -> Self {
        let mut i = Mat4::ZERO;
        i.row(3)[3] = 1.;
        Self {
            inertial_ref: i,
            gravity: Vec3::new(0., 0., 0.),
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

struct CollisionEffectMessage {
    entity: Entity,
    velocity: Vec3,
    normal: Vec3,
    Rr: Vec3,
    interial: Mat4,
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
    let mesh = meshes.add(Mesh::from(shape::Plane { size: 1. }));
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
            transform: Transform {
                scale: Vec3::new(2., 2., 2.),
                ..default()
            },
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
                base_color: Color::rgb(0.1, 0.1, 0.8),
                metallic: 0.2,
                perceptual_roughness: 0.5,
                ..default()
            }),
            transform: Transform {
                translation: Vec3::new(2., 0., 0.),
                rotation: Quat::from_euler(EulerRot::XYZ, 0., 0., FRAC_PI_2),
                scale: Vec3::new(2., 2., 2.),
                ..default()
            },
            ..default()
        })
        .insert(Collision {
            normal: Vec3::new(0., 1., 0.),
        });

    // camera
    commands.spawn_bundle(Camera3dBundle {
        transform: Transform::from_xyz(-1.730627, 2.0196069, -1.5942)
            .with_rotation(Quat::from_euler(EulerRot::XYZ, 0.4172, 1.046666, 0.))
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
    let transform = Transform::from_translation(Vec3::new(0., 0.6, 0.))
        .with_rotation(Quat::from_euler(EulerRot::XYZ, 1.39555, 0., 0.));
    let scale = transform.scale;

    let mut interial_ref: Mat4 = Mat4::ZERO;
    interial_ref.col_mut(3)[3] = 1.;
    if let Some(mesh) = meshes.get(&game_assets.bunny) {
        // 计算质量
        let mass = Mass(mesh.count_vertices() as f32);
        let vertices = get_scaled_vertices(mesh, &scale);
        // println!(" position {:?}", positions);
        // let position = mesh.attributes();

        for i in 0..vertices.len() {
            let _v = vertices[i];
            let diag = _v.length_squared();

            // interial_ref.row(0)[0] += diag;
            // interial_ref.row(1)[1] += diag;
            // interial_ref.row(2)[2] += diag;

            // interial_ref.row(0)[0] -= _v.x * _v.x;
            // interial_ref.row(0)[1] -= _v.x * _v.y;
            // interial_ref.row(0)[2] -= _v.x * _v.z;

            // interial_ref.row(1)[0] -= _v.y * _v.x;
            // interial_ref.row(1)[1] -= _v.y * _v.y;
            // interial_ref.row(1)[2] -= _v.y * _v.z;

            // interial_ref.row(2)[0] -= _v.z * _v.x;
            // interial_ref.row(2)[1] -= _v.z * _v.y;
            // interial_ref.row(2)[2] -= _v.z * _v.z;

            interial_ref.col_mut(0)[0] += diag;
            interial_ref.col_mut(1)[1] += diag;
            interial_ref.col_mut(2)[2] += diag;

            interial_ref.col_mut(0)[0] -= _v.x * _v.x;
            interial_ref.col_mut(0)[1] -= _v.y * _v.x;
            interial_ref.col_mut(0)[2] -= _v.z * _v.x;

            interial_ref.col_mut(1)[0] -= _v.x * _v.y;
            interial_ref.col_mut(1)[1] -= _v.y * _v.y;
            interial_ref.col_mut(1)[2] -= _v.z * _v.y;

            interial_ref.col_mut(2)[0] -= _v.x * _v.z;
            interial_ref.col_mut(2)[1] -= _v.y * _v.z;
            interial_ref.col_mut(2)[2] -= _v.z * _v.z;
        }
        interial_ref.row(3)[3] = 1.;
        println!("interial_ref {:?}", interial_ref);
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
            .insert(Velocity::default())
            .insert(AnguleVelocity::default())
            .insert(mass);
    }
}

fn keyboard_input(
    mut commands: Commands,
    keys: Res<Input<KeyCode>>,
    mut r_query: Query<(Entity, &mut Rigidbody, &mut Transform, &mut Velocity)>,
    mut restitution: ResMut<Restitution>,
) {
    match r_query.get_single_mut() {
        Ok((_entity, mut rb, mut transform, mut v)) => {
            if keys.pressed(KeyCode::L) {
                v.0 = Vec3::new(5., 2., 0.);
                rb.gravity = Vec3::new(0., -9.8, 0.);
            }
            if keys.pressed(KeyCode::R) {
                restitution.0 = 0.5;
                transform.translation = Vec3::new(0., 0.6, 0.);
                rb.gravity = Vec3::new(0., 0., 0.)
            }
        }
        _ => {}
    }
}

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
        let interial = R * rigid_body.inertial_ref * R.transpose();

        if let Some(mesh) = meshes.get(handle_mesh) {
            let vertices = get_scaled_vertices(mesh, &S);
            for (_, collsion, c_transform) in collsion_query.iter() {
                let mut effect_message: Option<CollisionEffectMessage> = None;
                let mut num = 0;
                let mut total_x = Vec3::ZERO.clone();
                let collsion_R = Mat4::from_quat(c_transform.rotation);
                let c_normal = (collsion_R * collsion.normal.extend(1.))
                    .truncate()
                    .normalize();

                for i in vertices.iter() {
                    // may be w is not 1. trigger bug
                    let xi = T + (R.mul_vec4(i.extend(1.))).truncate();
                    let vi = velocity.0
                        + angular_velocity
                            .0
                            .cross((R.mul_vec4(i.extend(1.))).truncate());

                    let collsion_position = c_transform.translation;

                    if rigid_body.collide_with_plane(&collsion_position, &c_normal, &xi)
                        && vi.dot(c_normal.clone()) < 0.
                    {
                        num += 1;
                        total_x += xi;
                    };
                }
                if num == 0 {
                    continue;
                }

                let x_mean = total_x.div(num as f32);
                let Rr = x_mean - T;
                let V = velocity.0 + angular_velocity.0.cross(Rr);
                effect_message = Some(CollisionEffectMessage {
                    entity: rigid_entity,
                    velocity: V,
                    normal: c_normal,
                    Rr,
                    interial: interial,
                });

                if effect_message.is_some() {
                    commands.spawn().insert(effect_message.unwrap());
                }
            }
        }
    }
}

fn collision_effect(
    mut commands: Commands,
    mut query: Query<(Entity, &CollisionEffectMessage)>,
    mut rigidy_query: Query<(
        Entity,
        &Mass,
        &mut Velocity,
        &mut AnguleVelocity,
        &Rigidbody,
    )>,
    mut restitution: ResMut<Restitution>,
) {
    for (_message_entity, message) in query.iter_mut() {
        let mut vn = message.velocity.dot(message.normal) * message.normal;
        let mut vt = message.velocity - vn;
        // println!("1  vn {} , vt {}", vn, vt);
        let Rr = message.Rr;
        let a = (0 as f32).max(1. - MU * (1. + restitution.0) * vn.length() / vt.length());
        vn = -1. * restitution.0 * vn;
        vt = a * vt;
        // println!("2  vn {} , vt {}", vn, vt);

        let vnew = vn + vt;

        let mut K = Mat4::IDENTITY;
        let (_entity, mass, mut velocity, mut angular_velocity, rigid_body) =
            rigidy_query.get_mut(message.entity).unwrap();

        // K.row(0)[0] /= mass.0;
        // K.row(1)[1] /= mass.0;
        // K.row(2)[2] /= mass.0;
        // K.row(3)[3] /= mass.0;

        K.col_mut(0)[0] /= mass.0;
        K.col_mut(1)[1] /= mass.0;
        K.col_mut(2)[2] /= mass.0;
        K.col_mut(3)[3] /= mass.0;

        let temp = vec3_to_antisymmetric_matrix4(Rr.clone())
            * message.interial.inverse()
            * vec3_to_antisymmetric_matrix4(Rr.clone());

        // K.row(0)[0] -= temp.row(0)[0];
        // K.row(0)[1] -= temp.row(0)[1];
        // K.row(0)[2] -= temp.row(0)[2];
        // K.row(0)[3] -= temp.row(0)[3];

        // K.row(1)[0] -= temp.row(1)[0];
        // K.row(1)[1] -= temp.row(1)[1];
        // K.row(1)[2] -= temp.row(1)[2];
        // K.row(1)[3] -= temp.row(1)[3];

        // K.row(2)[0] -= temp.row(2)[0];
        // K.row(2)[1] -= temp.row(2)[1];
        // K.row(2)[2] -= temp.row(2)[2];

        // K.row(3)[3] -= temp.row(3)[3];

        K.col_mut(0)[0] -= temp.col(0)[0];
        K.col_mut(1)[0] -= temp.col(1)[0];
        K.col_mut(2)[0] -= temp.col(2)[0];
        K.col_mut(3)[0] -= temp.col(3)[0];

        K.col_mut(0)[1] -= temp.col(0)[1];
        K.col_mut(1)[1] -= temp.col(1)[1];
        K.col_mut(2)[1] -= temp.col(2)[1];
        K.col_mut(3)[1] -= temp.col(3)[1];

        K.col_mut(0)[2] -= temp.col(0)[2];
        K.col_mut(1)[2] -= temp.col(1)[2];
        K.col_mut(2)[2] -= temp.col(2)[2];

        K.col_mut(3)[3] -= temp.col(3)[3];

        let j: Vec4 = K
            .inverse()
            .mul_vec4(vnew.extend(1.) - message.velocity.clone().extend(1.));

        let j = j.truncate();

        velocity.0 = velocity.0 + j / mass.0;
        let _delta_angular_velocity = message
            .interial
            .inverse()
            .mul_vec4((Rr.cross(j)).extend(1.));
        // println!("delta angular velocity : {}", _delta_angular_velocity);
        let delta_angular_velocity = (_delta_angular_velocity).truncate();
        angular_velocity.0 = angular_velocity.0 + delta_angular_velocity;

        restitution.0 = restitution.0 * 0.9;
        if restitution.0 < 0.01 || angular_velocity.0.length() < 0.01 {
            restitution.0 = 0.;
        }
    }
    for (message_entity, _) in query.iter() {
        commands.entity(message_entity).despawn_recursive();
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
        angular.0 *= ANGULE_DECAY * delta
    }
}

fn get_scaled_vertices(mesh: &Mesh, scale: &Vec3) -> Vec<Vec3> {
    let vertices = mesh
        .attribute(Mesh::ATTRIBUTE_POSITION)
        .unwrap()
        .as_float3()
        .unwrap();

    let res = vertices
        .iter()
        .map(move |v| {
            let _v = Vec3::new(v[0], v[1], v[2]);
            _v
        })
        .collect();

    return res;
}

fn vec3_to_antisymmetric_matrix4(vec: Vec3) -> Mat4 {
    let mut temp = Mat4::ZERO;
    // temp.row(0)[0] = 0.;
    // temp.row(0)[1] = -vec.z;
    // temp.row(0)[2] = vec.y;

    // temp.row(1)[0] = vec.z;
    // temp.row(1)[1] = 0.;
    // temp.row(1)[2] = -vec.x;

    // temp.row(2)[0] = -vec.y;
    // temp.row(2)[1] = vec.x;
    // temp.row(2)[2] = 0.;

    // temp.row(3)[3] = 1.;

    temp.col_mut(0)[0] = 0.;
    temp.col_mut(1)[0] = -vec.z;
    temp.col_mut(2)[0] = vec.y;

    temp.col_mut(0)[1] = vec.z;
    temp.col_mut(1)[1] = 0.;
    temp.col_mut(2)[1] = -vec.x;

    temp.col_mut(0)[2] = -vec.y;
    temp.col_mut(1)[2] = vec.x;
    temp.col_mut(2)[2] = 0.;

    temp.col_mut(3)[3] = 1.;

    temp
}

fn update_transform(
    mut query: Query<(&mut Transform, &Velocity, &AnguleVelocity)>,
    timer: Res<Time>,
) {
    let delta = timer.delta_seconds();
    for (mut transform, v, w) in query.iter_mut() {
        transform.translation = transform.translation + v.0 * delta;
        // println!("update transform {:?}", transform.translation);
        let delta_q = w.0 * delta / 2.;
        let q0 = transform.rotation;
        let temp = Quat::from_array([delta_q.x, delta_q.y, delta_q.z, 0.]) * q0;

        transform.rotation = Quat::from_array([
            q0.x + temp.x,
            q0.y + temp.y,
            delta_q.z + q0.z,
            temp.w + q0.w,
        ])
        .normalize();
    }
}
