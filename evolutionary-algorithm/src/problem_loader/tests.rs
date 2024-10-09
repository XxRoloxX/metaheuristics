use std::fs::read_to_string;

use super::Problem;

#[test]
fn eval_test_cvrp() {
    let test_problem = read_to_string("./src/problem-instances/test.txt").unwrap();
    let mut problem = crate::problem_loader::CVRProblem::from(test_problem);
    problem.precalculate_distances();

    let solution = vec![0u16, 4u16, 3u16, 4u16, 5u16, 0u16];

    match problem.eval(&solution) {
        Ok(val) => assert_eq!(val, 265f32),
        Err(err) => {
            println!("{}", err);
        }
    };
}
