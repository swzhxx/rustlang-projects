use crate::*;
mod gaussian;
pub use self::gaussian::*;
pub trait MutationMethod {
    fn mutate(&self, rng: &mut dyn RngCore, child: &mut Chromosome);
}
