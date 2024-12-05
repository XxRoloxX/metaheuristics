use std::fs::read_to_string;

use crate::{
    evolutionary_algorithm::{
        algorithm::{EvolutionaryAlgorithm, EvolutionaryAlgorithmBuilder},
        crossover::{CrossoverOperator, OrderedCrossover},
        mutation::SwapMutation,
        selection::{SimulatedAnnealingRouletteSelector, TournamentSelector},
    },
    greedy_algorithm::GreedyAlgorithm,
    individual::Fitness,
    logger::{inverse_fitness, CSVEntry, CSVLogger, PersistableLogger},
    neighbor::SwapNeighborhoodOperator,
    problem::Problem,
    problem_loader::CVRProblem,
    simulated_annealing::{
        algorithm::{SimulatedAnnealing, SimulatedAnnealingBuilder},
        cooling_schedule::ExponentialCoolingScheduleBuilder,
        criterion_operator::BoltzmanProbabilityCriterionOperator,
    },
    solver::Solver,
    tabu_search::algorithm::{TabuSearch, TabuSearchBuilder},
    tssa::algorithm::{TSSABuilder, TSSA},
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
    tabu: Score,
    random: Score,
    greedy: Score,
    sa: Score,
    tssa: Score,
    saea: Score,
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
            String::from("Tabu: best"),
            String::from("Tabu: worst"),
            String::from("Tabu: avg"),
            String::from("Tabu: std"),
            String::from("SA: best"),
            String::from("SA: worst"),
            String::from("SA: avg"),
            String::from("SA: std"),
            String::from("TSSA: best"),
            String::from("TSSA: worst"),
            String::from("TSSA: avg"),
            String::from("TSSA: std"),
            String::from("SAEA: best"),
            String::from("SAEA: worst"),
            String::from("SAEA: avg"),
            String::from("SAEA: std"),
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
            score_set.tabu.best.to_string(),
            score_set.tabu.worst.to_string(),
            score_set.tabu.avg.to_string(),
            score_set.tabu.std.to_string(),
            score_set.sa.best.to_string(),
            score_set.sa.worst.to_string(),
            score_set.sa.avg.to_string(),
            score_set.sa.std.to_string(),
            score_set.tssa.best.to_string(),
            score_set.tssa.worst.to_string(),
            score_set.tssa.avg.to_string(),
            score_set.tssa.std.to_string(),
            score_set.saea.best.to_string(),
            score_set.saea.worst.to_string(),
            score_set.saea.avg.to_string(),
            score_set.saea.std.to_string(),
        ])
    }
}

fn optimal_ea() -> EvolutionaryAlgorithm {
    EvolutionaryAlgorithmBuilder::new()
        .population_size(500)
        .generations(500)
        .crossover_prob(0.7)
        .mutation_prob(0.6)
        .logger(Box::new(CSVLogger::new("comparisons", None)))
        .crossover_operator(CrossoverOperator::SingleChildCrossoverOperator(Box::new(
            OrderedCrossover {},
        )))
        .mutation_operator(Box::new(SwapMutation {}))
        .selection_operator(Box::new(TournamentSelector::new(10)))
        .build()
        .unwrap()
}

fn optimal_tabu() -> TabuSearch {
    TabuSearchBuilder::default()
        .iterations(1000)
        .tabu_list_size(20)
        .neighborhood_operator(Box::new(SwapNeighborhoodOperator::new(40)))
        .logger(Box::new(CSVLogger::new(
            format!("{}-tabu", "comparisons").as_str(),
            None,
        )))
        .build()
        .unwrap()
}

fn optimal_sa() -> SimulatedAnnealing {
    SimulatedAnnealingBuilder::default()
        .iterations(10000)
        .cooling_schedule(Box::new(
            ExponentialCoolingScheduleBuilder::default()
                .initial_temperature(1f32)
                .cooling_factor(0.999f32)
                .build()
                .unwrap(),
        ))
        .neighbor_operator(Box::new(SwapNeighborhoodOperator::new(40)))
        .criterion_operator(Box::new(BoltzmanProbabilityCriterionOperator {}))
        .logger(Box::new(CSVLogger::new("sa-comparisions", None)))
        .build()
        .unwrap()
}
fn optimal_tssa() -> TSSA {
    TSSABuilder::default()
        .iterations(20)
        .cooling_schedule(Box::new(
            ExponentialCoolingScheduleBuilder::default()
                .initial_temperature(1f32)
                .cooling_factor(0.999f32)
                .build()
                .unwrap(),
        ))
        .algorithm_switch_interval(1000)
        .neighborhood_operator(Box::new(SwapNeighborhoodOperator::new(40)))
        .tabu_list_size(200)
        .criterion_operator(Box::new(BoltzmanProbabilityCriterionOperator {}))
        .logger(Box::new(CSVLogger::new("sa-comparisions", None)))
        .build()
        .unwrap()
}

