// use std::fs::read_to_string;

use crate::individual::{Gene, VecIndividual};

// use super::Problem;
//
// #[test]
// fn eval_test_cvrp() {
//     let test_problem = read_to_string("./src/problem-instances/test.txt").unwrap();
//     let mut problem = crate::problem_loader::CVRProblem::from(test_problem);
//     problem.precalculate_distances();
//
//     let test_solution: Vec<Gene> = vec![0u16, 4u16, 3u16, 4u16, 5u16, 0u16];
//     let solution = VecIndividual::from(&test_solution);
//
//     match problem.eval(&solution) {
//         Ok(val) => assert_eq!(val, 265f32),
//         Err(err) => {
//             println!("{}", err);
//         }
//     };
// }

#[test]
fn random_index_test() {
    let individual = VecIndividual::from(&vec![1, 2, 3, 4, 5, 6]);

    for _ in 0..100 {
        let (start, end) = individual.random_gene_range_indexes();
        // println!("{}: {}", start, end);
        assert!(start < end, "start index is smaller than end index");
        assert!(
            start < individual.number_of_genes() - 1,
            "start index is not the last one"
        );
        assert!(
            end < individual.number_of_genes(),
            "end index is smaller than length"
        );
    }
}
