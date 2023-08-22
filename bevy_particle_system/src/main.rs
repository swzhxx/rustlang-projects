use bevy::{prelude::*, render::texture::ImageSampler};
use bevy_inspector_egui::WorldInspectorPlugin;

use wgpu::{Extent3d, TextureDimension, TextureFormat, TextureUsages};

mod particle_system;
mod particle_render;
mod particle_update;
use particle_system::ParticlePlguin;

#[derive(Component, Default, Clone)]
pub struct ParticleSystem {
    pub rendered_texture: Handle<Image>,
}

pub const HEIGHT: f32 = 480.0;
pub const WIDTH: f32 = 640.0;
fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        window: WindowDescriptor {
            width: WIDTH,
            height: HEIGHT,
            title: "Logic Particles".to_string(),
            resizable: false,
            ..default()
        },
        ..default()
    }))
    .add_plugin(WorldInspectorPlugin::new())
    .add_plugin(ParticlePlguin)
    .add_startup_system(setup)
    .add_system(spawn_on_space_bar);
    app.run()
}

fn create_texture(images: &mut Assets<Image>) -> Handle<Image> {
    let mut image = Image::new_fill(
        Extent3d {
            width: WIDTH as u32,
            height: HEIGHT as u32,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &[0, 0, 0, 0],
        TextureFormat::Rgba8Unorm,
    );
    image.texture_descriptor.usage =
        TextureUsages::COPY_DST | TextureUsages::STORAGE_BINDING | TextureUsages::TEXTURE_BINDING;
    image.sampler_descriptor = ImageSampler::nearest();
    images.add(image)
}

fn setup() {}
fn spawn_on_space_bar() {}
