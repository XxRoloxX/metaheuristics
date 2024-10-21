use std::fs::read_to_string;

use anyhow::Result;
use evolutionary_algorithm::{
    collector::{inverse_fitness, CSVLogger, PersistableLogger},
    crossover::{CrossoverOperator, OrderedCrossover, PartiallyMappedCrossover},
    evolutionary_algorithm::{EvolutionaryAlgorithm, EvolutionaryAlgorithmBuilder, GenerationInfo},
    greedy_algorithm::GreedyAlgorithm,
    mutation::{InverseMutation, Mutation, SwapMutation},
    problem::Problem,
    problem_loader,
    runners::run_comparisons,
    selection::{RouletteSelector, Selector, TournamentSelector},
    solver::Solver,
};

fn get_configuration(instance: &str) -> Result<Vec<EvolutionaryAlgorithm>> {
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
        EvolutionaryAlgorithmBuilder::new()
            .population_size(500)
            .generations(3000)
            .crossover_prob(0.5)
            .mutation_prob(0.8)
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

fn main() {
    let instances: Vec<&str> = vec![
        "./src/problem-instances/A-n32-k5.txt",
        // "./src/problem-instances/A-n37-k5.txt",
        // "./src/problem-instances/A-n39-k5.txt",
        // "./src/problem-instances/A-n45-k7.txt",
        // "./src/problem-instances/A-n48-k7.txt",
        // "./src/problem-instances/A-n54-k7.txt",
        // "./src/problem-instances/A-n60-k9.txt",
    ];

    for instance in instances.iter() {
        get_configuration(instance.split('/').last().unwrap())
            .unwrap()
            .iter_mut()
            .for_each(|configuration| {
                let problem_contents = read_to_string(instance).unwrap();
                let mut problem = problem_loader::CVRProblem::from(problem_contents);
                problem.precalculate_distances();
                match configuration.solve(&problem) {
                    Err(err) => {
                        println!("failed to solve test data {}", err)
                    }
                    Ok(val) => {
                        println!("Solved at: {}", inverse_fitness(val.0))
                    }
                }
            });
    }

    // run_comparisons()
}
