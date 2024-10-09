use crate::solution::{Solution, SolutionQuality};
use anyhow::Result;

pub trait Problem {
    fn eval(&self, solution: &Solution) -> Result<SolutionQuality>;
    fn random_solution(&self) -> Solution;
}
