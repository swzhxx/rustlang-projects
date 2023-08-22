use bevy::{
    prelude::{Entity, Plugin, Resource},
    render::{
        extract_component::{ExtractComponent, ExtractComponentPlugin},
        render_resource::{BindGroup, Buffer},
        RenderApp, RenderStage,
    },
    utils::HashMap,
};

use crate::{particle_update::ParticleUpdatePipeline, ParticleSystem};

pub struct ParticlePlguin;
impl Plugin for ParticlePlguin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugin(ExtractComponentPlugin::<ParticleSystem>::default());
        let render_app = app.sub_app_mut(RenderApp);
        render_app
            .init_resource::<ParticleUpdatePipeline>()
            .init_resource::<ParticleSystemRender>()
            .add_system_to_stage(RenderStage::Queue, queue_bind_group);
        let update_node = todo!();
    }
}

fn queue_bind_group() {}

// Must maintain all our own data because render world flushes between frames :,(
#[derive(Resource, Default)]
pub struct ParticleSystemRender {
    pub update_bind_group: HashMap<Entity, BindGroup>,
    pub render_bind_group: HashMap<Entity, BindGroup>,
    pub particle_buffers: HashMap<Entity, Buffer>,
}

impl ExtractComponent for ParticleSystem {
    type Query = &'static ParticleSystem;

    type Filter = ();

    fn extract_component(item: bevy::ecs::query::QueryItem<'_, Self::Query>) -> Self {
        item.clone()
    }
}
