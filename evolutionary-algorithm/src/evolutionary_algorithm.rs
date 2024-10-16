use anyhow::{anyhow, Result};
use rand::Rng;

use crate::{
    collector::{CSVEntry, CSVLogger, PersistableLogger},
    crossover::{OrderedCrossover, SingleChildCrossoverOperator},
    individual::{Fitness, VecIndividual},
    mutation::{Mutation, SwapMutation},
    population::Population,
    problem::Problem,
    selection::Selector,
    solver::Solver,
};

struct GenerationInfo {
    worst_fitness: Fitness,
    average_fitness: Fitness,
    best_fitness: Fitness,
    mutations: usize,
    crossovers: usize,
    population_size: usize,
}

impl GenerationInfo {
    pub fn new(
        crossovers: usize,
        mutations: usize,
        population: &Population,
        problem: &dyn Problem,
    ) -> Self {
        let (_, best_fitness) = population.highest_fitness(problem);
        let (_, worst_fitness) = population.lowest_fitness(problem);
        let average_fitness = population.average_fitness(problem);
        GenerationInfo {
            crossovers,
            mutations,
            population_size: population.number_of_solutions(),
            best_fitness,
            average_fitness,
            worst_fitness,
        }
    }
}

impl From<&GenerationInfo> for CSVEntry {
    fn from(val: &GenerationInfo) -> Self {
        CSVEntry::from(vec![
            val.best_fitness.to_string(),
            val.worst_fitness.to_string(),
            val.average_fitness.to_string(),
            val.mutations.to_string(),
            val.crossovers.to_string(),
            val.population_size.to_string(),
        ])
    }
}

pub struct EvolutionaryAlgorithm<'a> {
    generations: u16,
    population_size: u16,
    best_solution: Option<VecIndividual>,
    best_solution_quality: Option<Fitness>,
    selection_operator: &'a dyn Selector,
    crossover_prob: f32,
    mutation_prob: f32,
    logger: Box<dyn PersistableLogger<GenerationInfo>>,
}

impl<'a> EvolutionaryAlgorithm<'a> {
    fn create_generation(&self, problem: &dyn Problem) -> Population {
        let initial_solutions: Vec<VecIndividual> = (0..self.population_size)
            .map(|_| problem.random_individual())
            .collect();

        Population::new(initial_solutions)
    }

    pub fn new(
        generations: u16,
        population_size: u16,
        selection_operator: &'a dyn Selector,
    ) -> Self {
        let logger: CSVLogger<GenerationInfo> = CSVLogger::new(vec![
            "best_fitness".to_string(),
            "worst_fitness".to_string(),
            "average_fitness".to_string(),
            "mutations".to_string(),
            "crossovers".to_string(),
            "population_size".to_string(),
        ]);
        Self {
            generations,
            population_size,
            best_solution: None,
            best_solution_quality: None,
            selection_operator,
            crossover_prob: 0.4,
            mutation_prob: 0.5,
            logger: Box::new(logger),
        }
    }

    fn crossover(&self, mut population: Population) -> (Population, usize) {
        let mut rng = rand::thread_rng();
        let ox = OrderedCrossover {};
        let mut children_population = Population::default();
        let mut crossover_count = 0;
        for _ in 0..population.number_of_solutions() {
            let are_crossed = rng.gen_range(0.0..1.0);
            if are_crossed < self.crossover_prob {
                let parent_a = population.random_individual();
                let parent_b = population.random_individual();

                let child = ox.crossover(parent_a, parent_b);
                children_population.add_individual(child);
                crossover_count += 1;
            }
        }

        population.replace_subpopulation(children_population);
        (population, crossover_count)
    }

    fn mutate(&self, population: &mut Population) -> usize {
        let sp = SwapMutation {};
        let mut rng = rand::thread_rng();
        let mut mutation_count = 0;

        for individual in population.mut_solutions().iter_mut() {
            let is_mutated = rng.gen_range(0.0..1.0);
            if is_mutated < self.mutation_prob {
                sp.mutate(individual);
                mutation_count += 1;
            }
        }
        mutation_count
    }
}

impl<'a> Solver for EvolutionaryAlgorithm<'a> {
    fn solve(&mut self, problem: &dyn Problem) -> Result<VecIndividual> {
        let mut population = self.create_generation(problem);
        for _ in 0..self.generations {
            // Generates initial population
            // let best_in_generation = population.highest_fitness(problem).into();
            // let best_in_generation_quality = problem.eval(&best_in_generation)?;
            // match self.best_solution_quality {
            //     None => {
            //         self.best_solution_quality = Some(best_in_generation_quality);
            //         self.best_solution = Some(best_in_generation);
            //     }
            //     Some(quality) if (best_in_generation_quality > quality) => {
            //         self.best_solution_quality = Some(best_in_generation_quality);
            //         self.best_solution = Some(best_in_generation);
            //     }
            //     _ => {}
            // }

            population = self.selection_operator.select(population);
            let (mut new_population, crossovers) = self.crossover(population);
            let mutations = self.mutate(&mut new_population);
            population = new_population;

            self.logger.log(GenerationInfo::new(
                crossovers,
                mutations,
                &population,
                problem,
            ));
        }

        self.logger.flush()?;

        match &self.best_solution {
            None => Err(anyhow!("No solution was found")),
            Some(solution) => Ok(solution.into()),
        }
    }
}
