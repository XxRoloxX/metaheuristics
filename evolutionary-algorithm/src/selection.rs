use crate::{population::Population, problem::Problem, solution::Solution};
use rand::seq::SliceRandom;

pub trait Selector {
    fn select(&self, population: Population) -> Population;
}

pub struct TournamentSelector<'a> {
    size: u16,
    problem: &'a dyn Problem,
}

impl<'a> TournamentSelector<'a> {
    pub fn new(size: u16, problem: &'a dyn Problem) -> Self {
        Self { size, problem }
    }
}

impl<'a> Selector for TournamentSelector<'a> {
    fn select(&self, population: Population) -> Population {
        let mut next_generation: Vec<Solution> = Vec::new();

        for _ in 0..population.solutions().len() {
            let tournament: Vec<&Solution> = population
                .solutions()
                .choose_multiple(&mut rand::thread_rng(), self.size as usize)
                .collect();

            let best_solution = Population::best_solution(self.problem, tournament);
            next_generation.push(best_solution.clone())
        }

        Population::new(next_generation)
    }
}
