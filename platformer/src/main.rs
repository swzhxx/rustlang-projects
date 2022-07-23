use bevy::{prelude::App, prelude::*, window::WindowDescriptor};
use bevy_rapier2d::{
    plugin::{NoUserData, RapierPhysicsPlugin},
    prelude::{
        Ccd, Collider, ColliderMassProperties, Friction, GravityScale, Restitution, RigidBody,
        Sensor, Sleeping, Velocity,
    },
    rapier::prelude::ColliderBuilder,
};
mod camera;
//Player Component
struct Player;

fn spawn_player(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    // let rigid_body = RigidBodyBundle {
    //     ..Default::default()
    // };

    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.7, 0.7, 0.7),
                custom_size: Some(Vec2::new(1.0, 1.0)),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(RigidBody::Dynamic)
        .insert_bundle(TransformBundle::from(Transform::from_xyz(0.0, 5.0, 0.0)))
        .insert(Velocity {
            linvel: Vec2::new(1.0, 2.0),
            angvel: 0.2,
        })
        .insert(GravityScale(0.5))
        .insert(Sleeping::disabled())
        .insert(Ccd::enabled())
        .insert(Collider::cuboid(1.0, 2.0))
        .insert(Sensor)
        .insert_bundle(TransformBundle::from(Transform::from_xyz(2.0, 0.0, 0.0)))
        .insert(Friction::coefficient(0.7))
        .insert(Restitution::coefficient(0.3))
        .insert(ColliderMassProperties::Density(2.0));
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(camera::new_camera_2d());
}

fn main() {
    // println!("Hello, world!");
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Platformer!".to_string(),
            width: 640.,
            height: 400.,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .add_startup_system(setup)
        .add_startup_stage("player_setup", SystemStage::single(spawn_player))
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .run()
}
