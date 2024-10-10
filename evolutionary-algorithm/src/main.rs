use std::fs::read_to_string;

use evolutionary_algorithm::{
    evolutionary_algorithm::EvolutionaryAlgorithm, greedy_algorithm::GreedyAlgorithm,
    problem::Problem, problem_loader, selection::TournamentSelector, solver::Solver,
};

fn main() {
    let test_problem = read_to_string("./src/problem-instances/test.txt").unwrap();
    let mut problem = problem_loader::CVRProblem::from(test_problem);
    problem.precalculate_distances();

    let selector = TournamentSelector::new(5, &problem);

    let mut solver = EvolutionaryAlgorithm::new(30000, 80, &selector);

    let mut greedy = GreedyAlgorithm::new(&problem);

    let greedy_solution = greedy.solve().expect("Failed to greedy");

    println!("Greedy {:#?}", greedy_solution);

    match solver.solve(&problem) {
        Err(err) => {
            println!("Failed to solve test data {}", err)
        }
        Ok(val) => {
            println!("Solution {:?}", val);
            println!("Quality {:?}", problem.eval(&val).unwrap());
        }
    }
    println!("Problem {:?}", problem);
}
