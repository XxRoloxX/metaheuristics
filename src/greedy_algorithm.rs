use anyhow::{Context, Result};

use crate::{
    individual::{Gene, VecIndividual},
    problem_loader::{CVRProblem, Demand},
};

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

impl<'a> GreedyAlgorithm<'a> {
    pub fn new(problem: &'a CVRProblem) -> Self {
        GreedyAlgorithm { problem }
    }

    pub fn solve(&mut self, first_node: Gene) -> Result<VecIndividual> {
        let initial = TripState {
            current_node: first_node,
            resources: self.problem.capacity() - self.problem.demands(&first_node)?,
            visited_nodes: vec![],
            unvisited_nodes: self
                .problem
                .stops()
                .iter()
                .filter(|stop| **stop != first_node)
                .copied()
                .collect::<Vec<Gene>>(),
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

                if self.need_trip_to_depot(&accum, closest_node)? {
                    accum.resources =
                        self.problem.capacity() - self.problem.demands(&closest_node)?
                } else {
                    accum.resources -= self.problem.demands(&closest_node)?
                }

                accum.visited_nodes.push(accum.current_node);
                accum.current_node = closest_node;
                Ok(accum)
            },
        )?;

        res.visited_nodes.push(res.current_node);

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
