mod animal;
mod animal_individual;
mod brain;
mod eye;
mod food;
mod world;
use lib_genetic_algorithm as ga;
use lib_neural_network as nn;
use na::{Point2, Rotation2};
use nalgebra as na;
use rand::{Rng, RngCore};

pub use {animal::*, animal_individual::*, brain::*, eye::*, food::*, world::*};

// FRAC_PI_2 = PI / 2.0; a convenient shortcut
use std::f32::consts::FRAC_PI_2;

/// Minimum speed of a bird.
///
/// Keeping it above zero prevents birds from getting stuck in one place.
const SPEED_MIN: f32 = 0.001;

/// Maximum speed of a bird.
///
/// Keeping it "sane" prevents birds from accelerating up to infinity,
/// which makes the simulation... unrealistic :-)
const SPEED_MAX: f32 = 0.005;

/// Speed acceleration; determines how much the brain can affect bird's
/// speed during one step.
///
/// Assuming our bird is currently flying with speed=0.5, when the brain
/// yells "stop flying!", a SPEED_ACCEL of:
///
/// - 0.1 = makes it take 5 steps ("5 seconds") for the bird to actually
///         slow down to SPEED_MIN,
///
/// - 0.5 = makes it take 1 step for the bird to slow down to SPEED_MIN.
///
/// This improves simulation faithfulness, because - as in real life -
/// it's not possible to increase speed from 1km/h to 50km/h in one
/// instant, even if your brain very much wants to.
const SPEED_ACCEL: f32 = 0.2;

/// Ditto, but for rotation:
///
/// - 2 * PI = it takes one step for the bird to do a 360° rotation,
/// - PI = it takes two steps for the bird to do a 360° rotation,
///
/// I've chosen PI/2, because - as our motto goes - this value seems
/// to play nice.
const ROTATION_ACCEL: f32 = FRAC_PI_2;

/// How much `.step()`-s have to occur before we push data into the
/// genetic algorithm.
///
/// Value that's too low might prevent the birds from learning, while
/// a value that's too high will make the evolution unnecessarily
/// slower.
///
/// You can treat this number as "for how many steps each bird gets
/// to live"; 2500 was chosen with a fair dice roll.
const GENERATION_LENGTH: usize = 2500;

pub struct Simulation {
    world: World,
    ga: ga::GeneticAlgorithm<ga::RouletteWheelSelection>,
    age: usize,
}

impl Simulation {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        let world = World::random(rng);
        let ga = ga::GeneticAlgorithm::new(
            ga::RouletteWheelSelection::default(),
            ga::UniformCrossover::default(),
            ga::GaussianMutation::new(0.01, 0.3),
        );
        Self { world, age: 0, ga }
    }
    pub fn world(&self) -> &World {
        &self.world
    }

    pub fn step(&mut self, rng: &mut dyn RngCore) -> Option<ga::Statistics> {
        self.process_collisions(rng);
        self.process_brain();
        self.process_movements();
        self.age += 1;

        if self.age > GENERATION_LENGTH {
            Some(self.evolve(rng))
        } else {
            None
        }
    }

    pub fn train(&mut self, rng: &mut dyn RngCore) -> ga::Statistics {
        loop {
            if let Some(summary) = self.step(rng) {
                return summary;
            }
        }
    }

    pub fn process_movements(&mut self) {
        for animal in &mut self.world.animals {
            animal.position += animal.rotation * na::Vector2::new(0.0, animal.speed);
            animal.position.x = na::wrap(animal.position.x, 0.0, 1.0);
            animal.position.y = na::wrap(animal.position.y, 0.0, 1.0);
        }
    }
    pub fn process_collisions(&mut self, rng: &mut dyn RngCore) {
        for animal in &mut self.world.animals {
            for food in &mut self.world.foods {
                let distance = na::distance(&animal.position, &food.position);
                if distance <= 0.01 {
                    animal.satiation += 1;
                    food.position = rng.gen();
                }
            }
        }
    }
    fn process_brain(&mut self) {
        for animal in &mut self.world.animals {
            let vision =
                animal
                    .eye
                    .process_vision(animal.position, animal.rotation, &self.world.foods);
            let response = animal.brain.nn.propagate(vision);
            // ---
            // | Limits number to given range.
            // -------------------- v---v
            let speed = response[0].clamp(-SPEED_ACCEL, SPEED_ACCEL);
            let rotation = response[1].clamp(-ROTATION_ACCEL, ROTATION_ACCEL);
            // Our speed & rotation here are *relative* - that is: when
            // they are equal to zero, what the brain says is "keep
            // flying as you are now", not "stop flying".
            //
            // Both values being relative is crucial, because our bird's
            // brain doesn't know its own speed and rotation*, meaning
            // that it fundamentally cannot return absolute values.
            //
            // * they'd have to be provided as separate inputs to the
            //   neural network, which would make the evolution process
            //   waaay longer, if even possible.
            animal.speed = (animal.speed + speed).clamp(SPEED_MIN, SPEED_MAX);
            animal.rotation = na::Rotation2::new(animal.rotation.angle() + rotation);
            // (btw, there is no need for ROTATION_MIN or ROTATION_MAX,
            // because rotation automatically wraps from 2*PI back to 0 -
            // we've already witnessed that when we were testing eyes,
            // inside `mod different_rotations { ... }`.)
        }
    }
    fn evolve(&mut self, rng: &mut dyn RngCore) -> ga::Statistics {
        self.age = 0;

        // Step 1: Prepare birdies to be sent into the genetic algorithm
        let current_population: Vec<AnimalIndividual> = self
            .world
            .animals
            .iter()
            .map(AnimalIndividual::from_animal)
            .collect();

        // Step 2: Evolve birdies
        let (evolved_population, stats) = self.ga.evolve(rng, &current_population);

        // Step 3: Bring birdies back from the genetic algorithm
        self.world.animals = evolved_population
            .into_iter()
            .map(|individual| individual.into_animal(rng))
            .collect();

        // Step 4: Restart foods
        //
        // (this is not strictly necessary, but it allows to easily spot
        // when the evolution happens - so it's more of a UI thing.)
        for food in &mut self.world.foods {
            food.position = rng.gen();
        }
        stats
    }
}

#[cfg(test)]
mod tests {}
