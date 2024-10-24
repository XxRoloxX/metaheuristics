use anyhow::Result;

use crate::{logger::CSVLogger, solver::Solver};

use super::{
    algorithm::TabuSearchBuilder,
    neighbor::{InverseNeighborhoodOperator, SwapNeighborhoodOperator},
};

pub fn get_tabu_search_scores_comparison_configurations(
    instance: &str,
) -> Result<Vec<Box<dyn Solver>>> {
    let iteration_info_headers = vec![
        "configuration".to_string(),
        "iteration".to_string(),
        "tabu_size".to_string(),
        "best_fitness".to_string(),
        "average_fitness".to_string(),
        "worst_fitness".to_string(),
        "current_fitness".to_string(),
    ];
    Ok(vec![Box::new(
        TabuSearchBuilder::default()
            .iterations(1000)
            .tabu_list_size(100)
            .neighborhood_operator(Box::new(SwapNeighborhoodOperator::new(10)))
            .logger(Box::new(CSVLogger::new(
                instance,
                Some(iteration_info_headers),
            )))
            .build()?,
    )])
}

pub fn get_tabu_search_general_configurations(instance: &str) -> Result<Vec<Box<dyn Solver>>> {
    let iteration_info_headers = vec![
        "configuration".to_string(),
        "iteration".to_string(),
        "tabu_size".to_string(),
        "best_fitness".to_string(),
        "average_fitness".to_string(),
        "worst_fitness".to_string(),
        "current_fitness".to_string(),
    ];
    Ok(vec![
        Box::new(
            TabuSearchBuilder::default()
                .iterations(1000)
                .tabu_list_size(100)
                .neighborhood_operator(Box::new(SwapNeighborhoodOperator::new(10)))
                .logger(Box::new(CSVLogger::new(
                    instance,
                    Some(iteration_info_headers),
                )))
                .build()?,
        ),
        Box::new(
            TabuSearchBuilder::default()
                .iterations(1000)
                .tabu_list_size(200)
                .neighborhood_operator(Box::new(SwapNeighborhoodOperator::new(10)))
                .logger(Box::new(CSVLogger::new(instance, None)))
                .build()?,
        ),
        Box::new(
            TabuSearchBuilder::default()
                .iterations(1000)
                .tabu_list_size(50)
                .neighborhood_operator(Box::new(SwapNeighborhoodOperator::new(10)))
                .logger(Box::new(CSVLogger::new(instance, None)))
                .build()?,
        ),
        Box::new(
            TabuSearchBuilder::default()
                .iterations(1000)
                .tabu_list_size(25)
                .neighborhood_operator(Box::new(SwapNeighborhoodOperator::new(10)))
                .logger(Box::new(CSVLogger::new(instance, None)))
                .build()?,
        ),
        Box::new(
            TabuSearchBuilder::default()
                .iterations(1000)
                .tabu_list_size(100)
                .neighborhood_operator(Box::new(SwapNeighborhoodOperator::new(20)))
                .logger(Box::new(CSVLogger::new(instance, None)))
                .build()?,
        ),
        Box::new(
            TabuSearchBuilder::default()
                .iterations(1000)
                .tabu_list_size(10)
                .neighborhood_operator(Box::new(SwapNeighborhoodOperator::new(10)))
                .logger(Box::new(CSVLogger::new(instance, None)))
                .build()?,
        ),
        Box::new(
            TabuSearchBuilder::default()
                .iterations(1000)
                .tabu_list_size(100)
                .neighborhood_operator(Box::new(SwapNeighborhoodOperator::new(5)))
                .logger(Box::new(CSVLogger::new(instance, None)))
                .build()?,
        ),
        Box::new(
            TabuSearchBuilder::default()
                .iterations(1000)
                .tabu_list_size(10)
                .neighborhood_operator(Box::new(InverseNeighborhoodOperator::new(25)))
                .logger(Box::new(CSVLogger::new(instance, None)))
                .build()?,
        ),
        Box::new(
            TabuSearchBuilder::default()
                .iterations(1000)
                .tabu_list_size(10)
                .neighborhood_operator(Box::new(InverseNeighborhoodOperator::new(10)))
                .logger(Box::new(CSVLogger::new(instance, None)))
                .build()?,
        ),
        Box::new(
            TabuSearchBuilder::default()
                .iterations(1000)
                .tabu_list_size(10)
                .neighborhood_operator(Box::new(InverseNeighborhoodOperator::new(5)))
                .logger(Box::new(CSVLogger::new(instance, None)))
                .build()?,
        ),
    ])
}

pub fn get_tabu_search_neighbors(instance: &str) -> Result<Vec<Box<dyn Solver>>> {
    let iteration_info_headers = vec![
        "configuration".to_string(),
        "iteration".to_string(),
        "tabu_size".to_string(),
        "best_fitness".to_string(),
        "average_fitness".to_string(),
        "worst_fitness".to_string(),
        "current_fitness".to_string(),
    ];
    Ok(vec![
        Box::new(
            TabuSearchBuilder::default()
                .iterations(1000)
                .tabu_list_size(100)
                .neighborhood_operator(Box::new(SwapNeighborhoodOperator::new(20)))
                .logger(Box::new(CSVLogger::new(
                    instance,
                    Some(iteration_info_headers),
                )))
                .build()?,
        ),
        Box::new(
            TabuSearchBuilder::default()
                .iterations(1000)
                .tabu_list_size(10)
                .neighborhood_operator(Box::new(SwapNeighborhoodOperator::new(100)))
                .logger(Box::new(CSVLogger::new(instance, None)))
                .build()?,
        ),
        Box::new(
            TabuSearchBuilder::default()
                .iterations(1000)
                .tabu_list_size(100)
                .neighborhood_operator(Box::new(SwapNeighborhoodOperator::new(50)))
                .logger(Box::new(CSVLogger::new(instance, None)))
                .build()?,
        ),
        Box::new(
            TabuSearchBuilder::default()
                .iterations(1000)
                .tabu_list_size(10)
                .neighborhood_operator(Box::new(SwapNeighborhoodOperator::new(25)))
                .logger(Box::new(CSVLogger::new(instance, None)))
                .build()?,
        ),
        Box::new(
            TabuSearchBuilder::default()
                .iterations(1000)
                .tabu_list_size(100)
                .neighborhood_operator(Box::new(SwapNeighborhoodOperator::new(200)))
                .logger(Box::new(CSVLogger::new(instance, None)))
                .build()?,
        ),
        Box::new(
            TabuSearchBuilder::default()
                .iterations(1000)
                .tabu_list_size(100)
                .neighborhood_operator(Box::new(SwapNeighborhoodOperator::new(100)))
                .logger(Box::new(CSVLogger::new(instance, None)))
                .build()?,
        ),
    ])
}
