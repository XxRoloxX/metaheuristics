use anyhow::Result;

use crate::{
    logger::CSVLogger,
    neighbor::{InverseNeighborhoodOperator, SwapNeighborhoodOperator},
    solver::Solver,
};

use super::{
    algorithm::SimulatedAnnealingBuilder,
    cooling_schedule::{
        ExponentialCoolingSchedule, ExponentialCoolingScheduleBuilder, LinearCoolingSchedule,
    },
    criterion_operator::BoltzmanProbabilityCriterionOperator,
};

pub fn get_simulated_annealing_config(instance: &str) -> Result<Vec<Box<dyn Solver>>> {
    let iteration_info_headers = vec![
        "configuration".to_string(),
        "iteration".to_string(),
        "temperature".to_string(),
        "best_fitness".to_string(),
        "current_fitness".to_string(),
    ];
    Ok(vec![
        Box::new(
            SimulatedAnnealingBuilder::default()
                .iterations(10000)
                .cooling_schedule(Box::new(
                    ExponentialCoolingScheduleBuilder::default()
                        .initial_temperature(1f32)
                        .cooling_factor(0.99f32)
                        .build()?,
                ))
                .neighbor_operator(Box::new(SwapNeighborhoodOperator::new(10)))
                .criterion_operator(Box::new(BoltzmanProbabilityCriterionOperator {}))
                .logger(Box::new(CSVLogger::new(
                    instance,
                    Some(iteration_info_headers),
                )))
                .build()?,
        ),
        Box::new(
            SimulatedAnnealingBuilder::default()
                .iterations(10000)
                .cooling_schedule(Box::new(
                    ExponentialCoolingScheduleBuilder::default()
                        .initial_temperature(0.5f32)
                        .cooling_factor(0.99f32)
                        .build()?,
                ))
                .neighbor_operator(Box::new(SwapNeighborhoodOperator::new(10)))
                .criterion_operator(Box::new(BoltzmanProbabilityCriterionOperator {}))
                .logger(Box::new(CSVLogger::new(instance, None)))
                .build()?,
        ),
        Box::new(
            SimulatedAnnealingBuilder::default()
                .iterations(10000)
                .cooling_schedule(Box::new(
                    ExponentialCoolingScheduleBuilder::default()
                        .initial_temperature(0.1f32)
                        .cooling_factor(0.99f32)
                        .build()?,
                ))
                .neighbor_operator(Box::new(SwapNeighborhoodOperator::new(10)))
                .criterion_operator(Box::new(BoltzmanProbabilityCriterionOperator {}))
                .logger(Box::new(CSVLogger::new(instance, None)))
                .build()?,
        ),
        Box::new(
            SimulatedAnnealingBuilder::default()
                .iterations(10000)
                .cooling_schedule(Box::new(
                    ExponentialCoolingScheduleBuilder::default()
                        .initial_temperature(1f32)
                        .cooling_factor(0.99f32)
                        .build()?,
                ))
                .neighbor_operator(Box::new(SwapNeighborhoodOperator::new(1)))
                .criterion_operator(Box::new(BoltzmanProbabilityCriterionOperator {}))
                .logger(Box::new(CSVLogger::new(instance, None)))
                .build()?,
        ),
        Box::new(
            SimulatedAnnealingBuilder::default()
                .iterations(10000)
                .cooling_schedule(Box::new(
                    ExponentialCoolingScheduleBuilder::default()
                        .initial_temperature(1f32)
                        .cooling_factor(0.99f32)
                        .build()?,
                ))
                .neighbor_operator(Box::new(SwapNeighborhoodOperator::new(20)))
                .criterion_operator(Box::new(BoltzmanProbabilityCriterionOperator {}))
                .logger(Box::new(CSVLogger::new(instance, None)))
                .build()?,
        ),
        Box::new(
            SimulatedAnnealingBuilder::default()
                .iterations(10000)
                .cooling_schedule(Box::new(
                    ExponentialCoolingScheduleBuilder::default()
                        .initial_temperature(1f32)
                        .cooling_factor(0.99f32)
                        .build()?,
                ))
                .neighbor_operator(Box::new(InverseNeighborhoodOperator::new(10)))
                .criterion_operator(Box::new(BoltzmanProbabilityCriterionOperator {}))
                .logger(Box::new(CSVLogger::new(instance, None)))
                .build()?,
        ),
        Box::new(
            SimulatedAnnealingBuilder::default()
                .iterations(10000)
                .cooling_schedule(Box::new(
                    ExponentialCoolingScheduleBuilder::default()
                        .initial_temperature(1f32)
                        .cooling_factor(0.99f32)
                        .build()?,
                ))
                .neighbor_operator(Box::new(InverseNeighborhoodOperator::new(1)))
                .criterion_operator(Box::new(BoltzmanProbabilityCriterionOperator {}))
                .logger(Box::new(CSVLogger::new(instance, None)))
                .build()?,
        ),
        Box::new(
            SimulatedAnnealingBuilder::default()
                .iterations(10000)
                .cooling_schedule(Box::new(
                    ExponentialCoolingScheduleBuilder::default()
                        .initial_temperature(1f32)
                        .cooling_factor(0.80f32)
                        .build()?,
                ))
                .neighbor_operator(Box::new(SwapNeighborhoodOperator::new(10)))
                .criterion_operator(Box::new(BoltzmanProbabilityCriterionOperator {}))
                .logger(Box::new(CSVLogger::new(instance, None)))
                .build()?,
        ),
        Box::new(
            SimulatedAnnealingBuilder::default()
                .iterations(10000)
                .cooling_schedule(Box::new(
                    ExponentialCoolingScheduleBuilder::default()
                        .initial_temperature(0.5f32)
                        .cooling_factor(0.80f32)
                        .build()?,
                ))
                .neighbor_operator(Box::new(SwapNeighborhoodOperator::new(10)))
                .criterion_operator(Box::new(BoltzmanProbabilityCriterionOperator {}))
                .logger(Box::new(CSVLogger::new(instance, None)))
                .build()?,
        ),
        Box::new(
            SimulatedAnnealingBuilder::default()
                .iterations(10000)
                .cooling_schedule(Box::new(LinearCoolingSchedule::new(10000)))
                .neighbor_operator(Box::new(SwapNeighborhoodOperator::new(10)))
                .criterion_operator(Box::new(BoltzmanProbabilityCriterionOperator {}))
                .logger(Box::new(CSVLogger::new(instance, None)))
                .build()?,
        ),
        Box::new(
            SimulatedAnnealingBuilder::default()
                .iterations(10000)
                .cooling_schedule(Box::new(LinearCoolingSchedule::new(10000)))
                .neighbor_operator(Box::new(InverseNeighborhoodOperator::new(10)))
                .criterion_operator(Box::new(BoltzmanProbabilityCriterionOperator {}))
                .logger(Box::new(CSVLogger::new(instance, None)))
                .build()?,
        ),
    ])
}

