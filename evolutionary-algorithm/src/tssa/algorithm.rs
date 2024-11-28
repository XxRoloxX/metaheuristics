use anyhow::{Context, Result};

use crate::{
    individual::{Fitness, VecIndividual},
    logger::{inverse_fitness, CSVEntry, PersistableLogger},
    population::Population,
    problem::Problem,
    simulated_annealing::{
        algorithm::Solution, cooling_schedule::CoolingSchedule,
        criterion_operator::CriterionOperator,
    },
    solver::Solver,
};

use crate::neighbor::NeighborOperator;

pub struct TSSA {
    iterations: u32,
    algorithm_switch_interval: u32,
    tabu_list_size: usize,
    logger: Box<dyn PersistableLogger<IterationInfo>>,
    neighborhood_operator: Box<dyn NeighborOperator>,
    criterion_operator: Box<dyn CriterionOperator>,
    cooling_schedule: Box<dyn CoolingSchedule>,
}

impl TSSA {
    fn configuration_name(&self) -> String {
        format!(
            "tabu_size: {}, neighborhood_operator: {}, criterion: {}, cooling_schedule: {}, switch_interval: {}",
            self.tabu_list_size,
            self.neighborhood_operator.name(),
            self.criterion_operator.name(),
            self.cooling_schedule.name(),
            self.algorithm_switch_interval
        )
    }
}

#[derive(Default)]
pub struct IterationInfo {
    configuration: String,
    iteration: u32,
    best_fitness: Fitness,
    worst_fitness: Fitness,
    average_fitness: Fitness,
    current_fitness: Fitness,
}

impl IterationInfo {
    pub fn new(
        configuration: String,
        iteration: u32,
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
            inverse_fitness(val.best_fitness).to_string(),
            inverse_fitness(val.average_fitness).to_string(),
            inverse_fitness(val.worst_fitness).to_string(),
            inverse_fitness(val.current_fitness).to_string(),
        ])
    }
}

enum CurrentAlgorithm {
    SA,
    TS,
}

impl Solver for TSSA {
    fn solve(&mut self, problem: &dyn Problem) -> Result<(Fitness, VecIndividual)> {
        let mut best_solution = problem.random_individual();
        let mut best_fitness: Fitness = -f32::INFINITY;
        let mut current_alg: CurrentAlgorithm = CurrentAlgorithm::TS;

        for iteration in 0..self.iterations {
            match current_alg {
                CurrentAlgorithm::SA => {
                    let new_best = self
                        .solve_with_tabu(
                            problem,
                            iteration * self.algorithm_switch_interval,
                            best_solution.clone(),
                            best_fitness,
                        )
                        .unwrap();
                    current_alg = CurrentAlgorithm::TS;

                    best_solution = new_best.1;
                    best_fitness = new_best.0;
                }
                CurrentAlgorithm::TS => {
                    let new_best = self
                        .solve_with_sa(
                            problem,
                            iteration * self.algorithm_switch_interval,
                            best_solution.clone(),
                            best_fitness,
                        )
                        .unwrap();

                    current_alg = CurrentAlgorithm::SA;
                    best_solution = new_best.1;
                    best_fitness = new_best.0;
                }
            }
        }

        self.logger.flush()?;

        Ok((best_fitness, best_solution))
    }
}

impl TSSA {
    fn solve_with_tabu(
        &mut self,
        problem: &dyn Problem,
        initial_iteration: u32,
        initial_solution: VecIndividual,
        initial_fitness: Fitness,
    ) -> Result<(Fitness, VecIndividual)> {
        let mut best_solution = initial_solution;
        let mut tabu_list: Vec<VecIndividual> = Vec::new();
        let mut best_fitness: Fitness = initial_fitness;

        for iteration in initial_iteration..(self.algorithm_switch_interval + initial_iteration) {
            let neighbors = self.neighborhood_operator.get_neighborhood(&best_solution);
            let population = Population::new(
                neighbors
                    .into_iter()
                    .filter(|neighbor| !tabu_list.contains(neighbor))
                    .collect(),
            );
            if population.number_of_solutions() == 0 {
                continue;
            }
            println!("Iteration of TS {}", iteration);

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
                population,
                problem,
                best_fitness,
            ));
        }

        self.logger.flush()?;

        Ok((best_fitness, best_solution))
    }

    fn solve_with_sa(
        &mut self,
        problem: &dyn Problem,
        initial_iteration: u32,
        initial_solution: VecIndividual,
        initial_fitness: Fitness,
    ) -> Result<(Fitness, VecIndividual)> {
        let mut solution = Solution {
            fitness: initial_fitness,
            individual: initial_solution,
            best_fitness: initial_fitness,
        };

        for i in initial_iteration..(self.algorithm_switch_interval + initial_iteration) {
            self.log(problem, &solution, i);
            solution = self.solution_iteration(solution, problem)?;
            self.decrease_temperature();
        }

        self.logger.flush()?;

        Ok((solution.fitness, solution.individual))
    }

    //fn configuration_name(&self) -> String {
    //    format!(
    //        "neighbor_operator: {}, cooling_schedule {}",
    //        self.neighbor_operator.name(),
    //        self.cooling_schedule.name(),
    //    )
    //}
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
            .neighborhood_operator
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

    //fn get_random_solution(&self, problem: &dyn Problem) -> Result<Solution> {
    //    let individual = problem.random_individual();
    //    let fitness = problem.eval(&individual)?;
    //    Ok(Solution {
    //        individual,
    //        fitness,
    //        best_fitness: fitness,
    //    })
    //}

    fn log(&mut self, problem: &dyn Problem, solution: &Solution, iteration: u32) {
        self.logger.log(IterationInfo::new(
            self.configuration_name(),
            iteration,
            Population::new(vec![solution.individual.clone()]),
            problem,
            solution.best_fitness,
        ));
    }
}

#[derive(Default)]
pub struct TSSABuilder {
    iterations: Option<u32>,
    tabu_list_size: Option<usize>,
    algorithm_switch_interval: Option<u32>,
    cooling_schedule: Option<Box<dyn CoolingSchedule>>,
    criterion_operator: Option<Box<dyn CriterionOperator>>,
    logger: Option<Box<dyn PersistableLogger<IterationInfo>>>,
    neighborhood_operator: Option<Box<dyn NeighborOperator>>,
}

impl TSSABuilder {
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

    pub fn cooling_schedule(mut self, cooling_schedule: Box<dyn CoolingSchedule>) -> Self {
        self.cooling_schedule = Some(cooling_schedule);
        self
    }

    pub fn algorithm_switch_interval(mut self, algorithm_switch_interval: u32) -> Self {
        self.algorithm_switch_interval = Some(algorithm_switch_interval);
        self
    }

    pub fn criterion_operator(mut self, criterion_operator: Box<dyn CriterionOperator>) -> Self {
        self.criterion_operator = Some(criterion_operator);
        self
    }

    pub fn build(self) -> Result<TSSA> {
        Ok(TSSA {
            iterations: self.iterations.context("Missing iterations parameters")?,
            tabu_list_size: self
                .tabu_list_size
                .context("Missing tabu list size parameter")?,
            logger: self.logger.context("No logger parameter")?,
            algorithm_switch_interval: self
                .algorithm_switch_interval
                .context("No algorithm switch interval")?,
            cooling_schedule: self.cooling_schedule.context("No cooling schedule")?,
            criterion_operator: self.criterion_operator.context("No criterion operator")?,
            neighborhood_operator: self
                .neighborhood_operator
                .context("No neighborhood operator")?,
        })
    }
}
