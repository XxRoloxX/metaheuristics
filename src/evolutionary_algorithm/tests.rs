use anyhow::Result;

use crate::logger::CSVLogger;

use super::{
    algorithm::{EvolutionaryAlgorithm, EvolutionaryAlgorithmBuilder},
    crossover::{CrossoverOperator, OrderedCrossover, PartiallyMappedCrossover},
    mutation::{InverseMutation, SwapMutation},
    selection::{RouletteSelector, TournamentSelector},
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
        Box::new(
            EvolutionaryAlgorithmBuilder::new()
                .population_size(100)
                .generations(500)
                .crossover_prob(0.7)
                .mutation_prob(0.3)
                .logger(Box::new(CSVLogger::new(
                    instance,
                    Some(generation_info_headers),
                )))
                .crossover_operator(CrossoverOperator::SingleChildCrossoverOperator(Box::new(
                    OrderedCrossover {},
                )))
                .mutation_operator(Box::new(SwapMutation {}))
                .selection_operator(Box::new(TournamentSelector::new(5)))
                .build()?,
        ),
        Box::new(
            EvolutionaryAlgorithmBuilder::new()
                .population_size(300)
                .generations(500)
                .crossover_prob(0.7)
                .mutation_prob(0.3)
                .logger(Box::new(CSVLogger::new(instance, None)))
                .crossover_operator(CrossoverOperator::SingleChildCrossoverOperator(Box::new(
                    OrderedCrossover {},
                )))
                .mutation_operator(Box::new(InverseMutation {}))
                .selection_operator(Box::new(TournamentSelector::new(5)))
                .build()?,
        ),
        Box::new(
            EvolutionaryAlgorithmBuilder::new()
                .population_size(500)
                .generations(500)
                .crossover_prob(0.7)
                .mutation_prob(0.3)
                .logger(Box::new(CSVLogger::new(instance, None)))
                .crossover_operator(CrossoverOperator::SingleChildCrossoverOperator(Box::new(
                    OrderedCrossover {},
                )))
                .mutation_operator(Box::new(InverseMutation {}))
                .selection_operator(Box::new(TournamentSelector::new(5)))
                .build()?,
        ),
        Box::new(
            EvolutionaryAlgorithmBuilder::new()
                .population_size(300)
                .generations(500)
                .crossover_prob(0.7)
                .mutation_prob(0.5)
                .logger(Box::new(CSVLogger::new(instance, None)))
                .crossover_operator(CrossoverOperator::SingleChildCrossoverOperator(Box::new(
                    OrderedCrossover {},
                )))
                .mutation_operator(Box::new(SwapMutation {}))
                .selection_operator(Box::new(TournamentSelector::new(5)))
                .build()?,
        ),
        Box::new(
            EvolutionaryAlgorithmBuilder::new()
                .population_size(300)
                .generations(500)
                .crossover_prob(0.7)
                .mutation_prob(0.5)
                .logger(Box::new(CSVLogger::new(instance, None)))
                .crossover_operator(CrossoverOperator::SingleChildCrossoverOperator(Box::new(
                    OrderedCrossover {},
                )))
                .mutation_operator(Box::new(SwapMutation {}))
                .selection_operator(Box::new(TournamentSelector::new(5)))
                .build()?,
        ),
        Box::new(
            EvolutionaryAlgorithmBuilder::new()
                .population_size(500)
                .generations(500)
                .crossover_prob(0.7)
                .mutation_prob(0.5)
                .logger(Box::new(CSVLogger::new(instance, None)))
                .crossover_operator(CrossoverOperator::SingleChildCrossoverOperator(Box::new(
                    OrderedCrossover {},
                )))
                .mutation_operator(Box::new(SwapMutation {}))
                .selection_operator(Box::new(TournamentSelector::new(5)))
                .build()?,
        ),
        Box::new(
            EvolutionaryAlgorithmBuilder::new()
                .population_size(500)
                .generations(500)
                .crossover_prob(0.5)
                .mutation_prob(0.8)
                .logger(Box::new(CSVLogger::new(instance, None)))
                .crossover_operator(CrossoverOperator::SingleChildCrossoverOperator(Box::new(
                    OrderedCrossover {},
                )))
                .mutation_operator(Box::new(SwapMutation {}))
                .selection_operator(Box::new(TournamentSelector::new(10)))
                .build()?,
        ),
        Box::new(
            EvolutionaryAlgorithmBuilder::new()
                .population_size(200)
                .generations(500)
                .crossover_prob(0.2)
                .mutation_prob(0.2)
                .logger(Box::new(CSVLogger::new(instance, None)))
                .crossover_operator(CrossoverOperator::TwoChildrenCrossoverOperator(Box::new(
                    PartiallyMappedCrossover {},
                )))
                .mutation_operator(Box::new(SwapMutation {}))
                .selection_operator(Box::new(RouletteSelector::new()))
                .build()?,
        ),
        Box::new(
            EvolutionaryAlgorithmBuilder::new()
                .population_size(200)
                .generations(500)
                .crossover_prob(0.5)
                .mutation_prob(0.3)
                .logger(Box::new(CSVLogger::new(instance, None)))
                .crossover_operator(CrossoverOperator::SingleChildCrossoverOperator(Box::new(
                    OrderedCrossover {},
                )))
                .mutation_operator(Box::new(SwapMutation {}))
                .selection_operator(Box::new(RouletteSelector::new()))
                .build()?,
        ),
    ])
}