fn optimal_saea() -> EvolutionaryAlgorithm {
    EvolutionaryAlgorithmBuilder::new()
        .population_size(200)
        .generations(1000)
        .crossover_prob(0.6)
        .mutation_prob(0.3)
        .logger(Box::new(CSVLogger::new("sa-comparisions", None)))
        .crossover_operator(CrossoverOperator::SingleChildCrossoverOperator(Box::new(
            OrderedCrossover {},
        )))
        .mutation_operator(Box::new(SwapMutation {}))
        .selection_operator(Box::new(SimulatedAnnealingRouletteSelector::new(1.0, 0.99)))
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

fn test_taboo(problem: &dyn Problem, repeats: u16) -> Vec<Fitness> {
    let mut ea = optimal_tabu();
    (0..repeats)
        .map(|_| {
            let (score, _) = ea.solve(problem).unwrap();
            score
        })
        .collect::<Vec<Fitness>>()
}
fn test_sa(problem: &dyn Problem, repeats: u16) -> Vec<Fitness> {
    let mut ea = optimal_sa();
    (0..repeats)
        .map(|_| {
            let (score, _) = ea.solve(problem).unwrap();
            score
        })
        .collect::<Vec<Fitness>>()
}

fn test_tssa(problem: &dyn Problem, repeats: u16) -> Vec<Fitness> {
    let mut ea = optimal_tssa();
    (0..repeats)
        .map(|_| {
            let (score, _) = ea.solve(problem).unwrap();
            score
        })
        .collect::<Vec<Fitness>>()
}

fn test_saea(problem: &dyn Problem, repeats: u16) -> Vec<Fitness> {
    let mut ea = optimal_saea();
    (0..repeats)
        .map(|_| {
            let (score, _) = ea.solve(problem).unwrap();
            score
        })
        .collect::<Vec<Fitness>>()
}

fn test_greedy(problem: &CVRProblem) -> Vec<Fitness> {
    let mut greedy = GreedyAlgorithm::new(problem);
    problem
        .random_individual()
        .genes()
        .iter()
        .map(|gene| {
            let solution = greedy.solve(*gene).unwrap();
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
    //let instances: Vec<&str> = vec![
    //    "./src/problem-instances/A-n32-k5.txt",
    //    "./src/problem-instances/A-n37-k5.txt",
    //    "./src/problem-instances/A-n39-k5.txt",
    //    "./src/problem-instances/A-n45-k7.txt",
    //    "./src/problem-instances/A-n48-k7.txt",
    //    "./src/problem-instances/A-n54-k7.txt",
    //    "./src/problem-instances/A-n60-k9.txt",
    //];
    //
    let instances: Vec<&str> = vec![
        "./csv/A-n32-k5",
        "./csv/A-n37-k6",
        "./csv/A-n39-k5",
        "./csv/A-n45-k6",
        "./csv/A-n48-k7",
        "./csv/A-n54-k7",
        "./csv/A-n60-k9",
    ];

    let mut logger: CSVLogger<ScoreSet> =
        CSVLogger::new("./csv/comparisons-sa.csv", Some(ScoreSet::headers()));

    for instance in instances {
        let problem_contents = read_to_string(instance).unwrap();
        let mut problem = CVRProblem::from(problem_contents);
        problem.precalculate_distances();

        let ea_scores = test_ea(&problem, 10);
        let ea_summary = Score::new(ea_scores);

        let greedy_scores = test_greedy(&problem);
        let greedy_summary = Score::new(greedy_scores);

        let random_scores = test_random(&problem, 10000);
        let random_summary = Score::new(random_scores);

        let tabu_scores = test_taboo(&problem, 10);
        let tabu_summary = Score::new(tabu_scores);

        let sa_scores = test_sa(&problem, 10);
        let sa_summary = Score::new(sa_scores);

        let tssa_scores = test_tssa(&problem, 10);
        let tssa_summary = Score::new(tssa_scores);

        let saea_scores = test_saea(&problem, 10);
        let saea_summary = Score::new(saea_scores);

        logger.log(ScoreSet {
            instance: String::from(instance),
            ea: ea_summary,
            random: random_summary,
            greedy: greedy_summary,
            tabu: tabu_summary,
            sa: sa_summary,
            tssa: tssa_summary,
            saea: saea_summary,
        })
    }

    logger.flush().expect("Failed to write to file");
}