pub fn get_simulated_annealing_best_three(instance: &str) -> Result<Vec<Box<dyn Solver>>> {
    let iteration_info_headers = vec![
        "configuration".to_string(),
        "iteration".to_string(),
        "temperature".to_string(),
        "best_fitness".to_string(),
        "current_fitness".to_string(),
    ];
    Ok(vec![
        Box::new(
            SimulatedAnnealingBuilder::default()
                .iterations(10000)
                .cooling_schedule(Box::new(
                    ExponentialCoolingScheduleBuilder::default()
                        .initial_temperature(1f32)
                        .cooling_factor(0.995f32)
                        .build()?,
                ))
                .neighbor_operator(Box::new(SwapNeighborhoodOperator::new(10)))
                .criterion_operator(Box::new(BoltzmanProbabilityCriterionOperator {}))
                .logger(Box::new(CSVLogger::new(
                    instance,
                    Some(iteration_info_headers),
                )))
                .build()?,
        ),
        Box::new(
            SimulatedAnnealingBuilder::default()
                .iterations(10000)
                .cooling_schedule(Box::new(
                    ExponentialCoolingScheduleBuilder::default()
                        .initial_temperature(1f32)
                        .cooling_factor(0.99f32)
                        .build()?,
                ))
                .neighbor_operator(Box::new(SwapNeighborhoodOperator::new(20)))
                .criterion_operator(Box::new(BoltzmanProbabilityCriterionOperator {}))
                .logger(Box::new(CSVLogger::new(instance, None)))
                .build()?,
        ),
        Box::new(
            SimulatedAnnealingBuilder::default()
                .iterations(10000)
                .cooling_schedule(Box::new(
                    ExponentialCoolingScheduleBuilder::default()
                        .initial_temperature(1f32)
                        .cooling_factor(0.999f32)
                        .build()?,
                ))
                .neighbor_operator(Box::new(SwapNeighborhoodOperator::new(40)))
                .criterion_operator(Box::new(BoltzmanProbabilityCriterionOperator {}))
                .logger(Box::new(CSVLogger::new(instance, None)))
                .build()?,
        ),
    ])
}
