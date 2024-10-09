use std::fs::read_to_string;

use evolutionary_algorithm::{
    evolutionary_algorithm::EvolutionaryAlgorithm, problem::Problem, problem_loader,
    selection::TournamentSelector, solver::Solver,
};

fn main() {
    let test_problem = read_to_string("./src/problem-instances/test.txt").unwrap();
    let mut problem = problem_loader::CVRProblem::from(test_problem);
    problem.precalculate_distances();

    let selector = TournamentSelector::new(5, &problem);

    let mut solver = EvolutionaryAlgorithm::new(30000, 80, &selector);

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
