use crate::{HEIGHT, PARTICLE_COUNT, WIDTH, WORKGROUP_SIZE};
use bevy::render::{
    render_resource::{CachedComputePipelineId, PipelineCache, *},
    renderer::RenderContext,
};
use wgpu::BindGroup;

pub fn run_compute_pass(
    render_context: &mut RenderContext,
    bind_group: &BindGroup,
    pipeline_cache: &PipelineCache,
    pipeline: CachedComputePipelineId,
) {
    let mut pass = render_context
        .command_encoder
        .begin_compute_pass(&ComputePassDescriptor::default());
    pass.set_bind_group(0, bind_group, &[]);
    let pipeline = pipeline_cache.get_compute_pipeline(pipeline).unwrap();
    pass.set_pipeline(pipeline);
    pass.dispatch_workgroups(PARTICLE_COUNT / WORKGROUP_SIZE, 1, 1);
}
