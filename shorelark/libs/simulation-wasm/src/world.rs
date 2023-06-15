use crate::*;
use serde::Serialize;
use wasm_bindgen::prelude::*;
#[derive(Clone, Debug, Serialize)]
pub struct World {
    pub animals: Vec<Animal>,
    pub foods: Vec<Food>,
}

impl From<&sim::World> for World {
    fn from(world: &sim::World) -> Self {
        let animals = world.animals().iter().map(Animal::from).collect();
        let foods = world.foods().iter().map(Food::from).collect();
        Self { animals, foods }
    }
}
