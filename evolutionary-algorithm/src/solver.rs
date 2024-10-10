use anyhow::Result;

use crate::{problem::Problem, solution::Solution};

pub trait Solver {
    fn solve(&mut self, problem: &dyn Problem) -> Result<Solution>;
}
