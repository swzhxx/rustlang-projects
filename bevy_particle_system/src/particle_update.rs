use bevy::{
    prelude::{AssetServer, Entity, FromWorld, QueryState, Resource, With, World},
    render::{
        render_graph,
        render_resource::{
            BindGroup, BindGroupLayout, CachedComputePipelineId, Pipeline, PipelineCache,
        },
        renderer::RenderDevice,
    },
    utils::HashMap,
};
use bevy_inspector_egui::bevy_egui::systems;

use crate::{particle_system::ParticleSystemRender, ParticleSystem};

#[derive(Resource, Clone)]
pub struct ParticleUpdatePipeline {
    bind_group_layout: BindGroupLayout,
    init_pipeline: CachedComputePipelineId,
    update_pipeline: CachedComputePipelineId,
}

pub struct UpdateParticlesNode {
    particle_systems: QueryState<Entity, With<ParticleSystem>>,
    update_state: HashMap<Entity, ParticleUpdateState>,
}

#[derive(Default, Clone)]
enum ParticleUpdateState {
    #[default]
    Loading,
    Init,
    Update,
}

pub fn update_bind_group() -> BindGroupLayoutDescriptor {
    todo!()
}

impl FromWorld for ParticleUpdatePipeline {
    fn from_world(world: &mut World) -> Self {
        let bind_group_layout = world
            .resource::<RenderDevice>()
            .create_bind_group_layout(&update_bind_group());
        let shader = world.resource::<AssetServer>().load("particle_update.wgsl");
        let mut pipeline_cache = world.resource_mut::<PipelineCache>();
        todo!()
    }
}

impl render_graph::Node for UpdateParticlesNode {
    fn update(&mut self, world: &mut World) {
        let mut systems = world.query_filtered::<Entity, With<ParticleSystem>>();
        let pipeline = world.resource::<ParticleUpdatePipeline>();
        let pipeline_cache = world.resource::<PipelineCache>();
        for entity in systems.iter(world) {
            self.update_state(entity, pipeline_cache, pipeline);
        }
        // update the query for the run step
        self.particle_systems.update_archetypes(world);
    }
    fn run(
        &self,
        graph: &mut render_graph::RenderGraphContext,
        render_context: &mut bevy::render::renderer::RenderContext,
        world: &bevy::prelude::World,
    ) -> Result<(), render_graph::NodeRunError> {
        let pipline_cache = world.resource::<PipelineCache>();
        let pipeline = world.resource::<ParticleUpdatePipeline>();
        let particle_systems_render = world.resource::<ParticleSystemRender>();
        for entity in self.particle_systems.iter_manual(world) {
            if let Some(pipeline) = match self.update_state[&entity] {
                ParticleUpdateState::Loading => None,
                ParticleUpdateState::Init => Some(pipeline.init_pipeline),
                ParticleUpdateState::Update => Some(pipeline.update_pipeline),
            } {
                run_compute_pass(
                    render_context,
                    &particle_systems_render.update_bind_group(&entity),
                    pipeline_cache,
                    pipeline,
                )
            }
        }
        Ok(())
    }
}

impl UpdateParticlesNode {
    fn update_state(
        &mut self,
        entity: Entity,
        pipeline_cache: &PipelineCache,
        pipeline: &ParticleUpdatePipeline,
    ) {
        let update_state = match self.update_state.get(&entity) {
            Some(state) => state,
            None => {
                self.update_state
                    .insert(entity, ParticleUpdateState::Loading);
                &ParticleUpdateState::Loading
            }
        };
        match update_state {
            ParticleUpdateState::Loading => {
                self.update_state.insert(entity, ParticleUpdateState::Init);
            }
            ParticleUpdateState::Init => {
                self.update_state
                    .insert(entity, ParticleUpdateState::Update);
            }
            ParticleUpdateState::Update => {}
        }
    }
}
