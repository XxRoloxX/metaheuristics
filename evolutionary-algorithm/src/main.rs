//use pprof::protos::Message;
//use std::collections::HashMap;
//use std::io::Write;
// use protobuf::Message;
use std::fs::read_to_string;

use evolutionary_algorithm::{
    evolutionary_algorithm::tests::get_ea_best_three,
    logger::inverse_fitness,
    problem_loader,
    runners::run_comparisons,
    simulated_annealing::tests::{
        get_simulated_annealing_best_three, get_simulated_annealing_config,
    },
    solver::Solver,
    tabu_search::tests::get_tabu_search_best_three,
    tssa::tests::get_tssa_config,
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

//fn run_tssa(insta)

fn main() {
    let instances: Vec<&str> = vec![
        "./csv/A-n32-k5",
        //"./csv/A-n37-k6",
        //"./csv/A-n39-k5",
        //"./csv/A-n45-k6",
        //"./csv/A-n48-k7",
        //"./csv/A-n54-k7",
        //"./csv/A-n60-k9",
    ];

    // let guard = pprof::ProfilerGuardBuilder::default()
    //     .frequency(1000)
    //     .blocklist(&["libc", "libgcc", "pthread", "vdso"])
    //     .build()
    //     .unwrap();
    //
    //run_simulated_annealing_precision(&instances);

    //for instance in instances.iter() {
    //    get_simulated_annealing_config(&format!(
    //        "./csv/simulated-annealing-precise-{}",
    //        instance
    //            .split('/')
    //            .last()
    //            .unwrap()
    //            .split('.')
    //            .collect::<Vec<&str>>()
    //            .first()
    //            .unwrap()
    //    ))
    //    .unwrap()
    //    .iter_mut()
    //    .for_each(|configuration| {
    //        let problem_contents = read_to_string(instance).unwrap();
    //        let mut problem = problem_loader::CVRProblem::from(problem_contents);
    //        problem.precalculate_distances();
    //        match configuration.solve(&problem) {
    //            Err(err) => {
    //                println!("failed to solve test data {}", err)
    //            }
    //            Ok(val) => {
    //                println!("Solved at: {}", inverse_fitness(val.0))
    //            }
    //        }
    //    });
    //}
    //
    // if let Ok(report) = guard.report().build() {
    //     let mut file = File::create("profile.pb").expect("Failed to create file");
    //     let profile = report.pprof().expect("Failed to create pprof");
    //
    //     let mut flames = File::create("flames.svg").expect("Failed to create flames");
    //
    //     report.flamegraph(&mut flames).expect("flames");
    //
    //     if let Err(er) = profile.write_to_writer(&mut file) {
    //         println!("Failed to write {}", er)
    //     }
    //
    //     /*
    //     let mut content = Vec::new();
    //     report.flame
    //     profile.encode(&mut content).unwrap();
    //     file.write_all(&content).unwrap();
    //     */
    //
    //     println!("report: {:?}", &report);
    // };
    // run_simulated_annealing_precision(&instances);
    // run_evolutionary_algorithm(&instances);
    run_tssa(&instances);

    //run_comparisons()
}
