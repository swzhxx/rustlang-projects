use rand::RngCore;

mod chromosome;
mod crossover;
mod individual;
mod mutation;
mod selection;
mod statistics;
pub use self::{
    chromosome::*, crossover::*, individual::*, mutation::*, selection::*, statistics::*,
};
pub struct GeneticAlgorithm<S> {
    selection_method: S,
    crossover_method: Box<dyn CrossoverMethod>,
    mutation_method: Box<dyn MutationMethod>,
}

impl<S> GeneticAlgorithm<S>
where
    S: SelectionMethod,
{
    pub fn new(
        selection_method: S,
        crossover_method: impl CrossoverMethod + 'static,
        mutation_method: impl MutationMethod + 'static,
    ) -> Self {
        Self {
            selection_method,
            crossover_method: Box::new(crossover_method),
            mutation_method: Box::new(mutation_method),
        }
    }
    pub fn evolve<I>(&self, rng: &mut dyn RngCore, population: &[I]) -> (Vec<I>, Statistics)
    where
        I: Individual,
    {
        assert!(!population.is_empty());
        let new_population = (0..population.len())
            .map(|_| {
                //  selection
                let parent_a = self.selection_method.select(rng, population).chromosome();
                let parent_b = self.selection_method.select(rng, population).chromosome();
                //  crossover
                let mut child = self.crossover_method.crossover(rng, parent_a, parent_b);
                //  mutation
                self.mutation_method.mutate(rng, &mut child);
                I::create(child)
            })
            .collect();
        let stats = Statistics::new(population);
        (new_population, stats)
    }
}

#[cfg(test)]
mod tests {}
