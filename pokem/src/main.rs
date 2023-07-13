mod bullet;
mod main_menu;
mod target;
mod tower;

pub use bullet::*;
pub use target::*;
pub use tower::*;

use bevy::{pbr::NotShadowCaster, prelude::*};
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_mod_picking::*;
pub const HEIGHT: f32 = 720.0;
pub const WIDTH: f32 = 1280.0;
fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.2, 0.2, 0.2)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                width: WIDTH,
                height: HEIGHT,
                title: "Bevy Tower Defense".to_string(),
                resizable: false,
                ..default()
            },
            ..default()
        }))
        .add_state(GameState::MainMenu)
        .add_system_set(SystemSet::on_enter(GameState::GamePlay).with_system(spawn_basic_scene))
        .add_startup_system_to_stage(StartupStage::PreStartup, asset_loading)
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
        .spawn(Camera3dBundle {
            transform: Transform::from_xyz(-2., 2.5, 5.).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        })
        .insert(PickingCameraBundle::default());

    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });

    let default_collider_color = materials.add(Color::rgba(0.3, 0.5, 0.3, 0.3).into());
    let selected_collider_color = materials.add(Color::rgba(0.3, 0.9, 0.3, 0.9).into());

    for i in 0..10 {
        for j in 0..2 {
            commands
                .spawn(SpatialBundle::from_transform(Transform::from_xyz(
                    2.0 * i as f32 + j as f32,
                    0.8,
                    5.0 * j as f32,
                )))
                .insert(Name::new("Tower_Base"))
                .insert(meshes.add(shape::Capsule::default().into()))
                .insert(Highlighting {
                    initial: default_collider_color.clone(),
                    hovered: Some(selected_collider_color.clone()),
                    pressed: Some(selected_collider_color.clone()),
                    selected: Some(selected_collider_color.clone()),
                })
                .insert(default_collider_color.clone())
                .insert(NotShadowCaster)
                .insert(PickableBundle::default())
                .with_children(|commands| {
                    commands.spawn(SceneBundle {
                        scene: game_assets.tower_base_scene.clone(),
                        transform: Transform::from_xyz(0.0, -0.8, 0.0),
                        ..Default::default()
                    });
                });
        }
    }

    for i in 1..25 {
        commands
            .spawn(SceneBundle {
                scene: game_assets.target_scene.clone(),
                transform: Transform::from_xyz(-2.0 * i as f32, 0.4, 2.5),
                ..Default::default()
            })
            .insert(Target { speed: 0.45 })
            .insert(Health { value: 3 })
            .insert(Name::new("Target"));
    }

    commands
        .spawn(PointLightBundle {
            transform: Transform::from_xyz(4., 8., 4.),
            point_light: PointLight {
                intensity: 1500.,
                shadows_enabled: true,
                ..default()
            },
            ..default()
        })
        .insert(Name::new("light"));
}

fn asset_loading(mut commands: Commands, assets: Res<AssetServer>) {
    commands.insert_resource(GameAssets {
        tower_base_scene: assets.load("TowerBase.glb#Scene0"),
        tomato_tower_scene: assets.load("TomatoTower.glb#Scene0"),
        tomato_scene: assets.load("Tomato.glb#Scene0"),
        potato_tower_scene: assets.load("PotatoTower.glb#Scene0"),
        potato_scene: assets.load("Potato.glb#Scene0"),
        cabbage_tower_scene: assets.load("CabbageTower.glb#Scene0"),
        cabbage_scene: assets.load("Cabbage.glb#Scene0"),
        target_scene: assets.load("Target.glb#Scene0"),
    })
}

#[derive(Resource)]
pub struct GameAssets {
    tower_base_scene: Handle<Scene>,
    tomato_tower_scene: Handle<Scene>,
    target_scene: Handle<Scene>,
    tomato_scene: Handle<Scene>,
    potato_tower_scene: Handle<Scene>,
    potato_scene: Handle<Scene>,
    cabbage_tower_scene: Handle<Scene>,
    cabbage_scene: Handle<Scene>,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    MainMenu,
    GamePlay,
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
