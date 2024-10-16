use crate::individual::{Fitness, VecIndividual};
use anyhow::Result;

pub trait Problem {
    fn eval(&self, individual: &VecIndividual) -> Result<Fitness>;
    fn random_individual(&self) -> VecIndividual;
    fn serialize_indiviual(&self, individual: &VecIndividual) -> String;
}
