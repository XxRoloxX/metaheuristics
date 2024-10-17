use anyhow::{Context, Result};

use crate::{
    individual::{Fitness, Gene, VecIndividual},
    problem::Problem,
    problem_loader::{CVRProblem, Demand},
    solver::Solver,
};

// Teoria wybranych elementów, operatory, funkcjie oceniające wykresy, porównanie losowego/greedy i
// AE
//
type Node = Gene;

struct TripState {
    current_node: Node,
    resources: Demand,
    visited_nodes: Vec<Node>,
    unvisited_nodes: Vec<Node>,
}

pub struct GreedyAlgorithm<'a> {
    problem: &'a CVRProblem,
}

// impl<'a> Solver for GreedyAlgorithm<'a> {
//     fn solve(&mut self, problem: &dyn Problem) -> Result<(Fitness, VecIndividual)> {
//
//     }
// }

impl<'a> GreedyAlgorithm<'a> {
    pub fn new(problem: &'a CVRProblem) -> Self {
        GreedyAlgorithm { problem }
    }

    pub fn solve(&mut self) -> Result<VecIndividual> {
        let initial = TripState {
            current_node: self.problem.closest_depot(),
            resources: self.problem.capacity(),
            visited_nodes: Vec::new(),
            unvisited_nodes: self.problem.stops().clone(),
        };

        let mut res = (0..initial.unvisited_nodes.len()).try_fold(
            initial,
            |mut accum, _| -> Result<TripState> {
                let closest_node = self.find_closest_node(&accum)?;
                accum.unvisited_nodes = accum
                    .unvisited_nodes
                    .into_iter()
                    .filter(|val| *val != closest_node)
                    .collect::<Vec<Node>>();

                accum.visited_nodes.push(accum.current_node);
                accum.current_node = closest_node;
                Ok(accum)
            },
        )?;

        res.visited_nodes.push(res.current_node);

        //Remove first depot from the list (it shouldn't be encoded into a solution)
        res.visited_nodes.remove(0);

        Ok(VecIndividual::from(&res.visited_nodes))
    }

    fn find_closest_node(&self, state: &TripState) -> Result<Node> {
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

    fn need_trip_to_depot(&self, state: &TripState, to: Node) -> Result<bool> {
        let next_demand = self.problem.demands(&to)?;
        Ok(next_demand > state.resources)
    }
}
