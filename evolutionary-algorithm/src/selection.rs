use anyhow::{Context, Result};
use rand::{seq::SliceRandom, RngCore};

use crate::{
    individual::{Fitness, VecIndividual},
    population::Population,
    problem::Problem,
};

pub trait Selector {
    fn select(&self, population: Population, problem: &dyn Problem) -> Result<Population>;
    fn name(&self) -> String;
}

pub struct TournamentSelector {
    size: u16,
}

impl TournamentSelector {
    pub fn new(size: u16) -> Self {
        Self { size }
    }
}

impl Selector for TournamentSelector {
    fn name(&self) -> String {
        format!("tournament of {}", self.size)
    }
    fn select(&self, population: Population, problem: &dyn Problem) -> Result<Population> {
        let mut next_generation: Vec<VecIndividual> = Vec::new();

        for _ in 0..population.solutions().len() {
            let tournament: Vec<VecIndividual> = population
                .solutions()
                .choose_multiple(&mut rand::thread_rng(), self.size as usize)
                .map(|solution| solution.into())
                .collect();

            let (best_solution, _) =
                Population::individual_with_highest_fitness(problem, &tournament);

            next_generation.push(VecIndividual::from(best_solution));
        }

        Ok(Population::new(next_generation))
    }
}
#[derive(Default)]
pub struct RouletteSelector {}

impl RouletteSelector {
    pub fn new() -> Self {
        Self {}
    }
}

impl RouletteSelector {
    fn find_individual_by_probability<'b>(
        &self,
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

impl Selector for RouletteSelector {
    fn name(&self) -> String {
        String::from("roulette")
    }
    fn select(&self, population: Population, problem: &dyn Problem) -> Result<Population> {
        let mut next_generation: Vec<VecIndividual> = Vec::new();
        let mut probabilities: Vec<f32> = Vec::new();
        let mut rng = rand::thread_rng();

        let score_sum: Fitness = population
            .solutions()
            .iter()
            .map(|sol| problem.eval(sol))
            .collect::<Result<Vec<Fitness>>>()?
            .iter()
            .sum();

        for individual in population.solutions() {
            let probability: f32 = problem.eval(individual)? / score_sum;
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
