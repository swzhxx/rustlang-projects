use bevy::{
    prelude::Resource,
    render::render_resource::{BindGroupLayout, CachedComputePipelineId},
};

#[derive(Resource, Clone)]
pub struct ParticleRenderPipeline {
    bind_group_layout: BindGroupLayout,
    clear_pipeline: CachedComputePipelineId,
    render_pipeline: CachedComputePipelineId,
}
