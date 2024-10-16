use std::fs::read_to_string;

use anyhow::Result;
use evolutionary_algorithm::{
    collector::CSVLogger,
    crossover::{CrossoverOperator, OrderedCrossover, PartiallyMappedCrossover},
    evolutionary_algorithm::{EvolutionaryAlgorithm, EvolutionaryAlgorithmBuilder, GenerationInfo},
    greedy_algorithm::GreedyAlgorithm,
    mutation::{InverseMutation, Mutation},
    problem::Problem,
    problem_loader,
    selection::{RouletteSelector, Selector, TournamentSelector},
    solver::Solver,
};

fn get_configuration<'a>(
    // selection_operator: Box<&'a dyn Selector>,
    problem: &'a dyn Problem, // tournament_selector: Box
                              // mutation_operator: Box<dyn Mutation>,
                              // crossover_operator: CrossoverOperator,
) -> Result<Vec<EvolutionaryAlgorithm<'a>>> {
    let generation_info_headers = vec![
        "generation".to_string(),
        "best_fitness".to_string(),
        "worst_fitness".to_string(),
        "average_fitness".to_string(),
        "mutations".to_string(),
        "crossovers".to_string(),
        "population_size".to_string(),
    ];

    Ok(vec![EvolutionaryAlgorithmBuilder::new()
        .population_size(50)
        .generations(1000)
        .crossover_prob(0.2)
        .mutation_prob(0.3)
        .logger(Box::new(CSVLogger::new(
            String::from("test1.csv"),
            generation_info_headers,
        )))
        .crossover_operator(CrossoverOperator::SingleChildCrossoverOperator(Box::new(
            OrderedCrossover {},
        )))
        .mutation_operator(Box::new(InverseMutation {}))
        .selection_operator(Box::new(TournamentSelector::new(5, problem)))
        // .selection_operator(Box::new(&roullette_selector))
        .build()?])
}

fn main() {
    let test_problem = read_to_string("./src/problem-instances/A-n32-k5.txt").unwrap();
    // let test_problem = read_to_string("./src/problem-instances/test.txt").unwrap();
    let mut problem = problem_loader::CVRProblem::from(test_problem);
    problem.precalculate_distances();

    let selector = TournamentSelector::new(5, &problem);
    let generation_info_headers = vec![
        "generation".to_string(),
        "best_fitness".to_string(),
        "worst_fitness".to_string(),
        "average_fitness".to_string(),
        "mutations".to_string(),
        "crossovers".to_string(),
        "population_size".to_string(),
    ];

    let roulette_selector = RouletteSelector::new(&problem);

    let logger: CSVLogger<GenerationInfo> =
        CSVLogger::new(String::from("logs.csv"), generation_info_headers);

    // let configuration: Vec<EvolutionaryAlgorithm> = vec![EvolutionaryAlgorithmBuilder::new()
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
    //     .build()?];

    let mut solver = EvolutionaryAlgorithmBuilder::new()
        .population_size(50)
        .generations(1000)
        .crossover_prob(0.2)
        .mutation_prob(0.3)
        .logger(Box::new(logger))
        .crossover_operator(CrossoverOperator::SingleChildCrossoverOperator(Box::new(
            OrderedCrossover {},
        )))
        // .crossover_operator(CrossoverOperator::TwoChildrenCrossoverOperator(Box::new(
        //     PartiallyMappedCrossover {},
        // )))
        .mutation_operator(Box::new(InverseMutation {}))
        .selection_operator(Box::new(&selector))
        // .selection_operator(Box::new(&roullette_selector))
        .build()
        .expect("Failed to create EA");

    let mut greedy = GreedyAlgorithm::new(&problem);
    let greedy_solution = greedy.solve().expect("Failed to greedy");

    println!("Greedy {:?}", greedy_solution);
    println!(
        "Greedy quality {:?}",
        problem.eval(&greedy_solution).unwrap()
    );

    match solver.solve(&problem) {
        Err(err) => {
            println!("Failed to solve test data {}", err)
        }
        Ok(val) => {
            println!("Solution {:?}", problem.serialize_indiviual(&val));
            println!("Quality {:?}", problem.eval(&val).unwrap());
        }
    }
}
