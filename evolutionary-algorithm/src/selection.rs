use rand::seq::SliceRandom;

use crate::{individual::VecIndividual, population::Population, problem::Problem};

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
        let mut next_generation: Vec<VecIndividual> = Vec::new();

        for _ in 0..population.solutions().len() {
            let tournament: Vec<&VecIndividual> = population
                .solutions()
                .choose_multiple(&mut rand::thread_rng(), self.size as usize)
                .collect();

            let best_solution =
                Population::individual_with_highest_fitness(self.problem, &tournament);

            next_generation.push(VecIndividual::from(best_solution));
        }

        Population::new(next_generation)
    }
}
