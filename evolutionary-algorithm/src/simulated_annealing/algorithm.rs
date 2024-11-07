use crate::{
    individual::{Fitness, VecIndividual},
    logger::{inverse_fitness, CSVEntry, PersistableLogger},
    neighbor::NeighborOperator,
    population::Population,
    problem::Problem,
    solver::Solver,
};
use anyhow::{Context, Result};
use rand::{thread_rng, Rng};

use super::{cooling_schedule::CoolingSchedule, criterion_operator::CriterionOperator};

pub type Temperature = f32;

pub struct SimulatedAnnealing {
    iterations: u32,
    criterion_operator: Box<dyn CriterionOperator>,
    cooling_schedule: Box<dyn CoolingSchedule>,
    neighbor_operator: Box<dyn NeighborOperator>,
    logger: Box<dyn PersistableLogger<SimulatedAnnealingIterationInfo>>,
}

struct Solution {
    individual: VecIndividual,
    fitness: Fitness,
    best_fitness: Fitness,
}

impl Solver for SimulatedAnnealing {
    fn solve(&mut self, problem: &dyn Problem) -> Result<(Fitness, VecIndividual)> {
        let mut solution = self.get_random_solution(problem)?;

        for i in 0..self.iterations {
            self.log(problem, &solution, i);
            solution = self.solution_iteration(solution, problem)?;
            self.decrease_temperature();
        }

        self.logger.flush()?;

        Ok((solution.fitness, solution.individual))
    }
}

impl SimulatedAnnealing {
    fn log(&mut self, problem: &dyn Problem, solution: &Solution, iteration: u32) {
        self.logger.log(SimulatedAnnealingIterationInfo::new(
            self.configuration_name(),
            iteration,
            self.cooling_schedule.temperature(),
            solution.individual.clone(),
            problem,
            solution.best_fitness,
        ));
    }

    fn configuration_name(&self) -> String {
        format!(
            "neighbor_operator: {}, cooling_schedule {}",
            self.neighbor_operator.name(),
            self.cooling_schedule.name(),
        )
    }
    fn solution_iteration(
        &mut self,
        solution: Solution,
        problem: &dyn Problem,
    ) -> Result<Solution> {
        let new_solution = self.get_neighbor(&solution, problem)?;

        if new_solution.fitness > solution.fitness {
            return Ok(Solution {
                fitness: new_solution.fitness,
                best_fitness: solution.best_fitness.max(new_solution.fitness),
                individual: new_solution.individual,
            });
        }

        if self.secondary_acceptance_criteria(solution.fitness - new_solution.fitness) {
            return Ok(Solution {
                fitness: new_solution.fitness,
                best_fitness: solution.best_fitness.max(new_solution.fitness),
                individual: new_solution.individual,
            });
        }

        Ok(solution)
    }

    fn get_neighbor(&self, solution: &Solution, problem: &dyn Problem) -> Result<Solution> {
        let neighbors = self
            .neighbor_operator
            .get_neighborhood(&solution.individual);

        let population = Population::new(neighbors);

        let (individual, fitness) = population.highest_fitness(problem);

        Ok(Solution {
            individual: individual.clone(),
            fitness,
            best_fitness: fitness,
        })
    }

    fn secondary_acceptance_criteria(&self, fitness_diff: Fitness) -> bool {
        self.criterion_operator
            .criterion(fitness_diff, self.cooling_schedule.temperature())
    }

    fn decrease_temperature(&mut self) {
        self.cooling_schedule.cooldown()
    }

    fn get_random_solution(&self, problem: &dyn Problem) -> Result<Solution> {
        let individual = problem.random_individual();
        let fitness = problem.eval(&individual)?;
        Ok(Solution {
            individual,
            fitness,
            best_fitness: fitness,
        })
    }
}

#[derive(Default)]
pub struct SimulatedAnnealingBuilder {
    iterations: Option<u32>,
    neighbor_operator: Option<Box<dyn NeighborOperator>>,
    cooling_schedule: Option<Box<dyn CoolingSchedule>>,
    criterion_operator: Option<Box<dyn CriterionOperator>>,
    logger: Option<Box<dyn PersistableLogger<SimulatedAnnealingIterationInfo>>>,
}

impl SimulatedAnnealingBuilder {
    pub fn new() -> Self {
        SimulatedAnnealingBuilder::default()
    }

    pub fn iterations(mut self, iterations: u32) -> Self {
        self.iterations = Some(iterations);
        self
    }

    pub fn cooling_schedule(mut self, cooling_schedule: Box<dyn CoolingSchedule>) -> Self {
        self.cooling_schedule = Some(cooling_schedule);
        self
    }

    pub fn criterion_operator(mut self, criterion_operator: Box<dyn CriterionOperator>) -> Self {
        self.criterion_operator = Some(criterion_operator);
        self
    }

    pub fn neighbor_operator(mut self, neighbor_operator: Box<dyn NeighborOperator>) -> Self {
        self.neighbor_operator = Some(neighbor_operator);
        self
    }

    pub fn logger(
        mut self,
        logger: Box<dyn PersistableLogger<SimulatedAnnealingIterationInfo>>,
    ) -> Self {
        self.logger = Some(logger);
        self
    }

    pub fn build(self) -> Result<SimulatedAnnealing> {
        Ok(SimulatedAnnealing {
            iterations: self.iterations.context("No iterations")?,
            cooling_schedule: self.cooling_schedule.context("No cooling schedule")?,
            criterion_operator: self.criterion_operator.context("No criterion operator")?,
            logger: self.logger.context("Missing logger")?,
            neighbor_operator: self
                .neighbor_operator
                .context("Missing neighbor operator")?,
        })
    }
}

#[derive(Default)]
pub struct SimulatedAnnealingIterationInfo {
    configuration: String,
    iteration: u32,
    temperature: Temperature,
    best_fitness: Fitness,
    current_fitness: Fitness,
}

impl SimulatedAnnealingIterationInfo {
    pub fn new(
        configuration: String,
        iteration: u32,
        temperature: Temperature,
        individual: VecIndividual,
        problem: &dyn Problem,
        best_fitness: Fitness,
    ) -> Self {
        let fitness = problem
            .eval(&individual)
            .expect("Failed to calculate fitness");
        SimulatedAnnealingIterationInfo {
            configuration,
            iteration,
            temperature,
            best_fitness,
            current_fitness: fitness,
        }
    }
}

impl From<&SimulatedAnnealingIterationInfo> for CSVEntry {
    fn from(val: &SimulatedAnnealingIterationInfo) -> Self {
        CSVEntry::from(vec![
            val.configuration.to_string(),
            val.iteration.to_string(),
            val.temperature.to_string(),
            inverse_fitness(val.best_fitness).to_string(),
            inverse_fitness(val.current_fitness).to_string(),
        ])
    }
}
