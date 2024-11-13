use crate::{logger::PersistableLogger, neighbor::NeighborOperator, solver::Solver};

use super::{
    algorithm::{SimulatedAnnealing, SimulatedAnnealingBuilder, SimulatedAnnealingIterationInfo},
    cooling_schedule::CoolingSchedule,
    criterion_operator::CriterionOperator,
};

// pub struct SimulatedAnnealing {
//     iterations: u32,
//     criterion_operator: Box<dyn CriterionOperator>,
//     cooling_schedule: Box<dyn CoolingSchedule>,
//     neighbor_operator: Box<dyn NeighborOperator>,
//     logger: Box<dyn PersistableLogger<SimulatedAnnealingIterationInfo>>,
// }

struct SimulatedAnnlealingOptimizerParams {
    min_iterations: usize,
    max_iterations: usize,
    iteration_step: usize,
    criterion_operators: Vec<Box<dyn CriterionOperator>>,
    cooling_schedules: Vec<Box<dyn CoolingSchedule>>,
    neighbor_operators: Vec<Box<dyn NeighborOperator>>,
    logger: Box<dyn PersistableLogger<SimulatedAnnealingIterationInfo>>,
}

trait Optimizer<T: Solver> {
    fn find_best_params(&self) -> T;
}

struct SimulatedAnnealingOptimizer {
    params: SimulatedAnnlealingOptimizerParams,
}

impl Optimizer<SimulatedAnnealing> for SimulatedAnnealingOptimizer {
    fn find_best_params(&self) -> SimulatedAnnealing {
        // todo!()
        for iterations in (self.params.min_iterations..self.params.max_iterations)
            .step_by(self.params.iteration_step)
        {
            for criterion in &self.params.criterion_operators {
                SimulatedAnnealingBuilder::default()
                    .iterations(iterations as u32)
                    .criterion_operator(criterion)
                    .build();
            }
        }

        SimulatedAnnealingBuilder::default()
            .iterations(self.params.min_iterations as u32)
            .build()
            .unwrap()
    }
}

// pub fn generate_sa_params(params: SimulatedAnnlealingOptimizerParams) {}
