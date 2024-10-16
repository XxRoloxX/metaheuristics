use crate::individual::{Gene, VecIndividual};
mod tests;

pub trait SingleChildCrossoverOperator {
    fn crossover(
        &self,
        individual_a: &VecIndividual,
        individual_b: &VecIndividual,
    ) -> VecIndividual;
}

pub struct OrderedCrossover {}

impl SingleChildCrossoverOperator for OrderedCrossover {
    fn crossover(
        &self,
        individual_a: &VecIndividual,
        individual_b: &VecIndividual,
    ) -> VecIndividual {
        let (start_index, end_index) = individual_a.random_gene_range_indexes();
        // let (start_index, end_index) = (2, 5);
        let substring = individual_a.get_genes_from_range(start_index, end_index);

        // Remove the first substring from the second individual
        let mut second_without_substring = individual_b
            .genes()
            .iter()
            .copied()
            .filter(|gene| !substring.contains(gene))
            .collect::<Vec<Gene>>();

        // Insert first substring into the same position on the second individual
        second_without_substring.splice(start_index..start_index, substring);

        VecIndividual::from(second_without_substring)
    }
}
