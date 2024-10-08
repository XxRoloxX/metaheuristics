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
    capacity: u16,
    dimenstion: usize,
    node_coordinates: Vec<Coordinates>,
    demands: Vec<u16>,
    depots: Vec<u16>,
    distances: Option<Vec<Vec<f32>>>,
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
        dimenstion: usize,
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
            dimenstion,
            depots,
            distances,
        }
    }

    pub fn precalculate_distances(&mut self) {
        self.distances = Some(vec![vec![0f32; self.dimenstion]; self.dimenstion]);

        for i in 0usize..self.dimenstion {
            for j in 0usize..self.dimenstion {
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
            problem.0.depots.push(val as u16);
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
            problem.0.dimenstion = row[1].to_string().parse::<usize>().unwrap();
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
    return problem;
}
