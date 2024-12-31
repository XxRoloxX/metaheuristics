use anyhow::Result;

use crate::{
    individual::{Fitness, VecIndividual},
    problem::Problem,
};

pub trait Solver {
    fn solve(&mut self, problem: &dyn Problem) -> Result<(Fitness, VecIndividual)>;
}
