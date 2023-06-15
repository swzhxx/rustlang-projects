use rand::seq::SliceRandom;

use crate::{individual, SelectionMethod};

#[derive(Clone, Debug, Default)]
pub struct RouletteWheelSelection;
impl RouletteWheelSelection {
    pub fn new() -> Self {
        Self
    }
}

impl SelectionMethod for RouletteWheelSelection {
    fn select<'a, I>(&self, rng: &mut dyn rand::RngCore, population: &'a [I]) -> &'a I
    where
        I: crate::Individual,
    {
        assert!(!population.is_empty());
        population
            .choose_weighted(rng, |individual| individual.fitness())
            .expect("got an empty population")
    }
}

#[cfg(test)]
mod test {
    use std::collections::BTreeMap;

    use crate::Individual;

    use super::*;
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    #[derive(Clone, Debug)]
    pub struct TestIndividual {
        fitness: f32,
    }

    impl TestIndividual {
        pub fn new(fitness: f32) -> Self {
            Self { fitness }
        }
    }

    impl Individual for TestIndividual {
        fn fitness(&self) -> f32 {
            self.fitness
        }

        fn chromosome(&self) -> &crate::Chromosome {
            todo!()
        }

        fn create(chromosome: crate::Chromosome) -> Self {
            todo!()
        }
    }

    #[test]
    fn test() {
        let method = RouletteWheelSelection::new();
        let mut rng = ChaCha8Rng::from_seed(Default::default());

        let population = vec![
            TestIndividual::new(2.0),
            TestIndividual::new(1.0),
            TestIndividual::new(4.0),
            TestIndividual::new(3.0),
        ];

        let mut actual_histogram = BTreeMap::new();

        //               there is nothing special about this thousand;
        //          v--v a number as low as fifty might do the trick, too
        for _ in 0..1000 {
            let fitness = method.select(&mut rng, &population).fitness() as i32;

            *actual_histogram.entry(fitness).or_insert(0) += 1;
        }

        let expected_histogram = BTreeMap::from_iter(vec![
            // (fitness, how many times this fitness has been chosen)
            (1, 0),
            (2, 0),
            (3, 0),
            (4, 0),
        ]);

        assert_eq!(actual_histogram, expected_histogram);
    }
}
