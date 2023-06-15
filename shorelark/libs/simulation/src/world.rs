use crate::{Animal, Food};
use rand::RngCore;

#[derive(Debug)]
pub struct World {
    pub(crate) animals: Vec<Animal>,
    pub(crate) foods: Vec<Food>,
}

impl World {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        let animals: Vec<Animal> = (0..40).map(|_| Animal::random(rng)).collect();
        let foods: Vec<Food> = (0..60).map(|_| Food::random(rng)).collect();
        Self { animals, foods }
    }
}

impl World {
    pub fn animals(&self) -> &[Animal] {
        &self.animals
    }

    pub fn foods(&self) -> &[Food] {
        &self.foods
    }
}
