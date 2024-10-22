use anyhow::Result;

use crate::{logger::CSVLogger, solver::Solver};

use super::{algorithm::TabuSearchBuilder, neighbor::SwapNeighborhoodOperator};

pub fn get_tabu_search_configuration(instance: &str) -> Result<Vec<Box<dyn Solver>>> {
    let iteration_info_headers = vec![
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
                format!("./csv/{}-tabu", instance).as_str(),
                Some(iteration_info_headers),
            )))
            .build()?,
    )])
}
