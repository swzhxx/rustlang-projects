use bevy::{
    input::mouse::{MouseMotion, MouseScrollUnit::*, MouseWheel},
    prelude::*,
};

use std::ops::RangeInclusive;

#[derive(Component)]
pub struct OrbitController {
    pub x: f32,
    pub y: f32,
    pub center: Vec3,
    pub pitch_range: RangeInclusive<f32>,
    pub distance: f32,
    pub rotate_sensitivity: f32,
    pub pan_sensitivity: f32,
    pub zoom_sensitivity: f32,
    pub rotate_button: MouseButton,
    pub pan_button: MouseButton,
    pub enabled: bool,
}

impl Default for OrbitController {
    fn default() -> Self {
        OrbitController {
            x: 0.0,
            y: std::f32::consts::FRAC_PI_2,
            pitch_range: 0.01..=3.13,
            distance: 5.,
            center: Vec3::ZERO,
            rotate_sensitivity: 1.,
            pan_sensitivity: 1.,
            zoom_sensitivity: 0.8,
            rotate_button: MouseButton::Left,
            pan_button: MouseButton::Right,
            enabled: true,
        }
    }
}

pub struct OrbitControllerPlugin;
impl OrbitControllerPlugin {
    fn update_transform_system(
        mut query: Query<
            (&OrbitController, &mut Transform),
            (Changed<OrbitController>, With<Camera>),
        >,
    ) {
        for (camera, mut transform) in query.iter_mut() {
            let rot = Quat::from_axis_angle(Vec3::Y, camera.x)
                * Quat::from_axis_angle(-Vec3::X, camera.y);
            transform.translation = (rot * Vec3::Y) * camera.distance + camera.center;
            transform.look_at(camera.center, Vec3::Y);
        }
    }

    fn mouse_motion_system(
        time: Res<Time>,
        mut mouse_motion_events: EventReader<MouseMotion>,
        mouse_button_input: Res<Input<MouseButton>>,
        mut query: Query<(&mut OrbitController, &mut Transform, &mut Camera)>,
    ) {
        let mut delta = Vec2::ZERO;
        for event in mouse_motion_events.iter() {
            delta += event.delta;
        }
        for (mut camera, transform, _) in query.iter_mut() {
            if !camera.enabled {
                continue;
            }
            if mouse_button_input.pressed(camera.rotate_button) {
                camera.x -= delta.x * camera.rotate_sensitivity * time.delta_seconds();
                camera.y -= delta.y * camera.rotate_sensitivity * time.delta_seconds();
                camera.y = camera
                    .y
                    .max(*camera.pitch_range.start())
                    .min(*camera.pitch_range.end());
            }
            if mouse_button_input.pressed(camera.pan_button) {
                let right_dir = transform.rotation * Vec3::X;
                let up_dir = transform.rotation * Vec3::Y;
                let pan_vector = (-delta.x * right_dir + delta.y * up_dir)
                    * camera.pan_sensitivity
                    * time.delta_seconds();
                camera.center += pan_vector;
            }
        }
    }

    fn zoom_system(
        mut mouse_wheel_events: EventReader<MouseWheel>,
        mut query: Query<&mut OrbitController, With<Camera>>,
    ) {
        let mut total = 0.;
        for event in mouse_wheel_events.iter() {
            total += event.y
                * match event.unit {
                    Line => 1.,
                    Pixel => 0.1,
                };
        }
        for mut camera in query.iter_mut() {
            if camera.enabled {
                camera.distance *= camera.zoom_sensitivity.powf(total);
            }
        }
    }
}

impl Plugin for OrbitControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(Self::mouse_motion_system)
            .add_system(Self::zoom_system)
            .add_system(Self::update_transform_system);
    }
}
