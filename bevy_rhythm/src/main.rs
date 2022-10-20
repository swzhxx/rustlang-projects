mod arrows;
mod consts;
mod types;
use bevy::{input::system::exit_on_esc_system, prelude::*};
use consts::*;
fn main() {
    App::build()
        .add_system(exit_on_esc_system.system())
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(WindowDescriptor {
            title: "Rhythm!".to_string(),
            width: 800.,
            height: 600.,
            ..Default::default()
        })
        .add_startup_system(setup.system())
        .add_system(exit_on_esc_system.system())
        .add_plugins(DefaultPlugins)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}
