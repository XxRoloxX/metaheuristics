use std::collections::HashMap;

use anyhow::{anyhow, Result};

use crate::individual::{Gene, VecIndividual};
mod tests;

pub trait SingleChildCrossoverOperator {
    fn crossover(
        &self,
        individual_a: &VecIndividual,
        individual_b: &VecIndividual,
    ) -> VecIndividual;
    fn name(&self) -> String;
}

pub trait TwoChildrenCrossoverOperator {
    fn crossover(
        &self,
        individual_a: &VecIndividual,
        individual_b: &VecIndividual,
    ) -> Result<(VecIndividual, VecIndividual)>;

    fn name(&self) -> String;
}

pub enum CrossoverOperator {
    SingleChildCrossoverOperator(Box<dyn SingleChildCrossoverOperator>),
    TwoChildrenCrossoverOperator(Box<dyn TwoChildrenCrossoverOperator>),
}

impl CrossoverOperator {
    pub fn name(&self) -> String {
        match self {
            Self::SingleChildCrossoverOperator(operator) => operator.name(),
            Self::TwoChildrenCrossoverOperator(operator) => operator.name(),
        }
    }
}

pub struct OrderedCrossover {}

impl SingleChildCrossoverOperator for OrderedCrossover {
    fn name(&self) -> String {
        String::from("ordered")
    }
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

pub struct PartiallyMappedCrossover {}

impl PartiallyMappedCrossover {
    fn build_translation_map(
        &self,
        parent_a: &[Gene],
        parent_b: &[Gene],
        translation_a: &mut HashMap<Gene, Vec<Gene>>,
        translation_b: &mut HashMap<Gene, Vec<Gene>>,
    ) {
        for (gene_a, gene_b) in parent_a.iter().zip(parent_b) {
            match translation_a.get_mut(gene_a) {
                Some(translated_genes) => translated_genes.push(*gene_b),
                None => {
                    translation_a.insert(*gene_a, vec![*gene_b]);
                }
            };

            match translation_b.get_mut(gene_b) {
                Some(translated_genes) => translated_genes.push(*gene_a),
                None => {
                    translation_b.insert(*gene_b, vec![*gene_a]);
                }
            };
        }
    }

    fn flatten_repeated_genes(&self, translations: &mut HashMap<Gene, Vec<Gene>>) -> Result<()> {
        let duplicated_genes = translations
            .iter()
            .filter(|(_, genes)| genes.len() > 1)
            .map(|(_, genes)| {
                if genes.len() > 2 {
                    Err(anyhow!("More than two genes!"))
                } else {
                    Ok((genes[0], genes[1]))
                }
            })
            .collect::<Result<Vec<(Gene, Gene)>>>()?;

        for (gene_a, gene_b) in duplicated_genes {
            translations.insert(gene_a, vec![gene_b]);
            translations.insert(gene_b, vec![gene_a]);
        }

        Ok(())
    }
    fn get_translated_gene(&self, translation_map: &HashMap<Gene, Vec<Gene>>, gene: Gene) -> Gene {
        match translation_map.get(&gene) {
            None => gene,
            Some(translation) => self.get_translated_gene(translation_map, translation[0]),
        }
    }
    fn produce_offspring(
        &self,
        translation_map: &mut HashMap<Gene, Vec<Gene>>,
        parent: &[Gene],
        middle_section_start_index: usize,
        middle_section_to_replace: Vec<Gene>,
    ) -> Result<Vec<Gene>> {
        let mut offspring: Vec<Gene> = Vec::new();

        for (index, parent_gene) in parent.iter().enumerate() {
            if (middle_section_start_index
                ..middle_section_start_index + middle_section_to_replace.len())
                .contains(&index)
            {
                offspring.push(middle_section_to_replace[index - middle_section_start_index])
            } else {
                offspring.push(self.get_translated_gene(translation_map, *parent_gene));
            }
        }

        Ok(offspring)
    }
}

impl TwoChildrenCrossoverOperator for PartiallyMappedCrossover {
    fn name(&self) -> String {
        String::from("partially-mapped")
    }
    fn crossover(
        &self,
        individual_a: &VecIndividual,
        individual_b: &VecIndividual,
    ) -> Result<(VecIndividual, VecIndividual)> {
        let (start_index, end_index) = individual_a.random_gene_range_indexes();

        let mut translation_map_a: HashMap<Gene, Vec<Gene>> = HashMap::new();
        let mut translation_map_b: HashMap<Gene, Vec<Gene>> = HashMap::new();
        let middle_section_a = individual_a.get_genes_from_range(start_index, end_index);
        let middle_section_b = individual_b.get_genes_from_range(start_index, end_index);

        self.build_translation_map(
            &middle_section_a,
            &middle_section_b,
            &mut translation_map_a,
            &mut translation_map_b,
        );
        if let Err(err) = self.flatten_repeated_genes(&mut translation_map_a) {
            println!("{}", err)
        }

        if let Err(err) = self.flatten_repeated_genes(&mut translation_map_b) {
            println!("{}", err)
        }

        let offspring_a = self.produce_offspring(
            &mut translation_map_b,
            individual_a.genes(),
            start_index,
            middle_section_b,
        )?;

        let offspring_b = self.produce_offspring(
            &mut translation_map_a,
            individual_b.genes(),
            start_index,
            middle_section_a,
        )?;

        Ok((
            VecIndividual::from(offspring_a),
            VecIndividual::from(offspring_b),
        ))
    }
}
