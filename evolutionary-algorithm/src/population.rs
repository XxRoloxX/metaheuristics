use crate::{problem::Problem, solution::Solution};

pub struct Population {
    solutions: Vec<Solution>,
}

impl Population {
    pub fn new(solutions: Vec<Solution>) -> Self {
        Population { solutions }
    }
    pub fn solutions(&self) -> &Vec<Solution> {
        &self.solutions
    }

    pub fn into_solutions(self) -> Vec<Solution> {
        self.solutions
    }

    pub fn best_solution(problem: &dyn Problem, solutions: Vec<&Solution>) -> Solution {
        solutions
            .iter()
            .min_by(|solution_a, solution_b| {
                let eval_a = problem
                    .eval(solution_a)
                    .expect("Solution should be valid for the problem");
                let eval_b = problem
                    .eval(solution_b)
                    .expect("Solution should be valid for the problem");

                eval_a
                    .partial_cmp(&eval_b)
                    .expect("Solution evaluations should be comparable")
            })
            .unwrap()
            .to_vec()
    }

    pub fn best(&self, problem: &dyn Problem) -> Solution {
        self.solutions
            .iter()
            .min_by(|solution_a, solution_b| {
                let eval_a = problem
                    .eval(solution_a)
                    .expect("Solution should be valid for the problem");
                let eval_b = problem
                    .eval(solution_b)
                    .expect("Solution should be valid for the problem");

                eval_a
                    .partial_cmp(&eval_b)
                    .expect("Solution evaluations should be comparable")
            })
            .unwrap()
            .clone()
    }

    pub fn worst(&self, problem: Box<dyn Problem>) -> Solution {
        self.solutions
            .iter()
            .min_by(|solution_a, solution_b| {
                let eval_a = problem
                    .eval(solution_a)
                    .expect("Solution should be valid for the problem");
                let eval_b = problem
                    .eval(solution_b)
                    .expect("Solution should be valid for the problem");

                eval_a
                    .partial_cmp(&eval_b)
                    .expect("Solution evaluations should be comparable")
            })
            .unwrap()
            .clone()
    }
}
