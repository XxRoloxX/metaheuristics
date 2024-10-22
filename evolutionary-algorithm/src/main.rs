use std::fs::read_to_string;

use evolutionary_algorithm::{
    logger::inverse_fitness, problem_loader, runners::run_comparisons,
    tabu_search::tests::get_tabu_search_configuration,
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
        get_tabu_search_configuration(instance.split('/').last().unwrap())
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
