use crate::{
    collector::PersistableLogger,
    crossover::CrossoverOperator,
    evolutionary_algorithm::{EvolutionaryAlgorithm, GenerationInfo},
    mutation::Mutation,
    selection::Selector,
};

// pub struct Configuration<'a> {
//     algorithm_config: EvolutionaryAlgorithm<'a>,
// }

// let mut solver = EvolutionaryAlgorithmBuilder::new()
//     .population_size(50)
//     .generations(1000)
//     .crossover_prob(0.2)
//     .mutation_prob(0.3)
//     .logger(Box::new(logger))
//     .crossover_operator(CrossoverOperator::SingleChildCrossoverOperator(Box::new(
//         OrderedCrossover {},
//     )))
//     // .crossover_operator(CrossoverOperator::TwoChildrenCrossoverOperator(Box::new(
//     //     PartiallyMappedCrossover {},
//     // )))
//     .mutation_operator(Box::new(InverseMutation {}))
//     .selection_operator(Box::new(&selector))
//     // .selection_operator(Box::new(&roullette_selector))
//     .build()
//     .expect("Failed to create EA");