pub fn get_ea_crossover_configuration(instance: &str) -> Result<Vec<Box<EvolutionaryAlgorithm>>> {
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
                .population_size(100)
                .generations(500)
                .crossover_prob(0.7)
                .mutation_prob(0.3)
                .logger(Box::new(CSVLogger::new(
                    instance,
                    Some(generation_info_headers),
                )))
                .crossover_operator(CrossoverOperator::SingleChildCrossoverOperator(Box::new(
                    OrderedCrossover {},
                )))
                .mutation_operator(Box::new(SwapMutation {}))
                .selection_operator(Box::new(TournamentSelector::new(5)))
                .build()?,
        ),
        Box::new(
            EvolutionaryAlgorithmBuilder::new()
                .population_size(300)
                .generations(500)
                .crossover_prob(0.3)
                .mutation_prob(0.3)
                .logger(Box::new(CSVLogger::new(instance, None)))
                .crossover_operator(CrossoverOperator::SingleChildCrossoverOperator(Box::new(
                    OrderedCrossover {},
                )))
                .mutation_operator(Box::new(SwapMutation {}))
                .selection_operator(Box::new(TournamentSelector::new(5)))
                .build()?,
        ),
        Box::new(
            EvolutionaryAlgorithmBuilder::new()
                .population_size(500)
                .generations(500)
                .crossover_prob(0.7)
                .mutation_prob(0.3)
                .logger(Box::new(CSVLogger::new(instance, None)))
                .crossover_operator(CrossoverOperator::TwoChildrenCrossoverOperator(Box::new(
                    PartiallyMappedCrossover {},
                )))
                .mutation_operator(Box::new(SwapMutation {}))
                .selection_operator(Box::new(TournamentSelector::new(5)))
                .build()?,
        ),
        Box::new(
            EvolutionaryAlgorithmBuilder::new()
                .population_size(500)
                .generations(500)
                .crossover_prob(0.3)
                .mutation_prob(0.3)
                .logger(Box::new(CSVLogger::new(instance, None)))
                .crossover_operator(CrossoverOperator::TwoChildrenCrossoverOperator(Box::new(
                    PartiallyMappedCrossover {},
                )))
                .mutation_operator(Box::new(SwapMutation {}))
                .selection_operator(Box::new(TournamentSelector::new(5)))
                .build()?,
        ),
    ])
}

