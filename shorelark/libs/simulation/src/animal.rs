use crate::*;
use nalgebra::{Point2, Rotation2};
use rand::{Rng, RngCore};
#[derive(Debug)]
pub struct Animal {
    pub(crate) position: Point2<f32>,
    pub(crate) rotation: Rotation2<f32>,
    pub(crate) speed: f32,
    pub(crate) eye: Eye,
    pub(crate) brain: Brain,
    pub(crate) satiation: usize,
}
impl Animal {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        let eye = Eye::default();
        let brain = Brain::random(rng, &eye);

        Self::new(eye, brain, rng)
    }
    pub fn position(&self) -> Point2<f32> {
        self.position
    }
    pub fn rotation(&self) -> Rotation2<f32> {
        self.rotation
    }
    fn new(eye: Eye, brain: Brain, rng: &mut dyn RngCore) -> Self {
        Self {
            position: rng.gen(),
            rotation: rng.gen(),
            speed: 0.002,
            eye,
            brain,
            satiation: 0,
        }
    }
    /// "Restores" bird from a chromosome.
    ///
    /// We have to have access to the PRNG in here, because our
    /// chromosomes encode only the brains - and while we restore the
    /// bird, we have to also randomize its position, direction, etc.
    /// (so it's stuff that wouldn't make sense to keep in the genome.)
    pub(crate) fn from_chromosome(chromosome: ga::Chromosome, rng: &mut dyn RngCore) -> Self {
        let eye = Eye::default();
        let brain = Brain::from_chromosome(chromosome, &eye);
        Self::new(eye, brain, rng)
    }

    pub(crate) fn as_chromosome(&self) -> ga::Chromosome {
        self.brain.as_chromosome()
    }
}
