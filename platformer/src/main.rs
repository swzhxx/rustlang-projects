use bevy::{prelude::App, prelude::*, window::WindowDescriptor};
mod camera;
//Player Component
struct Player;

fn spawn_player(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.7, 0.7, 0.7),
            custom_size: Some(Vec2::new(1.0, 1.0)),
            ..Default::default()
        },
        ..Default::default()
    });
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
        .run()
}
