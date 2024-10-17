use rand::{thread_rng, RngCore};

use crate::{
    individual::{Fitness, VecIndividual},
    problem::Problem,
};

#[derive(Default)]
pub struct Population {
    individuals: Vec<VecIndividual>,
}

impl Population {
    pub fn new(individuals: Vec<VecIndividual>) -> Self {
        Population { individuals }
    }
    pub fn solutions(&self) -> &Vec<VecIndividual> {
        &self.individuals
    }

    pub fn mut_solutions(&mut self) -> &mut Vec<VecIndividual> {
        &mut self.individuals
    }

    pub fn number_of_solutions(&self) -> usize {
        self.individuals.len()
    }

    pub fn into_solutions(self) -> Vec<VecIndividual> {
        self.individuals
    }
    pub fn add_individual(&mut self, individual: VecIndividual) {
        self.individuals.push(individual)
    }

    pub fn random_individual_index(&self) -> usize {
        let mut rng = thread_rng();
        rng.next_u32() as usize % self.number_of_solutions()
    }

    pub fn replace_subpopulation(&mut self, mut population: Population) {
        self.individuals
            .drain(0..(std::cmp::min(population.number_of_solutions(), self.individuals.len())));
        self.individuals.append(&mut population.individuals);
    }

    pub fn random_individual(&self) -> &VecIndividual {
        self.individuals
            .get(self.random_individual_index())
            .expect("random indvidual index was out of bounds")
    }

    pub fn individual_with_highest_fitness<'a>(
        problem: &dyn Problem,
        individuals: &'a [VecIndividual],
    ) -> (&'a VecIndividual, Fitness) {
        let solution = individuals
            .iter()
            .max_by(|solution_a, solution_b| {
                let eval_a = problem
                    .eval(solution_a)
                    .expect("VecIndividual should be valid for the problem");
                let eval_b = problem
                    .eval(solution_b)
                    .expect("VecIndividual should be valid for the problem");

                eval_a
                    .partial_cmp(&eval_b)
                    .expect("VecIndividual evaluations should be comparable")
            })
            .unwrap();

        (solution, problem.eval(solution).unwrap())
    }

    pub fn highest_fitness(&self, problem: &dyn Problem) -> (&VecIndividual, Fitness) {
        Population::individual_with_highest_fitness(problem, self.solutions())

        // let solution = self
        //     .solutions()
        //     .iter()
        //     .max_by(|solution_a, solution_b| {
        //         let eval_a = problem
        //             .eval(solution_a)
        //             .expect("VecIndividual should be valid for the problem");
        //         let eval_b = problem
        //             .eval(solution_b)
        //             .expect("VecIndividual should be valid for the problem");
        //
        //         eval_a
        //             .partial_cmp(&eval_b)
        //             .expect("VecIndividual evaluations should be comparable")
        //     })
        //     .unwrap();
        //
        // (solution, problem.eval(solution).unwrap())
    }

    pub fn lowest_fitness(&self, problem: &dyn Problem) -> (&VecIndividual, Fitness) {
        let solution = self
            .solutions()
            .iter()
            .min_by(|solution_a, solution_b| {
                let eval_a = problem
                    .eval(solution_a)
                    .expect("VecIndividual should be valid for the problem");
                let eval_b = problem
                    .eval(solution_b)
                    .expect("VecIndividual should be valid for the problem");

                eval_a
                    .partial_cmp(&eval_b)
                    .expect("VecIndividual evaluations should be comparable")
            })
            .unwrap();

        (solution, problem.eval(solution).unwrap())
    }

    pub fn average_fitness(&self, problem: &dyn Problem) -> Fitness {
        let solution: Fitness = self
            .solutions()
            .iter()
            .map(|sol| problem.eval(sol).unwrap())
            .sum::<Fitness>();

        solution / self.number_of_solutions() as f32
    }
}
