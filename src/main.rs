use std::fs::read_to_string;

use evolutionary_algorithm::{
    evolutionary_algorithm::tests::get_ea_best_three, logger::inverse_fitness, problem_loader,
    runners::run_comparisons, saea_roulette::tests::get_saea_configuration,
    simulated_annealing::tests::get_simulated_annealing_best_three, solver::Solver,
    tabu_search::tests::get_tabu_search_best_three, tssa::tests::get_tssa_config,
};

fn run_simulated_annealing_precision(instances: &Vec<&str>) {
    for instance in instances.iter() {
        get_simulated_annealing_best_three(&format!(
            "./csv/simulated-annealing-best-{}",
            instance
                .split('/')
                .last()
                .into_iter()
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
}
fn run_simulated_tabu_seatch(instances: &Vec<&str>) {
    for instance in instances.iter() {
        get_tabu_search_best_three(&format!(
            "./csv/tabu-search-best-{}",
            instance
                .split('/')
                .last()
                .into_iter()
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
}

fn run_evolutionary_algorithm(instances: &Vec<&str>) {
    for instance in instances.iter() {
        get_ea_best_three(&format!(
            "./csv/ea-best-{}",
            instance
                .split('/')
                .last()
                .into_iter()
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
}

fn run_tssa(instances: &Vec<&str>) {
    for instance in instances.iter() {
        get_tssa_config(&format!(
            "./csv/tssa-best-{}",
            instance
                .split('/')
                .last()
                .into_iter()
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
}

fn run_saea(instances: &Vec<&str>) {
    for instance in instances.iter() {
        get_saea_configuration(&format!(
            "./csv/saea-best-{}",
            instance
                .split('/')
                .last()
                .into_iter()
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
}

fn main() {
    let instances: Vec<&str> = vec![
        "./csv/A-n32-k5",
        "./csv/A-n37-k6",
        "./csv/A-n39-k5",
        "./csv/A-n45-k6",
        "./csv/A-n48-k7",
        "./csv/A-n54-k7",
        "./csv/A-n60-k9",
    ];

    run_simulated_annealing_precision(&instances);

    run_comparisons()
}
