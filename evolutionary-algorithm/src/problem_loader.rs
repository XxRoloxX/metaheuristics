use crate::problem::Problem;
use crate::solution::{Solution, SolutionEntry, SolutionQuality};
use anyhow::{anyhow, Ok, Result};

#[cfg(test)]
mod tests;

#[derive(Debug)]
pub struct Coordinates {
    pub x: u16,
    pub y: u16,
}

impl Coordinates {
    pub fn distance(&self, other: &Coordinates) -> f32 {
        let sum = (self.x as i16 - other.x as i16).pow(2) + (self.y as i16 - other.y as i16).pow(2);
        f32::from(sum).sqrt()
    }
}

#[derive(Debug, Default)]
pub struct CVRProblem {
    name: String,
    comment: String,
    type_: String,
    edge_weight_type: String,
    capacity: Capacity,
    dimension: usize,
    node_coordinates: Vec<Coordinates>,
    demands: Vec<Demand>,
    depots: Vec<u16>,
    distances: Option<Vec<Vec<SolutionQuality>>>,
}

type Capacity = u16;
pub type Demand = u16;

impl Problem for CVRProblem {
    fn random_solution(&self) -> Solution {
        let mut left_nodes = (1..self.dimension as SolutionEntry).collect::<Vec<SolutionEntry>>();
        let mut selected_nodes: Solution = vec![self.closest_depot()];

        for _ in 1..self.dimension {
            let random_index: usize = rand::random();
            let selected_node_index: usize = random_index % left_nodes.len();
            selected_nodes.push(left_nodes[selected_node_index]);
            left_nodes.remove(selected_node_index);
        }

        selected_nodes
    }
    fn eval(&self, solution: &Solution) -> Result<SolutionQuality> {
        struct CurrentState {
            node: SolutionEntry,
            distance: SolutionQuality,
            resources: u16,
        }

        let initial_state = CurrentState {
            node: solution[0],
            distance: SolutionQuality::default(),
            resources: self.capacity,
        };

        let evaluation = solution[1..]
            .iter()
            .try_fold(initial_state, |mut accum, &curr| {
                let next_demand = &self.demands(&curr)?;
                let distance = if next_demand > &accum.resources {
                    let depot = &self.closest_depot();
                    accum.resources = self.capacity - next_demand;
                    self.distance(&accum.node, depot)? + self.distance(depot, &curr)?
                } else {
                    accum.resources -= next_demand;
                    self.distance(&accum.node, &curr)?
                };

                accum.distance += distance;
                accum.node = curr;
                Ok(accum)
            })?;

        let res = evaluation.distance + self.distance(&evaluation.node, &self.closest_depot())?;
        Ok(res)
    }
}

impl CVRProblem {
    pub fn new(
        name: String,
        comment: String,
        type_: String,
        edge_weight_type: String,
        capacity: u16,
        node_coordinates: Vec<Coordinates>,
        demands: Vec<u16>,
        dimension: usize,
        distances: Option<Vec<Vec<f32>>>,
        depots: Vec<u16>,
    ) -> CVRProblem {
        CVRProblem {
            name,
            comment,
            type_,
            edge_weight_type,
            capacity,
            node_coordinates,
            demands,
            dimension,
            depots,
            distances,
        }
    }

    pub fn non_depot_nodes(&self) -> Vec<SolutionEntry> {
        (1..self.dimension as SolutionEntry).collect()
    }

    pub fn capacity(&self) -> Capacity {
        self.capacity
    }

    pub fn distance(
        &self,
        node_a: &SolutionEntry,
        node_b: &SolutionEntry,
    ) -> Result<SolutionQuality> {
        match &self.distances {
            None => Err(anyhow!(
                "Failed to get distance, distances matrix is not prepared yet"
            )),
            Some(distances) => {
                if *node_a as usize > distances.len() || *node_b as usize > distances.len() {
                    Err(anyhow!("Failed to get distance, invalid node indexes"))
                } else {
                    Ok(distances[*node_a as usize][*node_b as usize])
                }
            }
        }
    }

    pub fn demands(&self, node: &SolutionEntry) -> Result<Demand> {
        match self.demands.get(*node as usize) {
            None => Err(anyhow!("Failed to get distance, invalid node indexes")),
            Some(demand) => Ok(*demand),
        }
    }

