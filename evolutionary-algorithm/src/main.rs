use std::fs::read_to_string;

use evolutionary_algorithm::{
    evolutionary_algorithm::EvolutionaryAlgorithm, greedy_algorithm::GreedyAlgorithm,
    problem::Problem, problem_loader, selection::TournamentSelector, solver::Solver,
};

fn main() {
    let test_problem = read_to_string("./src/problem-instances/A-n32-k5.txt").unwrap();
    // let test_problem = read_to_string("./src/problem-instances/test.txt").unwrap();
    let mut problem = problem_loader::CVRProblem::from(test_problem);
    problem.precalculate_distances();

    let selector = TournamentSelector::new(3, &problem);
    // let test_solution = VecIndividual::from(&vec![1, 4, 3, 2, 5]);

    let mut solver = EvolutionaryAlgorithm::new(5000, 300, &selector);
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
