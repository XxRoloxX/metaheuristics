use std::fs::read_to_string;

use crate::{
    collector::{inverse_fitness, CSVEntry, CSVLogger, PersistableLogger},
    crossover::{CrossoverOperator, OrderedCrossover},
    evolutionary_algorithm::{EvolutionaryAlgorithm, EvolutionaryAlgorithmBuilder},
    greedy_algorithm::GreedyAlgorithm,
    individual::Fitness,
    mutation::SwapMutation,
    problem::Problem,
    problem_loader::CVRProblem,
    selection::TournamentSelector,
    solver::Solver,
};

pub struct Score {
    best: f32,
    worst: f32,
    avg: f32,
    std: f32,
}

impl Score {
    pub fn new(scores: Vec<Fitness>) -> Self {
        let inversed_scores: Vec<Fitness> =
            scores.iter().map(|score| inverse_fitness(*score)).collect();
        Score {
            best: *inversed_scores
                .iter()
                .min_by(|x, y| f32::total_cmp(x, y))
                .unwrap(),
            worst: *inversed_scores
                .iter()
                .max_by(|x, y| f32::total_cmp(x, y))
                .unwrap(),
            avg: inversed_scores.iter().sum::<Fitness>() / scores.len() as Fitness,
            std: calculate_std_dev(&inversed_scores),
        }
    }
}

pub struct ScoreSet {
    instance: String,
    ea: Score,
    random: Score,
    greedy: Score,
}

impl ScoreSet {
    pub fn headers() -> Vec<String> {
        vec![
            String::from("Instance"),
            String::from("Random: best"),
            String::from("Random: worst"),
            String::from("Random: avg"),
            String::from("Random: std"),
            String::from("Greedy: best"),
            String::from("Greedy: worst"),
            String::from("Greedy: avg"),
            String::from("Greedy: std"),
            String::from("Evolutionary: best"),
            String::from("Evolutionary: worst"),
            String::from("Evolutionary: avg"),
            String::from("Evolutionary: std"),
        ]
    }
}

impl From<&ScoreSet> for CSVEntry {
    fn from(score_set: &ScoreSet) -> CSVEntry {
        CSVEntry::from(vec![
            score_set.instance.to_string(),
            score_set.random.best.to_string(),
            score_set.random.worst.to_string(),
            score_set.random.avg.to_string(),
            score_set.random.std.to_string(),
            score_set.greedy.best.to_string(),
            score_set.greedy.worst.to_string(),
            score_set.greedy.avg.to_string(),
            score_set.greedy.std.to_string(),
            score_set.ea.best.to_string(),
            score_set.ea.worst.to_string(),
            score_set.ea.avg.to_string(),
            score_set.ea.std.to_string(),
        ])
    }
}

fn optimal_ea() -> EvolutionaryAlgorithm {
    EvolutionaryAlgorithmBuilder::new()
        .population_size(500)
        .generations(500)
        .crossover_prob(0.7)
        .mutation_prob(0.5)
        .logger(Box::new(CSVLogger::new("comparisons", None)))
        .crossover_operator(CrossoverOperator::SingleChildCrossoverOperator(Box::new(
            OrderedCrossover {},
        )))
        .mutation_operator(Box::new(SwapMutation {}))
        .selection_operator(Box::new(TournamentSelector::new(5)))
        .build()
        .unwrap()
}

fn test_ea(problem: &dyn Problem, repeats: u16) -> Vec<Fitness> {
    let mut ea = optimal_ea();
    (0..repeats)
        .map(|_| {
            let (score, _) = ea.solve(problem).unwrap();
            score
        })
        .collect::<Vec<Fitness>>()
}

fn test_greedy(problem: &CVRProblem, repeats: u16) -> Vec<Fitness> {
    let mut greedy = GreedyAlgorithm::new(problem);
    (0..repeats)
        .map(|_| {
            let solution = greedy.solve().unwrap();
            problem.eval(&solution).unwrap()
        })
        .collect::<Vec<Fitness>>()
}

fn test_random(problem: &CVRProblem, repeats: u16) -> Vec<Fitness> {
    (0..repeats)
        .map(|_| {
            let solution = problem.random_individual();
            problem.eval(&solution).unwrap()
        })
        .collect::<Vec<Fitness>>()
}

fn calculate_std_dev(data: &[Fitness]) -> Fitness {
    let mean: Fitness = data.iter().sum::<Fitness>() / data.len() as Fitness;
    let variance: f32 = data
        .iter()
        .map(|value| {
            let diff = value - mean;
            diff * diff
        })
        .sum::<f32>()
        / data.len() as f32;

    variance.sqrt()
}

pub fn run_comparisons() {
    let instances: Vec<&str> = vec![
        "./src/problem-instances/A-n32-k5.txt",
        "./src/problem-instances/A-n37-k5.txt",
        "./src/problem-instances/A-n39-k5.txt",
        "./src/problem-instances/A-n45-k7.txt",
        "./src/problem-instances/A-n48-k7.txt",
    ];

    let mut logger: CSVLogger<ScoreSet> =
        CSVLogger::new("comparisons.csv", Some(ScoreSet::headers()));

    for instance in instances {
        let problem_contents = read_to_string(instance).unwrap();
        let mut problem = CVRProblem::from(problem_contents);
        problem.precalculate_distances();

        let ea_scores = test_ea(&problem, 10);
        let ea_summary = Score::new(ea_scores);

        let greedy_scores = test_greedy(&problem, 200);
        let greedy_summary = Score::new(greedy_scores);

        let random_scores = test_random(&problem, 10000);
        let random_summary = Score::new(random_scores);
        logger.log(ScoreSet {
            instance: String::from(instance),
            ea: ea_summary,
            random: random_summary,
            greedy: greedy_summary,
        })
    }

    logger.flush().expect("Failed to write to file");
}