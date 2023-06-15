use lib_simulation as sim;
use rand::prelude::*;
use wasm_bindgen::prelude::*;
mod animal;
mod food;
mod world;
pub use {animal::*, food::*, world::*};
#[wasm_bindgen]
pub fn whos_that_dog() -> String {
    "Mister Peanutbutter".into()
}

#[wasm_bindgen]
pub struct Simulation {
    rng: ThreadRng,
    sim: sim::Simulation,
}

#[wasm_bindgen]
impl Simulation {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        let mut rng = thread_rng();
        let sim = sim::Simulation::random(&mut rng);

        Self { rng, sim }
    }

    pub fn world(&self) -> JsValue {
        let world = World::from(self.sim.world());
        serde_wasm_bindgen::to_value(&world).unwrap()
    }

    pub fn step(&mut self) {
        self.sim.step(&mut self.rng);
    }
    /// min = minimum amount of food eaten by any bird
    ///
    /// max = maximum amount of food eaten by any bird
    ///
    /// avg = sum of all the food eaten by all the birds,
    ///       divided by the number of birds
    ///
    /// Median could also come useful!
    pub fn train(&mut self) -> String {
        let stats = self.sim.train(&mut self.rng);
        format!(
            "min={:.2}, max={:.2}, avg={:.2}",
            stats.min_fitness(),
            stats.max_fitness(),
            stats.avg_fitness()
        )
    }
}

#[cfg(test)]
mod tests {}
