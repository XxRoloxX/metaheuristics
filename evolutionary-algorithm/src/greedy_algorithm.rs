use anyhow::{anyhow, Context, Result};

use crate::{
    problem_loader::{CVRProblem, Demand},
    solution::{Solution, SolutionEntry},
};

// Teoria wybranych elementów, operatory, funkcjie oceniające wykresy, porównanie losowego/greedy i
// AE

struct TripState {
    current_node: SolutionEntry,
    resources: Demand,
    visited_nodes: Vec<SolutionEntry>,
    unvisited_nodes: Vec<SolutionEntry>,
}

pub struct GreedyAlgorithm<'a> {
    problem: &'a CVRProblem,
}

impl<'a> GreedyAlgorithm<'a> {
    pub fn new(problem: &'a CVRProblem) -> Self {
        GreedyAlgorithm { problem }
    }

    pub fn solve(&mut self) -> Result<Solution> {
        let initial = TripState {
            current_node: self.problem.closest_depot(),
            resources: self.problem.capacity(),
            visited_nodes: Solution::new(),
            unvisited_nodes: self.problem.non_depot_nodes(),
        };

        let res: Result<TripState> =
            (0..initial.unvisited_nodes.len()).try_fold(initial, |mut accum, _| {
                let closest_node = self.find_closest_node(&accum)?;
                accum.visited_nodes.push(closest_node);
                accum.unvisited_nodes = accum
                    .unvisited_nodes
                    .into_iter()
                    .filter(|val| *val != closest_node)
                    .collect::<Vec<SolutionEntry>>();
                Ok(accum)
            });

        Ok(res?.visited_nodes)
    }

    fn find_closest_node(&self, state: &TripState) -> Result<SolutionEntry> {
        let closes_node = state
            .unvisited_nodes
            .iter()
            .min_by_key(|node| {
                if self.need_trip_to_depot(state, **node).unwrap() {
                    self.problem
                        .distance(&state.current_node, &self.problem.closest_depot())
                        .unwrap() as i32
                        + self
                            .problem
                            .distance(&self.problem.closest_depot(), node)
                            .unwrap() as i32
                } else {
                    self.problem.distance(&state.current_node, node).unwrap() as i32
                }
            })
            .context("Oh boy")?;

        Ok(*closes_node)
    }

    fn need_trip_to_depot(&self, state: &TripState, to: SolutionEntry) -> Result<bool> {
        let next_demand = self.problem.demands(&to)?;
        Ok(next_demand > state.resources)
    }
}
