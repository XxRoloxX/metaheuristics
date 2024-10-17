use anyhow::{Context, Result};
use rand::Rng;

use crate::{
    collector::{inverse_fitness, CSVEntry, PersistableLogger},
    crossover::{CrossoverOperator, SingleChildCrossoverOperator},
    individual::{Fitness, VecIndividual},
    mutation::Mutation,
    population::Population,
    problem::Problem,
    selection::Selector,
    solver::Solver,
};

pub struct GenerationInfo {
    configuration_name: String,
    index: usize,
    worst_fitness: Fitness,
    average_fitness: Fitness,
    best_fitness: Fitness,
    mutations: usize,
    crossovers: usize,
    population_size: usize,
}

impl GenerationInfo {
    pub fn new(
        configuration_name: String,
        index: usize,
        crossovers: usize,
        mutations: usize,
        population: &Population,
        problem: &dyn Problem,
    ) -> Self {
        let (_, best_fitness) = population.highest_fitness(problem);
        let (_, worst_fitness) = population.lowest_fitness(problem);
        let average_fitness = population.average_fitness(problem);
        GenerationInfo {
            configuration_name,
            index,
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
            val.configuration_name.to_string(),
            val.index.to_string(),
            inverse_fitness(val.best_fitness).to_string(),
            inverse_fitness(val.average_fitness).to_string(),
            inverse_fitness(val.worst_fitness).to_string(),
            val.mutations.to_string(),
            val.crossovers.to_string(),
            val.population_size.to_string(),
        ])
    }
}

#[derive(Default)]
pub struct EvolutionaryAlgorithmBuilder {
    generations: Option<u16>,
    population_size: Option<u16>,
    selection_operator: Option<Box<dyn Selector>>,
    mutation_operator: Option<Box<dyn Mutation>>,
    crossover_operator: Option<CrossoverOperator>,
    crossover_prob: Option<f32>,
    mutation_prob: Option<f32>,
    logger: Option<Box<dyn PersistableLogger<GenerationInfo>>>,
}

impl EvolutionaryAlgorithmBuilder {
    pub fn new() -> Self {
        EvolutionaryAlgorithmBuilder::default()
    }

    pub fn generations(mut self, generations: u16) -> Self {
        self.generations = Some(generations);
        self
    }

    pub fn population_size(mut self, population_size: u16) -> Self {
        self.population_size = Some(population_size);
        self
    }

    pub fn selection_operator(mut self, selection_operator: Box<dyn Selector>) -> Self {
        self.selection_operator = Some(selection_operator);
        self
    }

    pub fn mutation_operator(mut self, mutation_operator: Box<dyn Mutation>) -> Self {
        self.mutation_operator = Some(mutation_operator);
        self
    }

    pub fn crossover_operator(mut self, crossover_operator: CrossoverOperator) -> Self {
        self.crossover_operator = Some(crossover_operator);
        self
    }

    pub fn crossover_prob(mut self, crossover_prob: f32) -> Self {
        self.crossover_prob = Some(crossover_prob);
        self
    }

    pub fn mutation_prob(mut self, mutation_prob: f32) -> Self {
        self.mutation_prob = Some(mutation_prob);
        self
    }

    pub fn logger(mut self, logger: Box<dyn PersistableLogger<GenerationInfo>>) -> Self {
        self.logger = Some(logger);
        self
    }

    pub fn build(self) -> Result<EvolutionaryAlgorithm> {
        Ok(EvolutionaryAlgorithm {
            generations: self.generations.context("Missing number of generations")?,
            population_size: self
                .population_size
                .context("Missing number of generations")?,
            selection_operator: self
                .selection_operator
                .context("Missing selection operator")?,
            mutation_operator: self
                .mutation_operator
                .context("Missing mutation operator")?,
            crossover_operator: self
                .crossover_operator
                .context("Missing crossover operator")?,
            mutation_prob: self.mutation_prob.context("Missing mutation probability")?,
            crossover_prob: self
                .crossover_prob
                .context("Missing crossover probability")?,
            logger: self.logger.context("Missing logger")?,
        })
    }
}

