use anyhow::{Context, Result};
use rand::{seq::SliceRandom, RngCore};

use crate::{
    individual::{Fitness, VecIndividual},
    population::Population,
    problem::Problem,
    simulated_annealing::algorithm::Temperature,
};

pub trait Selector {
    fn select(&mut self, population: Population, problem: &dyn Problem) -> Result<Population>;
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
    fn select(&mut self, population: Population, problem: &dyn Problem) -> Result<Population> {
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

#[derive(Default)]
pub struct SimulatedAnnealingRouletteSelector {
    temperature: Temperature,
    cooldown_factor: f32,
}

impl SimulatedAnnealingRouletteSelector {
    pub fn new(initial_temperature: Temperature, cooldown_factor: f32) -> Self {
        Self {
            temperature: initial_temperature,
            cooldown_factor,
        }
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
            if selected_probability >= summed_probability
                && selected_probability < summed_probability + probabilities[individual_idx]
            {
                return Ok(&population.solutions()[individual_idx]);
            }

            summed_probability += probabilities[individual_idx];
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
    fn select(&mut self, population: Population, problem: &dyn Problem) -> Result<Population> {
        let mut next_generation: Vec<VecIndividual> = Vec::new();
        let mut probabilities: Vec<f32> = Vec::new();
        let mut rng = rand::thread_rng();

        let scores: Vec<Fitness> = population
            .solutions()
            .iter()
            .map(|sol| problem.eval(sol))
            .collect::<Result<Vec<Fitness>>>()?;

        let min_score = scores
            .iter()
            .min_by(|x, y| x.total_cmp(y))
            .context("Failed to compare scores")?;
        let max_score = scores
            .iter()
            .max_by(|x, y| x.total_cmp(y))
            .context("Failed to compare scores")?;

        let scores_sum: Fitness = scores
            .iter()
            .map(|score| (score - min_score) / (max_score - min_score))
            .sum();

        for score in scores
            .iter()
            .map(|score| (score - min_score) / (max_score - min_score))
        {
            let probability: f32 = score / scores_sum;
            probabilities.push(probability);
        }

        for _ in 0..population.number_of_solutions() {
            let random_probality: f32 = (rng.next_u32() % 100) as f32 / (100f32);
            let child =
                self.find_individual_by_probability(&population, &probabilities, random_probality)?;

            next_generation.push(VecIndividual::from(child));
        }

        Ok(Population::new(next_generation))
    }
}

impl SimulatedAnnealingRouletteSelector {
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
            if selected_probability >= summed_probability
                && selected_probability < summed_probability + probabilities[individual_idx]
            {
                return Ok(&population.solutions()[individual_idx]);
            }

            summed_probability += probabilities[individual_idx];
        }

        population
            .solutions()
            .last()
            .context("No idividuals in population")
    }
}

impl Selector for SimulatedAnnealingRouletteSelector {
    fn name(&self) -> String {
        format!("saea: cooldown: {}", self.cooldown_factor)
    }
    fn select(&mut self, population: Population, problem: &dyn Problem) -> Result<Population> {
        let mut next_generation: Vec<VecIndividual> = Vec::new();
        let mut probabilities: Vec<f32> = Vec::new();
        let mut rng = rand::thread_rng();

        let mut scores: Vec<Fitness> = population
            .solutions()
            .iter()
            .map(|sol| problem.eval(sol))
            .collect::<Result<Vec<Fitness>>>()?;

        let max_index = scores
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.total_cmp(b))
            .map(|(index, _)| index)
            .unwrap();

        scores[max_index] *= 1f32 / self.temperature;

        self.temperature *= self.cooldown_factor;

        let min_score = scores
            .iter()
            .min_by(|x, y| x.total_cmp(y))
            .context("Failed to compare scores")?;
        let max_score = scores
            .iter()
            .max_by(|x, y| x.total_cmp(y))
            .context("Failed to compare scores")?;

        let scores_sum: Fitness = scores
            .iter()
            .map(|score| (score - min_score) / (max_score - min_score))
            .sum();

        for score in scores
            .iter()
            .map(|score| (score - min_score) / (max_score - min_score))
        {
            let probability: f32 = score / scores_sum;
            probabilities.push(probability);
        }

        for _ in 0..population.number_of_solutions() {
            let random_probality: f32 = (rng.next_u32() % 100) as f32 / (100f32);
            let child =
                self.find_individual_by_probability(&population, &probabilities, random_probality)?;

            next_generation.push(VecIndividual::from(child));
        }

        Ok(Population::new(next_generation))
    }
}
