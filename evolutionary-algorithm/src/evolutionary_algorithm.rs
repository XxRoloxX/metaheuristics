use anyhow::Result;

use crate::{
    population::Population,
    problem::Problem,
    selection::Selector,
    solution::{Solution, SolutionQuality},
    solver::Solver,
};

pub struct EvolutionaryAlgorithm<'a> {
    generations: u16,
    population_size: u16,
    best_solution: Option<Solution>,
    best_solution_quality: Option<SolutionQuality>,
    selection_operator: &'a dyn Selector,
}

impl<'a> EvolutionaryAlgorithm<'a> {
    fn get_generation(&self, problem: &dyn Problem) -> Population {
        let initial_solutions: Vec<Solution> = (0..self.population_size)
            .map(|_| problem.random_solution())
            .collect();

        Population::new(initial_solutions)
    }

    pub fn new(
        generations: u16,
        population_size: u16,
        selection_operator: &'a dyn Selector,
    ) -> Self {
        Self {
            generations,
            population_size,
            best_solution: None,
            best_solution_quality: None,
            selection_operator,
        }
    }
}

impl<'a> Solver for EvolutionaryAlgorithm<'a> {
    fn solve(&mut self, problem: &dyn Problem) -> Result<Solution> {
        let mut population = self.get_generation(problem);
        for _ in 0..self.generations {
            // Generates initial population
            let best_in_generation = population.best(problem);
            let best_in_generation_quality = problem.eval(&best_in_generation)?;
            match self.best_solution_quality {
                None => {
                    self.best_solution_quality = Some(best_in_generation_quality);
                    self.best_solution = Some(best_in_generation);
                }
                Some(quality) if (best_in_generation_quality > quality) => {
                    self.best_solution_quality = Some(best_in_generation_quality);
                    self.best_solution = Some(best_in_generation);
                }
                _ => {}
            }

            population = self.selection_operator.select(population);
            // println!("Population {:?}", population.solutions());
        }

        Ok(self.best_solution.clone().unwrap())
    }
}
