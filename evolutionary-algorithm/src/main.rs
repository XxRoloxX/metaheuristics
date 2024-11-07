use pprof::protos::Message;
use std::io::Write;
// use protobuf::Message;
use std::fs::{read_to_string, File};

use evolutionary_algorithm::{
    logger::inverse_fitness, problem_loader, runners::run_comparisons,
    simulated_annealing::tests::get_simulated_annealing_config,
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
    // let guard = pprof::ProfilerGuardBuilder::default()
    //     .frequency(1000)
    //     // .blocklist(&["libc", "libgcc", "pthread", "vdso"])
    //     .build()
    //     .unwrap();

    for instance in instances.iter() {
        get_simulated_annealing_config(&format!(
            "./csv/simulated-annealing-{}",
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

    // run_comparisons()
}
