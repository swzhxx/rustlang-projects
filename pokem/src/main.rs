mod bullet;
mod target;
mod tower;

pub use bullet::*;
pub use target::*;
pub use tower::*;

use bevy::{pbr::NotShadowCaster, prelude::*};
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_mod_picking::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.2, 0.2, 0.2)))
        .insert_resource(WindowDescriptor {
            width: 1280.,
            height: 720.,
            title: "Bevy tower defense".to_string(),
            resizable: false,
            ..default()
        })
        .add_startup_system(spawn_basic_scene)
        .add_startup_system_to_stage(StartupStage::PreStartup, asset_loading)
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(TowerPlugin)
        .add_plugin(TargetPlguin)
        .add_plugin(BulletPlugin)
        // Mod Picking
        .add_plugins(DefaultPickingPlugins)
        .add_system(camera_controls)
        .run();
}
fn spawn_basic_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    game_assets: ResMut<GameAssets>,
) {
    commands
        .spawn_bundle(Camera3dBundle {
            transform: Transform::from_xyz(-2., 2.5, 5.).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        })
        .insert_bundle(PickingCameraBundle::default());

    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });

    let default_collider_color = materials.add(Color::rgba(0.3, 0.5, 0.3, 0.3).into());
    let selected_collider_color = materials.add(Color::rgba(0.3, 0.9, 0.3, 0.9).into());

    commands
        .spawn_bundle(SpatialBundle::from_transform(Transform::from_xyz(
            0., 0.8, 0.,
        )))
        .insert(Name::new("Tower_base"))
        .insert(meshes.add(shape::Capsule::default().into()))
        .insert(NotShadowCaster)
        .insert(Highlighting {
            initial: default_collider_color.clone(),
            hovered: Some(selected_collider_color.clone()),
            pressed: Some(selected_collider_color.clone()),
            selected: Some(selected_collider_color),
        })
        .insert(default_collider_color)
        .insert_bundle(PickableBundle::default())
        .with_children(|commands| {
            commands.spawn_bundle(SceneBundle {
                scene: game_assets.tower_base_scene.clone(),
                transform: Transform::from_xyz(0., -0.8, 0.),
                ..default()
            });
            // .insert(Tower {
            //     shooting_timer: Timer::from_seconds(1., true),
            //     bullet_offset: Vec3::new(0., 0.7, 0.6),
            // })
            // .insert(Name::new("Tower"));
        });

    commands
        .spawn_bundle(PointLightBundle {
            transform: Transform::from_xyz(4., 8., 4.),
            point_light: PointLight {
                intensity: 1500.,
                shadows_enabled: true,
                ..default()
            },
            ..default()
        })
        .insert(Name::new("light"));

    // commands
    //     .spawn_bundle(PbrBundle {
    //         mesh: meshes.add(Mesh::from(shape::Cube { size: 0.4 })),
    //         material: materials.add(Color::rgb(0.67, 0.84, 0.92).into()),
    //         transform: Transform::from_xyz(-2.0, 0.2, 1.5),
    //         ..default()
    //     })
    //     .insert(Target { speed: 0.3 })
    //     .insert(Health { value: 3 })
    //     .insert(Name::new("Target"));

    // commands
    //     .spawn_bundle(PbrBundle {
    //         mesh: meshes.add(Mesh::from(shape::Cube { size: 0.4 })),
    //         material: materials.add(Color::rgb(0.67, 0.84, 0.92).into()),
    //         transform: Transform::from_xyz(-8.0, 0.2, 1.5),
    //         ..default()
    //     })
    //     .insert(Target { speed: 0.3 })
    //     .insert(Health { value: 3 })
    //     .insert(Name::new("Target"));

    commands
        .spawn_bundle(SceneBundle {
            scene: game_assets.target_scene.clone(),
            transform: Transform::from_xyz(-4.0, 0.4, 2.5),
            ..default()
        })
        .insert(Target { speed: 0.3 })
        .insert(Health { value: 3 })
        .insert(Name::new("Target"));

    commands
        .spawn_bundle(SceneBundle {
            scene: game_assets.target_scene.clone(),
            transform: Transform::from_xyz(-2.0, 0.4, 2.5),
            ..default()
        })
        .insert(Target { speed: 0.3 })
        .insert(Health { value: 3 })
        .insert(Name::new("Target"));
}

fn asset_loading(mut commands: Commands, assets: Res<AssetServer>) {
    commands.insert_resource(GameAssets {
        bullet_scene: assets.load("Bullet.glb#Scene0"),
        tower_base_scene: assets.load("TowerBase.glb#Scene0"),
        tomato_tower_scene: assets.load("TomatoTower.glb#Scene0"),
        tomato_scene: assets.load("Tomato.glb#Scene0"),
        target_scene: assets.load("Target.glb#Scene0"),
    })
}

pub struct GameAssets {
    bullet_scene: Handle<Scene>,
    tower_base_scene: Handle<Scene>,
    tomato_tower_scene: Handle<Scene>,
    target_scene: Handle<Scene>,
    tomato_scene: Handle<Scene>,
}

fn camera_controls(
    keyboard: Res<Input<KeyCode>>,
    mut camera_query: Query<&mut Transform, With<Camera3d>>,
    time: Res<Time>,
) {
    let mut camera = camera_query.single_mut();
    let mut forward = camera.forward();
    forward.y = 0.0;
    forward = forward.normalize();

    let mut left = camera.left();
    left.y = 0.;
    left = left.normalize();
    let rotate_speed = 0.3;
    let speed = 3.0;
    if keyboard.pressed(KeyCode::W) {
        camera.translation += forward * time.delta_seconds() * speed;
    }
    if keyboard.pressed(KeyCode::S) {
        camera.translation -= forward * time.delta_seconds() * speed;
    }

    if keyboard.pressed(KeyCode::A) {
        camera.translation += left * time.delta_seconds() * speed;
    }
    if keyboard.pressed(KeyCode::D) {
        camera.translation -= left * time.delta_seconds() * speed;
    }
    if keyboard.pressed(KeyCode::Q) {
        camera.rotate_axis(Vec3::Y, rotate_speed * time.delta_seconds());
    }
    if keyboard.pressed(KeyCode::E) {
        camera.rotate_axis(Vec3::Y, -rotate_speed * time.delta_seconds());
    }
}
