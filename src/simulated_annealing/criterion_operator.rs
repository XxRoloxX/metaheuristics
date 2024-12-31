use rand::{thread_rng, Rng};

use crate::individual::Fitness;

use super::algorithm::Temperature;

pub trait CriterionOperator {
    fn criterion(&self, fitness_diff: Fitness, temperature: Temperature) -> bool;
    fn name(&self) -> String;
}

#[derive(Clone)]
pub struct BoltzmanProbabilityCriterionOperator {}

impl CriterionOperator for BoltzmanProbabilityCriterionOperator {
    fn criterion(&self, fitness_diff: Fitness, temperature: Temperature) -> bool {
        let rand: f32 = thread_rng().gen_range(0f32..1f32);
        let fitness_diff_by_temperature = -(fitness_diff * 0.01) / temperature;
        println!("Diff {:.40}", fitness_diff);
        println!("Temp {:.40}", temperature);
        println!("By {:.40}", fitness_diff_by_temperature.exp());
        rand < fitness_diff_by_temperature.exp()
    }

    fn name(&self) -> String {
        String::from("Boltzman criterion")
    }
}
