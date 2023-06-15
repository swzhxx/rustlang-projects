use std::f32::consts::{FRAC_PI_4, PI};

use crate::Food;
use nalgebra as na;
use nalgebra::{Point2, Rotation2, Vector2};

/// How far our eye can see:
///
/// -----------------
/// |               |
/// |               |
/// |               |
/// |@      %      %|
/// |               |
/// |               |
/// |               |
/// -----------------
///
/// If @ marks our birdie and % marks food, then a FOV_RANGE of:
///
/// - 0.1 = 10% of the map = bird sees no foods (at least in this case)
/// - 0.5 = 50% of the map = bird sees one of the foods
/// - 1.0 = 100% of the map = bird sees both foods
const FOV_RANGE: f32 = 0.25;

/// How wide our eye can see.
///
/// If @> marks our birdie (rotated to the right) and . marks the area
/// our birdie sees, then a FOV_ANGLE of:
///
/// - PI/2 = 90° =
///   -----------------
///   |             /.|
///   |           /...|
///   |         /.....|
///   |       @>......|
///   |         \.....|
///   |           \...|
///   |             \.|
///   -----------------
///
/// - PI = 180° =
///   -----------------
///   |       |.......|
///   |       |.......|
///   |       |.......|
///   |       @>......|
///   |       |.......|
///   |       |.......|
///   |       |.......|
///   -----------------
///
/// - 2 * PI = 360° =
///   -----------------
///   |...............|
///   |...............|
///   |...............|
///   |.......@>......|
///   |...............|
///   |...............|
///   |...............|
///   -----------------
///
/// Field of view depends on both FOV_RANGE and FOV_ANGLE:
///
/// - FOV_RANGE=0.4, FOV_ANGLE=PI/2:
///   -----------------
///   |       @       |
///   |     /.v.\     |
///   |   /.......\   |
///   |   ---------   |
///   |               |
///   |               |
///   |               |
///   -----------------
///
/// - FOV_RANGE=0.5, FOV_ANGLE=2*PI:
///   -----------------
///   |               |
///   |      ---      |
///   |     /...\     |
///   |    |..@..|    |
///   |     \.../     |
///   |      ---      |
///   |               |
///   -----------------
const FOV_ANGLE: f32 = PI + FRAC_PI_4;

/// How much photoreceptors there are in a single eye.
///
/// More cells means our birds will have more "crisp" vision, allowing
/// them to locate the food more precisely - but the trade-off is that
/// the evolution process will then take longer, or even fail, unable
/// to find any solution.
///
/// I've found values between 3~11 sufficient, with eyes having more
/// than ~20 photoreceptors yielding progressively worse results.
const CELLS: usize = 9;

#[derive(Debug)]
pub struct Eye {
    fov_range: f32,
    fov_angle: f32,
    cells: usize,
}

impl Eye {
    fn new(fov_range: f32, fov_angle: f32, cells: usize) -> Self {
        assert!(fov_range > 0.0);
        assert!(fov_angle > 0.0);
        assert!(cells > 0);

        Self {
            fov_angle,
            fov_range,
            cells,
        }
    }

    pub fn cells(&self) -> usize {
        self.cells
    }

    pub fn process_vision(
        &self,
        position: Point2<f32>,
        rotation: Rotation2<f32>,
        foods: &[Food],
    ) -> Vec<f32> {
        let mut cells = vec![0.; self.cells];
        for food in foods {
            let vec = food.position - position;
            let dist = vec.norm();
            if dist > self.fov_range {
                continue;
            }
            let angle = Rotation2::rotation_between(&Vector2::y(), &vec).angle();
            let angle = angle - rotation.angle();
            let angle = na::wrap(angle, -PI, PI);
            if angle < -self.fov_angle / 2.0 || angle > self.fov_angle / 2.0 {
                continue;
            }
            let angle = angle + self.fov_angle / 2.0;
            let cell = angle / self.fov_angle * (self.cells as f32);
            let cell = (cell as usize).min(cells.len() - 1);
            let energy = (self.fov_range - dist) / self.fov_range;
            cells[cell] += energy;
        }
        cells
    }
}

impl Default for Eye {
    fn default() -> Self {
        Self::new(FOV_RANGE, FOV_ANGLE, CELLS)
    }
}
