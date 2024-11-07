use anyhow::{Context, Result};

use crate::{
    individual::{Fitness, VecIndividual},
    logger::{inverse_fitness, CSVEntry, PersistableLogger},
    population::Population,
    problem::Problem,
    solver::Solver,
};

use crate::neighbor::NeighborOperator;

pub struct TabuSearch {
    iterations: u32,
    tabu_list_size: usize,
    logger: Box<dyn PersistableLogger<IterationInfo>>,
    neighborhood_operator: Box<dyn NeighborOperator>,
}

impl TabuSearch {
    fn configuration_name(&self) -> String {
        format!(
            "iterations: {}, tabu_size: {}, neighborhood_operator: {}",
            self.iterations,
            self.tabu_list_size,
            self.neighborhood_operator.name()
        )
    }
}

#[derive(Default)]
pub struct IterationInfo {
    configuration: String,
    iteration: u32,
    tabu_list_size: usize,
    best_fitness: Fitness,
    worst_fitness: Fitness,
    average_fitness: Fitness,
    current_fitness: Fitness,
}

impl IterationInfo {
    pub fn new(
        configuration: String,
        iteration: u32,
        tabu_list_size: usize,
        neighbours: Population,
        problem: &dyn Problem,
        best_fitness: Fitness,
    ) -> Self {
        let (_, current_fitness) = neighbours.highest_fitness(problem);
        let (_, worst_fitness) = neighbours.lowest_fitness(problem);
        let average_fitness = neighbours.average_fitness(problem);
        IterationInfo {
            configuration,
            iteration,
            tabu_list_size,
            best_fitness,
            worst_fitness,
            average_fitness,
            current_fitness,
        }
    }
}

impl From<&IterationInfo> for CSVEntry {
    fn from(val: &IterationInfo) -> Self {
        CSVEntry::from(vec![
            val.configuration.to_string(),
            val.iteration.to_string(),
            val.tabu_list_size.to_string(),
            inverse_fitness(val.best_fitness).to_string(),
            inverse_fitness(val.average_fitness).to_string(),
            inverse_fitness(val.worst_fitness).to_string(),
            inverse_fitness(val.current_fitness).to_string(),
        ])
    }
}

impl Solver for TabuSearch {
    fn solve(&mut self, problem: &dyn Problem) -> Result<(Fitness, VecIndividual)> {
        let mut best_solution = problem.random_individual();
        let mut tabu_list: Vec<VecIndividual> = Vec::new();
        let mut best_fitness: Fitness = -f32::INFINITY;

        for iteration in 0..self.iterations {
            let neighbors = self.neighborhood_operator.get_neighborhood(&best_solution);
            let population = Population::new(
                neighbors
                    .into_iter()
                    .filter(|neighbor| !tabu_list.contains(neighbor))
                    .collect(),
            );

            let (current_solution, current_solution_fitness) = population.highest_fitness(problem);
            if current_solution_fitness > best_fitness {
                best_fitness = current_solution_fitness;
                best_solution = current_solution.clone();
            }

            tabu_list.push(current_solution.clone());
            if tabu_list.len() > self.tabu_list_size {
                tabu_list.remove(0);
            }

            self.logger.log(IterationInfo::new(
                self.configuration_name(),
                iteration,
                tabu_list.len(),
                population,
                problem,
                best_fitness,
            ));
        }

        self.logger.flush()?;

        Ok((best_fitness, best_solution))
    }
}

#[derive(Default)]
pub struct TabuSearchBuilder {
    iterations: Option<u32>,
    tabu_list_size: Option<usize>,
    logger: Option<Box<dyn PersistableLogger<IterationInfo>>>,
    neighborhood_operator: Option<Box<dyn NeighborOperator>>,
}

impl TabuSearchBuilder {
    pub fn iterations(mut self, iterations: u32) -> Self {
        self.iterations = Some(iterations);
        self
    }

    pub fn tabu_list_size(mut self, tabu_list_size: usize) -> Self {
        self.tabu_list_size = Some(tabu_list_size);
        self
    }

    pub fn logger(mut self, logger: Box<dyn PersistableLogger<IterationInfo>>) -> Self {
        self.logger = Some(logger);
        self
    }

    pub fn neighborhood_operator(mut self, neighborhood: Box<dyn NeighborOperator>) -> Self {
        self.neighborhood_operator = Some(neighborhood);
        self
    }
    pub fn build(self) -> Result<TabuSearch> {
        Ok(TabuSearch {
            iterations: self.iterations.context("Missing iterations parameters")?,
            tabu_list_size: self
                .tabu_list_size
                .context("Missing tabu list size parameter")?,
            logger: self.logger.context("No logger parameter")?,
            neighborhood_operator: self
                .neighborhood_operator
                .context("No neighborhood operator")?,
        })
    }
}
