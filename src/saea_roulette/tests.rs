use anyhow::Result;

use crate::{
    evolutionary_algorithm::{
        algorithm::{EvolutionaryAlgorithm, EvolutionaryAlgorithmBuilder},
        crossover::{CrossoverOperator, OrderedCrossover},
        mutation::SwapMutation,
        selection::{RouletteSelector, SimulatedAnnealingRouletteSelector, TournamentSelector},
    },
    logger::CSVLogger,
};

pub fn get_saea_configuration(instance: &str) -> Result<Vec<Box<EvolutionaryAlgorithm>>> {
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
        Box::new(
            EvolutionaryAlgorithmBuilder::new()
                .population_size(200)
                .generations(1000)
                .crossover_prob(0.6)
                .mutation_prob(0.3)
                .logger(Box::new(CSVLogger::new(
                    instance,
                    Some(generation_info_headers),
                )))
                .crossover_operator(CrossoverOperator::SingleChildCrossoverOperator(Box::new(
                    OrderedCrossover {},
                )))
                .mutation_operator(Box::new(SwapMutation {}))
                .selection_operator(Box::new(SimulatedAnnealingRouletteSelector::new(1.0, 0.99)))
                .build()?,
        ),
        Box::new(
            EvolutionaryAlgorithmBuilder::new()
                .population_size(500)
                .generations(1000)
                .crossover_prob(0.7)
                .mutation_prob(0.2)
                .logger(Box::new(CSVLogger::new(instance, None)))
                .crossover_operator(CrossoverOperator::SingleChildCrossoverOperator(Box::new(
                    OrderedCrossover {},
                )))
                .mutation_operator(Box::new(SwapMutation {}))
                .selection_operator(Box::new(SimulatedAnnealingRouletteSelector::new(
                    1.0, 0.999,
                )))
                .build()?,
        ),
        Box::new(
            EvolutionaryAlgorithmBuilder::new()
                .population_size(200)
                .generations(1000)
                .crossover_prob(0.5)
                .mutation_prob(0.2)
                .logger(Box::new(CSVLogger::new(instance, None)))
                .crossover_operator(CrossoverOperator::SingleChildCrossoverOperator(Box::new(
                    OrderedCrossover {},
                )))
                .mutation_operator(Box::new(SwapMutation {}))
                .selection_operator(Box::new(RouletteSelector {}))
                .build()?,
        ),
        Box::new(
            EvolutionaryAlgorithmBuilder::new()
                .population_size(200)
                .generations(1000)
                .crossover_prob(0.5)
                .mutation_prob(0.2)
                .logger(Box::new(CSVLogger::new(instance, None)))
                .crossover_operator(CrossoverOperator::SingleChildCrossoverOperator(Box::new(
                    OrderedCrossover {},
                )))
                .mutation_operator(Box::new(SwapMutation {}))
                .selection_operator(Box::new(SimulatedAnnealingRouletteSelector::new(
                    1.0, 0.999,
                )))
                .build()?,
        ),
        Box::new(
            EvolutionaryAlgorithmBuilder::new()
                .population_size(300)
                .generations(1000)
                .crossover_prob(0.8)
                .mutation_prob(0.2)
                .logger(Box::new(CSVLogger::new(instance, None)))
                .crossover_operator(CrossoverOperator::SingleChildCrossoverOperator(Box::new(
                    OrderedCrossover {},
                )))
                .mutation_operator(Box::new(SwapMutation {}))
                .selection_operator(Box::new(SimulatedAnnealingRouletteSelector::new(
                    2.0, 0.999,
                )))
                .build()?,
        ),
    ])
}
