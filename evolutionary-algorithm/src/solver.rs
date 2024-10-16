use anyhow::Result;

use crate::{individual::VecIndividual, problem::Problem};

pub trait Solver<'a> {
    fn solve(&'a mut self, problem: &'a dyn Problem) -> Result<VecIndividual>;
}
