use std::fs::read_to_string;

use evolutionary_algorithm::{
    evolutionary_algorithm::tests::{
        get_ea_configuration, get_ea_crossover_configuration, get_ea_mutation_configuration,
    },
    logger::inverse_fitness,
    problem_loader,
    runners::run_comparisons,
    solver::Solver,
    tabu_search::tests::{
        get_tabu_search_general_configurations, get_tabu_search_neighbors,
        get_tabu_search_scores_comparison_configurations,
    },
};

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
        // get_ea_configuration(&format!(
        //     "./csv/ea-parameters-{}",
        get_tabu_search_neighbors(&format!(
            "./csv/tabu-neighbors-{}",
            instance
                .split('/')
                .last()
                .unwrap()
                .split('.')
                .collect::<Vec<&str>>()
                .first()
                .unwrap()
        ))
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
