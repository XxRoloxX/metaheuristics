use anyhow::Result;

use crate::{population::Population, problem::Problem, solution::Solution};

pub trait Solver {
    fn solve(&mut self, problem: &dyn Problem) -> Result<Solution>;

    // fn get_generation(population_size: u16) -> Population {
    //     let initial_solutions: Vec<Solution> = (0..population_size)
    //         .map(|_| problem.random_solution())
    //         .collect();
    // }
}