pub struct EvolutionaryAlgorithm {
    generations: u16,
    population_size: u16,
    selection_operator: Box<dyn Selector>,
    mutation_operator: Box<dyn Mutation>,
    crossover_operator: CrossoverOperator,
    crossover_prob: f32,
    mutation_prob: f32,
    logger: Box<dyn PersistableLogger<GenerationInfo>>,
}

impl EvolutionaryAlgorithm {
    fn configuration_name(&self) -> String {
        format!(
            "population: {}, generations: {}, selection: {}, mutation: {}, crossover: {}",
            self.population_size,
            self.generations,
            self.selection_operator.name(),
            self.mutation_operator.name(),
            self.crossover_operator.name(),
        )
    }
    fn create_generation(&self, problem: &dyn Problem) -> Population {
        let initial_solutions: Vec<VecIndividual> = (0..self.population_size)
            .map(|_| problem.random_individual())
            .collect();

        Population::new(initial_solutions)
    }

    // pub fn new(
    //     generations: u16,
    //     population_size: u16,
    //     selection_operator: Box<dyn Selector>,
    //     crossover_operator: CrossoverOperator,
    //     mutation_operator: Box<dyn Mutation>,
    //     crossover_prob: f32,
    //     mutation_prob: f32,
    //     logger: Box<dyn PersistableLogger<GenerationInfo>>,
    // ) -> Self {
    //     Self {
    //         generations,
    //         population_size,
    //         selection_operator,
    //         crossover_prob,
    //         mutation_prob,
    //         mutation_operator,
    //         crossover_operator,
    //         logger,
    //     }
    // }

    fn crossover(&self, mut population: Population) -> Result<(Population, usize)> {
        let mut rng = rand::thread_rng();
        let mut children_population = Population::default();
        let mut crossover_count = 0;
        for _ in 0..population.number_of_solutions() / 2 {
            let are_crossed = rng.gen_range(0.0..1.0);
            if are_crossed < self.crossover_prob {
                let parent_a = population.random_individual();
                let parent_b = population.random_individual();

                match &self.crossover_operator {
                    CrossoverOperator::SingleChildCrossoverOperator(operator) => {
                        let mut child = operator.crossover(parent_a, parent_b);
                        children_population.add_individual(child);
                        child = operator.crossover(parent_a, parent_b);
                        children_population.add_individual(child);
                    }
                    CrossoverOperator::TwoChildrenCrossoverOperator(operator) => {
                        let (child_a, child_b) = operator.crossover(parent_a, parent_b)?;
                        children_population.add_individual(child_a);
                        children_population.add_individual(child_b);
                    }
                };

                crossover_count += 1;
            }
        }

        // TODO: Handle odd population number
        population.replace_subpopulation(children_population);
        Ok((population, crossover_count))
    }

    fn mutate(&self, population: &mut Population) -> usize {
        let sp = &self.mutation_operator;
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

impl Solver for EvolutionaryAlgorithm {
    fn solve(&mut self, problem: &dyn Problem) -> Result<(Fitness, VecIndividual)> {
        let mut population = self.create_generation(problem);
        let mut best_solution: Option<VecIndividual> = None;
        let mut best_solution_fitness: Option<Fitness> = None;
        for idx in 0..self.generations {
            let (current_best, current_best_fitness) = population.highest_fitness(problem);

            match (&best_solution, best_solution_fitness) {
                (None, None) => {
                    best_solution = Some(current_best.into());
                    best_solution_fitness = Some(current_best_fitness);
                }
                (Some(_), Some(fitness)) if current_best_fitness > fitness => {
                    best_solution = Some(current_best.into());
                    best_solution_fitness = Some(current_best_fitness);
                }
                _ => {}
            }

            population = self.selection_operator.select(population, problem)?;
            let (mut new_population, crossovers) = self.crossover(population)?;
            let mutations = self.mutate(&mut new_population);
            population = new_population;

            self.logger.log(GenerationInfo::new(
                self.configuration_name(),
                idx as usize,
                crossovers,
                mutations,
                &population,
                problem,
            ));
        }

        self.logger.flush()?;
        Ok((
            best_solution_fitness.context("No solution found!")?,
            best_solution.context("No solution found!")?,
        ))
    }
}
