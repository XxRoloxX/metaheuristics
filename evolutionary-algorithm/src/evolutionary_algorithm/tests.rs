use anyhow::Result;

use crate::logger::CSVLogger;

use super::{
    algorithm::{EvolutionaryAlgorithm, EvolutionaryAlgorithmBuilder},
    crossover::{CrossoverOperator, OrderedCrossover},
    mutation::SwapMutation,
    selection::TournamentSelector,
};

pub fn get_ea_configuration(instance: &str) -> Result<Vec<Box<EvolutionaryAlgorithm>>> {
    let generation_info_headers = vec![
        "configuration".to_string(),
        "generation".to_string(),
        "best_fitness".to_string(),
        "average_fitness".to_string(),
        "worst_fitness".to_string(),
        "mutations".to_string(),
        "crossovers".to_string(),
        "population_size".to_string(),
    ];

    Ok(vec![
        // EvolutionaryAlgorithmBuilder::new()
        //     .population_size(100)
        //     .generations(1000)
        //     .crossover_prob(0.7)
        //     .mutation_prob(0.3)
        //     .logger(Box::new(CSVLogger::new(
        //         instance,
        //         Some(generation_info_headers),
        //     )))
        //     .crossover_operator(CrossoverOperator::SingleChildCrossoverOperator(Box::new(
        //         OrderedCrossover {},
        //     )))
        //     .mutation_operator(Box::new(SwapMutation {}))
        //     .selection_operator(Box::new(TournamentSelector::new(5)))
        //     .build()?,
        // EvolutionaryAlgorithmBuilder::new()
        //     .population_size(300)
        //     .generations(1000)
        //     .crossover_prob(0.7)
        //     .mutation_prob(0.3)
        //     .logger(Box::new(CSVLogger::new(instance, None)))
        //     .crossover_operator(CrossoverOperator::SingleChildCrossoverOperator(Box::new(
        //         OrderedCrossover {},
        //     )))
        //     .mutation_operator(Box::new(InverseMutation {}))
        //     .selection_operator(Box::new(TournamentSelector::new(5)))
        //     .build()?,
        // EvolutionaryAlgorithmBuilder::new()
        //     .population_size(500)
        //     .generations(1000)
        //     .crossover_prob(0.7)
        //     .mutation_prob(0.3)
        //     .logger(Box::new(CSVLogger::new(instance, None)))
        //     .crossover_operator(CrossoverOperator::SingleChildCrossoverOperator(Box::new(
        //         OrderedCrossover {},
        //     )))
        //     .mutation_operator(Box::new(InverseMutation {}))
        //     .selection_operator(Box::new(TournamentSelector::new(5)))
        //     .build()?,
        // EvolutionaryAlgorithmBuilder::new()
        //     .population_size(100)
        //     .generations(1000)
        //     .crossover_prob(0.7)
        //     .mutation_prob(0.3)
        //     .logger(Box::new(CSVLogger::new(instance, None)))
        //     .crossover_operator(CrossoverOperator::TwoChildrenCrossoverOperator(Box::new(
        //         PartiallyMappedCrossover {},
        //     )))
        //     .mutation_operator(Box::new(SwapMutation {}))
        //     .selection_operator(Box::new(TournamentSelector::new(5)))
        //     .build()?,
        // EvolutionaryAlgorithmBuilder::new()
        //     .population_size(300)
        //     .generations(500)
        //     .crossover_prob(0.7)
        //     .mutation_prob(0.3)
        //     .logger(Box::new(CSVLogger::new(instance, None)))
        //     .crossover_operator(CrossoverOperator::TwoChildrenCrossoverOperator(Box::new(
        //         PartiallyMappedCrossover {},
        //     )))
        //     .mutation_operator(Box::new(InverseMutation {}))
        //     .selection_operator(Box::new(TournamentSelector::new(5)))
        //     .build()?,
        // EvolutionaryAlgorithmBuilder::new()
        //     .population_size(500)
        //     .generations(500)
        //     .crossover_prob(0.7)
        //     .mutation_prob(0.3)
        //     .logger(Box::new(CSVLogger::new(instance, None)))
        //     .crossover_operator(CrossoverOperator::TwoChildrenCrossoverOperator(Box::new(
        //         PartiallyMappedCrossover {},
        //     )))
        //     .mutation_operator(Box::new(InverseMutation {}))
        //     .selection_operator(Box::new(TournamentSelector::new(5)))
        //     .build()?,
        // EvolutionaryAlgorithmBuilder::new()
        //     .population_size(100)
        //     .generations(500)
        //     .crossover_prob(0.7)
        //     .mutation_prob(0.5)
        //     .logger(Box::new(CSVLogger::new(instance, None)))
        //     .crossover_operator(CrossoverOperator::SingleChildCrossoverOperator(Box::new(
        //         OrderedCrossover {},
        //     )))
        //     .mutation_operator(Box::new(SwapMutation {}))
        //     .selection_operator(Box::new(TournamentSelector::new(5)))
        //     .build()?,
        // EvolutionaryAlgorithmBuilder::new()
        //     .population_size(300)
        //     .generations(500)
        //     .crossover_prob(0.7)
        //     .mutation_prob(0.5)
        //     .logger(Box::new(CSVLogger::new(instance, None)))
        //     .crossover_operator(CrossoverOperator::SingleChildCrossoverOperator(Box::new(
        //         OrderedCrossover {},
        //     )))
        //     .mutation_operator(Box::new(SwapMutation {}))
        //     .selection_operator(Box::new(TournamentSelector::new(5)))
        //     .build()?,
        // EvolutionaryAlgorithmBuilder::new()
        //     .population_size(500)
        //     .generations(500)
        //     .crossover_prob(0.7)
        //     .mutation_prob(0.5)
        //     .logger(Box::new(CSVLogger::new(
        //         instance, // Some(generation_info_headers),
        //         None,
        //     )))
        //     .crossover_operator(CrossoverOperator::SingleChildCrossoverOperator(Box::new(
        //         OrderedCrossover {},
        //     )))
        //     .mutation_operator(Box::new(SwapMutation {}))
        //     .selection_operator(Box::new(TournamentSelector::new(5)))
        //     .build()?,
        Box::new(
            EvolutionaryAlgorithmBuilder::new()
                .population_size(500)
                .generations(3000)
                .crossover_prob(0.5)
                .mutation_prob(0.8)
                .logger(Box::new(CSVLogger::new(
                    format!("{}-ea", instance).as_str(),
                    // instance,
                    Some(generation_info_headers),
                )))
                .crossover_operator(CrossoverOperator::SingleChildCrossoverOperator(Box::new(
                    OrderedCrossover {},
                )))
                .mutation_operator(Box::new(SwapMutation {}))
                .selection_operator(Box::new(TournamentSelector::new(5)))
                .build()?,
        ),
        //     EvolutionaryAlgorithmBuilder::new()
        //         .population_size(200)
        //         .generations(1000)
        //         .crossover_prob(0.2)
        //         .mutation_prob(0.2)
        //         .logger(Box::new(CSVLogger::new(instance, None)))
        //         .crossover_operator(CrossoverOperator::TwoChildrenCrossoverOperator(Box::new(
        //             PartiallyMappedCrossover {},
        //         )))
        //         .mutation_operator(Box::new(SwapMutation {}))
        //         .selection_operator(Box::new(RouletteSelector::new()))
        //         .build()?,
        //     EvolutionaryAlgorithmBuilder::new()
        //         .population_size(200)
        //         .generations(1000)
        //         .crossover_prob(0.5)
        //         .mutation_prob(0.3)
        //         .logger(Box::new(CSVLogger::new(instance, None)))
        //         .crossover_operator(CrossoverOperator::SingleChildCrossoverOperator(Box::new(
        //             OrderedCrossover {},
        //         )))
        //         .mutation_operator(Box::new(SwapMutation {}))
        //         .selection_operator(Box::new(RouletteSelector::new()))
        //         .build()?,
    ])
}
