use nalgebra::Point2;
use rand::{Rng, RngCore};

#[derive(Debug)]
pub struct Food {
    pub (crate) position: Point2<f32>,
}
impl Food {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        Self {
            position: rng.gen(),
        }
    }
    pub fn position(&self) -> Point2<f32> {
        self.position
    }
}
