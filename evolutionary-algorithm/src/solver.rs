use anyhow::Result;

use crate::{individual::VecIndividual, problem::Problem};

pub trait Solver {
    fn solve(&mut self, problem: &dyn Problem) -> Result<VecIndividual>;
}
