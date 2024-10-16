use crate::individual::VecIndividual;

pub trait Mutation {
    fn mutate(&self, individual: &mut VecIndividual);
}

pub struct SwapMutation {}

impl Mutation for SwapMutation {
    fn mutate(&self, individual: &mut VecIndividual) {
        let start_index = individual.random_gene_index();
        let end_index = individual.random_gene_index();

        individual.genes_mut().swap(start_index, end_index);
    }
}
