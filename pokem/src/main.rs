use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;
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
        .add_startup_system(setup)
        .add_startup_system(asset_loading)
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .register_type::<Tower>()
        .add_system(tower_shooting)
        .add_system(bullet_despawn)
        .run();
}
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn_bundle(Camera3dBundle {
        transform: Transform::from_xyz(-2., 2.5, 5.).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1. })),
            material: materials.add(Color::rgb(0.67, 0.84, 0.92).into()),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        })
        .insert(Tower {
            shooting_timer: Timer::from_seconds(1., true),
        })
        .insert(Name::new("Tower"));
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
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Tower {
    shooting_timer: Timer,
}

fn tower_shooting(
    mut commands: Commands,
    bullet_assets: Res<GameAssets>,
    mut towers: Query<&mut Tower>,
    time: Res<Time>,
) {
    for mut tower in &mut towers {
        tower.shooting_timer.tick(time.delta());
        if tower.shooting_timer.just_finished() {
            let spawn_transform =
                Transform::from_xyz(0., 0.7, 0.6).with_rotation(Quat::from_rotation_y(-PI / 2.));
            commands
                .spawn_bundle(SceneBundle {
                    scene: bullet_assets.bullet_scene.clone(),
                    transform: spawn_transform,
                    ..default()
                })
                .insert(Lifetime {
                    timer: Timer::from_seconds(0.5, false),
                })
                .insert(Name::new("Bullet"));
        }
    }
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Lifetime {
    timer: Timer,
}

fn bullet_despawn(
    mut commands: Commands,
    mut bullets: Query<(Entity, &mut Lifetime)>,
    time: Res<Time>,
) {
    for (entity, mut lifetime) in &mut bullets {
        lifetime.timer.tick(time.delta());
        if lifetime.timer.just_finished() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn asset_loading(mut commands: Commands, assets: Res<AssetServer>) {
    commands.insert_resource(GameAssets {
        bullet_scene: assets.load("Bullet.glb#Scene0"),
    })
}

pub struct GameAssets {
    bullet_scene: Handle<Scene>,
}
