use anyhow::Result;

use crate::{
    logger::CSVLogger,
    neighbor::{InverseNeighborhoodOperator, SwapNeighborhoodOperator},
    simulated_annealing::{
        cooling_schedule::{ExponentialCoolingScheduleBuilder, LinearCoolingSchedule},
        criterion_operator::BoltzmanProbabilityCriterionOperator,
    },
    solver::Solver,
};

use super::algorithm::TSSABuilder;

pub fn get_tssa_config(instance: &str) -> Result<Vec<Box<dyn Solver>>> {
    let iteration_info_headers = vec![
        "configuration".to_string(),
        "iteration".to_string(),
        // "temperature".to_string(),
        "best_fitness".to_string(),
        "average_fitness".to_string(),
        "worst_fitness".to_string(),
        "current_fitness".to_string(),
    ];
    Ok(vec![
        Box::new(
            TSSABuilder::default()
                .iterations(20)
                .cooling_schedule(Box::new(
                    ExponentialCoolingScheduleBuilder::default()
                        .initial_temperature(1f32)
                        .cooling_factor(0.990f32)
                        .build()?,
                ))
                .algorithm_switch_interval(1000)
                .neighborhood_operator(Box::new(SwapNeighborhoodOperator::new(40)))
                .tabu_list_size(100)
                .criterion_operator(Box::new(BoltzmanProbabilityCriterionOperator {}))
                .logger(Box::new(CSVLogger::new(
                    instance,
                    Some(iteration_info_headers),
                )))
                .build()?,
        ),
        Box::new(
            TSSABuilder::default()
                .iterations(20)
                .cooling_schedule(Box::new(
                    ExponentialCoolingScheduleBuilder::default()
                        .initial_temperature(1f32)
                        .cooling_factor(0.999f32)
                        .build()?,
                ))
                .algorithm_switch_interval(1000)
                .neighborhood_operator(Box::new(SwapNeighborhoodOperator::new(40)))
                .tabu_list_size(200)
                .criterion_operator(Box::new(BoltzmanProbabilityCriterionOperator {}))
                .logger(Box::new(CSVLogger::new(instance, None)))
                .build()?,
        ),
        Box::new(
            TSSABuilder::default()
                .iterations(20)
                .cooling_schedule(Box::new(
                    ExponentialCoolingScheduleBuilder::default()
                        .initial_temperature(1f32)
                        .cooling_factor(0.999f32)
                        .build()?,
                ))
                .algorithm_switch_interval(1000)
                .neighborhood_operator(Box::new(InverseNeighborhoodOperator::new(40)))
                .tabu_list_size(100)
                .criterion_operator(Box::new(BoltzmanProbabilityCriterionOperator {}))
                .logger(Box::new(CSVLogger::new(instance, None)))
                .build()?,
        ),
        Box::new(
            TSSABuilder::default()
                .iterations(20)
                .cooling_schedule(Box::new(
                    ExponentialCoolingScheduleBuilder::default()
                        .initial_temperature(1f32)
                        .cooling_factor(0.999f32)
                        .build()?,
                ))
                .algorithm_switch_interval(1000)
                .neighborhood_operator(Box::new(InverseNeighborhoodOperator::new(40)))
                .tabu_list_size(50)
                .criterion_operator(Box::new(BoltzmanProbabilityCriterionOperator {}))
                .logger(Box::new(CSVLogger::new(instance, None)))
                .build()?,
        ),
        Box::new(
            TSSABuilder::default()
                .iterations(20)
                .cooling_schedule(Box::new(
                    ExponentialCoolingScheduleBuilder::default()
                        .initial_temperature(1f32)
                        .cooling_factor(0.999f32)
                        .build()?,
                ))
                .algorithm_switch_interval(1000)
                .neighborhood_operator(Box::new(InverseNeighborhoodOperator::new(40)))
                .tabu_list_size(200)
                .criterion_operator(Box::new(BoltzmanProbabilityCriterionOperator {}))
                .logger(Box::new(CSVLogger::new(instance, None)))
                .build()?,
        ),
    ])
}