pub fn get_ea_mutation_configuration(instance: &str) -> Result<Vec<Box<EvolutionaryAlgorithm>>> {
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
                .population_size(100)
                .generations(500)
                .crossover_prob(0.7)
                .mutation_prob(0.3)
                .logger(Box::new(CSVLogger::new(
                    instance,
                    Some(generation_info_headers),
                )))
                .crossover_operator(CrossoverOperator::SingleChildCrossoverOperator(Box::new(
                    OrderedCrossover {},
                )))
                .mutation_operator(Box::new(SwapMutation {}))
                .selection_operator(Box::new(TournamentSelector::new(5)))
                .build()?,
        ),
        Box::new(
            EvolutionaryAlgorithmBuilder::new()
                .population_size(300)
                .generations(500)
                .crossover_prob(0.3)
                .mutation_prob(0.7)
                .logger(Box::new(CSVLogger::new(instance, None)))
                .crossover_operator(CrossoverOperator::SingleChildCrossoverOperator(Box::new(
                    OrderedCrossover {},
                )))
                .mutation_operator(Box::new(SwapMutation {}))
                .selection_operator(Box::new(TournamentSelector::new(5)))
                .build()?,
        ),
        Box::new(
            EvolutionaryAlgorithmBuilder::new()
                .population_size(500)
                .generations(500)
                .crossover_prob(0.3)
                .mutation_prob(0.3)
                .logger(Box::new(CSVLogger::new(instance, None)))
                .crossover_operator(CrossoverOperator::SingleChildCrossoverOperator(Box::new(
                    OrderedCrossover {},
                )))
                .mutation_operator(Box::new(InverseMutation {}))
                .selection_operator(Box::new(TournamentSelector::new(5)))
                .build()?,
        ),
        Box::new(
            EvolutionaryAlgorithmBuilder::new()
                .population_size(500)
                .generations(500)
                .crossover_prob(0.3)
                .mutation_prob(0.7)
                .logger(Box::new(CSVLogger::new(instance, None)))
                .crossover_operator(CrossoverOperator::SingleChildCrossoverOperator(Box::new(
                    OrderedCrossover {},
                )))
                .mutation_operator(Box::new(InverseMutation {}))
                .selection_operator(Box::new(TournamentSelector::new(5)))
                .build()?,
        ),
    ])
}

pub fn get_ea_best_three(instance: &str) -> Result<Vec<Box<EvolutionaryAlgorithm>>> {
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
                .population_size(500)
                .generations(500)
                .crossover_prob(0.7)
                .mutation_prob(0.6)
                .logger(Box::new(CSVLogger::new(
                    instance,
                    Some(generation_info_headers),
                )))
                .crossover_operator(CrossoverOperator::SingleChildCrossoverOperator(Box::new(
                    OrderedCrossover {},
                )))
                .mutation_operator(Box::new(SwapMutation {}))
                .selection_operator(Box::new(TournamentSelector::new(10)))
                .build()?,
        ),
        Box::new(
            EvolutionaryAlgorithmBuilder::new()
                .population_size(400)
                .generations(500)
                .crossover_prob(0.7)
                .mutation_prob(0.6)
                .logger(Box::new(CSVLogger::new(instance, None)))
                .crossover_operator(CrossoverOperator::SingleChildCrossoverOperator(Box::new(
                    OrderedCrossover {},
                )))
                .mutation_operator(Box::new(SwapMutation {}))
                .selection_operator(Box::new(TournamentSelector::new(20)))
                .build()?,
        ),
        Box::new(
            EvolutionaryAlgorithmBuilder::new()
                .population_size(500)
                .generations(500)
                .crossover_prob(0.7)
                .mutation_prob(0.8)
                .logger(Box::new(CSVLogger::new(instance, None)))
                .crossover_operator(CrossoverOperator::SingleChildCrossoverOperator(Box::new(
                    OrderedCrossover {},
                )))
                .mutation_operator(Box::new(SwapMutation {}))
                .selection_operator(Box::new(TournamentSelector::new(30)))
                .build()?,
        ),
    ])
}
