use anyhow::{Context, Result};
use rand::{seq::SliceRandom, RngCore};

use crate::{
    individual::{Fitness, VecIndividual},
    population::Population,
    problem::Problem,
};

pub trait Selector {
    fn select(&self, population: Population) -> Result<Population>;
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
    fn select(&self, population: Population) -> Result<Population> {
        let mut next_generation: Vec<VecIndividual> = Vec::new();

        for _ in 0..population.solutions().len() {
            let tournament: Vec<VecIndividual> = population
                .solutions()
                .choose_multiple(&mut rand::thread_rng(), self.size as usize)
                .map(|solution| solution.into())
                .collect();

            let (best_solution, _) =
                Population::individual_with_highest_fitness(self.problem, &tournament);

            next_generation.push(VecIndividual::from(best_solution));
        }

        Ok(Population::new(next_generation))
    }
}

pub struct RouletteSelector<'a> {
    // size: u16,
    problem: &'a dyn Problem,
}

impl<'a> RouletteSelector<'a> {
    pub fn new(problem: &'a dyn Problem) -> Self {
        Self { problem }
    }
}

impl<'a> RouletteSelector<'a> {
    fn find_individual_by_probability<'b>(
        &'a self,
        population: &'b Population,
        probabilities: &[f32],
        selected_probability: f32,
    ) -> Result<&'b VecIndividual> {
        let mut summed_probability = 0f32;

        let populations_iter =
            (0..population.number_of_solutions()).zip(1..population.number_of_solutions());

        for (individual_idx, next_individual_idx) in populations_iter {
            summed_probability += probabilities[individual_idx];
            if selected_probability >= summed_probability
                && selected_probability < summed_probability + probabilities[next_individual_idx]
            {
                return Ok(&population.solutions()[individual_idx]);
            }
        }

        population
            .solutions()
            .last()
            .context("No idividuals in population")
    }
}

impl<'a> Selector for RouletteSelector<'a> {
    fn select(&self, population: Population) -> Result<Population> {
        let mut next_generation: Vec<VecIndividual> = Vec::new();
        let mut probabilities: Vec<f32> = Vec::new();
        let mut rng = rand::thread_rng();

        let score_sum: Fitness = population
            .solutions()
            .iter()
            .map(|sol| self.problem.eval(sol))
            .collect::<Result<Vec<Fitness>>>()?
            .iter()
            .sum();

        for individual in population.solutions() {
            let probability: f32 = self.problem.eval(individual)? / score_sum;
            probabilities.push(probability);
        }

        for _ in 0..population.number_of_solutions() {
            let random_probality: f32 = (rng.next_u32() % 100) as f32 / (100 as f32);
            let child =
                self.find_individual_by_probability(&population, &probabilities, random_probality)?;

            next_generation.push(VecIndividual::from(child));
        }

        Ok(Population::new(next_generation))
    }
}
