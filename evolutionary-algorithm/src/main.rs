use std::fs::read_to_string;

use evolutionary_algorithm::problem_loader;

fn main() {
    let test_problem = read_to_string("./src/problem-instances/test.txt").unwrap();
    let mut problem = problem_loader::CVRProblem::from(test_problem);
    problem.precalculate_distances();

    println!("Problem {:?}", problem);
}