    pub fn closest_depot(&self) -> SolutionEntry {
        self.depots[0]
    }

    pub fn precalculate_distances(&mut self) {
        self.distances = Some(vec![vec![0f32; self.dimension]; self.dimension]);

        for i in 0usize..self.dimension {
            for j in 0usize..self.dimension {
                match &mut self.distances {
                    None => {
                        continue;
                    }
                    Some(distances) => {
                        distances[i][j] =
                            self.node_coordinates[i].distance(&self.node_coordinates[j]);
                    }
                }
            }
        }
    }
}

#[derive(Copy, Clone)]
enum ProblemLoadingStage {
    Beginning,
    NodeCoordinates,
    Demands,
    Depot,
    Finished,
}

impl From<String> for CVRProblem {
    fn from(content: String) -> Self {
        let current_stage = ProblemLoadingStage::Beginning;
        let initial_problem = CVRProblem::default();

        content
            .lines()
            .fold(
                (initial_problem, current_stage),
                |accum, line| match accum.1 {
                    ProblemLoadingStage::Finished => accum,
                    ProblemLoadingStage::Demands => handle_loading_problem_demands(accum, line),
                    ProblemLoadingStage::NodeCoordinates => {
                        handle_loading_problem_node_coordinates(accum, line)
                    }
                    ProblemLoadingStage::Depot => handle_loading_problem_depots(accum, line),
                    _ => handle_loading_problem_param(accum, line),
                },
            )
            .0
    }
}

fn handle_loading_problem_demands(
    mut problem: (CVRProblem, ProblemLoadingStage),
    line: &str,
) -> (CVRProblem, ProblemLoadingStage) {
    let row = line.trim().split(" ").collect::<Vec<&str>>();
    match row.len() {
        1 => (problem.0, ProblemLoadingStage::Depot),
        _ => {
            let demand = row[1].parse::<u16>().unwrap();
            problem.0.demands.push(demand);
            problem
        }
    }
}

fn handle_loading_problem_node_coordinates(
    mut problem: (CVRProblem, ProblemLoadingStage),
    line: &str,
) -> (CVRProblem, ProblemLoadingStage) {
    let row = line.trim().split(" ").collect::<Vec<&str>>();
    match row.len() {
        1 => (problem.0, ProblemLoadingStage::Demands),
        2 => (problem.0, ProblemLoadingStage::Demands),
        _ => {
            let x = row[1].parse::<u16>().unwrap();
            let y = row[2].parse::<u16>().unwrap();
            problem.0.node_coordinates.push(Coordinates { x, y });
            problem
        }
    }
}

fn handle_loading_problem_depots(
    mut problem: (CVRProblem, ProblemLoadingStage),
    line: &str,
) -> (CVRProblem, ProblemLoadingStage) {
    let row = line.trim().parse::<i16>().unwrap();
    match row {
        -1 => (problem.0, ProblemLoadingStage::Finished),
        val => {
            problem.0.depots.push(val as u16 - 1);
            problem
        }
    }
}

fn handle_loading_problem_param(
    mut problem: (CVRProblem, ProblemLoadingStage),
    line: &str,
) -> (CVRProblem, ProblemLoadingStage) {
    let row = line.trim().split(" : ").collect::<Vec<&str>>();
    match row[0] {
        "NAME" => {
            problem.0.name = row[1].to_string();
        }
        "COMMENT" => {
            problem.0.comment = row[1].to_string();
        }
        "TYPE" => {
            problem.0.type_ = row[1].to_string();
        }
        "DIMENSION" => {
            problem.0.dimension = row[1].to_string().parse::<usize>().unwrap();
        }
        "EDGE_WEIGHT_TYPE" => {
            problem.0.edge_weight_type = row[1].to_string();
        }
        "CAPACITY" => {
            problem.0.capacity = row[1].to_string().parse::<u16>().unwrap();
        }
        "NODE_COORD_SECTION" => {
            problem.1 = ProblemLoadingStage::NodeCoordinates;
        }
        "DEMAND_SECTION" => {
            problem.1 = ProblemLoadingStage::Demands;
        }
        "DEPOT_SECTION" => {
            problem.1 = ProblemLoadingStage::Depot;
        }
        _ => {}
    };

    problem
}
