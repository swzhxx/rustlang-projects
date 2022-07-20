use bevy::{
    prelude::*,
    render::camera::{Camera2d, DepthCalculation, ScalingMode},
};
pub fn new_camera_2d() -> OrthographicCameraBundle<Camera2d> {
    let far = 1000.;
    let mut camera = OrthographicCameraBundle::new_2d();
    camera.orthographic_projection = OrthographicProjection {
        far,
        depth_calculation: DepthCalculation::ZDifference,
        scaling_mode: ScalingMode::FixedHorizontal,

        ..Default::default()
    };
    camera.transform.scale = Vec3::new(10., 10., 1.);
    camera
}
