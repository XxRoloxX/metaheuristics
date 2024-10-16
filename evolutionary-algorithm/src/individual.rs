use rand::RngCore;

pub type Gene = u16;
pub type Fitness = f32;

#[derive(Debug, Default)]
pub struct VecIndividual {
    genes: Vec<Gene>,
}

impl VecIndividual {
    pub fn genes(&self) -> &Vec<Gene> {
        &self.genes
    }
    pub fn genes_mut(&mut self) -> &mut Vec<Gene> {
        &mut self.genes
    }
    pub fn new() -> Self {
        VecIndividual { genes: Vec::new() }
    }

    pub fn add_gene(&mut self, gene: Gene) {
        self.genes.push(gene)
    }

    pub fn remove_gene(&mut self, gene: Gene) {
        if let Some(index) = self.genes.iter().position(|curr| *curr == gene) {
            self.genes.remove(index);
        };
    }

    pub fn random_gene(&self) -> Gene {
        let random_index: usize = rand::random();
        let selected_gene_index: usize = random_index % self.genes.len();
        self.genes[selected_gene_index]
    }

    pub fn random_gene_index(&self) -> usize {
        rand::random::<usize>() % self.number_of_genes()
    }

    pub fn number_of_genes(&self) -> usize {
        self.genes.len()
    }

    pub fn random_gene_range_indexes(&self) -> (usize, usize) {
        // Make sure that the range always has more than one element
        let mut rng = rand::thread_rng();

        let start_index: usize = rng.next_u32() as usize % (self.number_of_genes() - 1);
        let end_index: usize =
            start_index + 1 + rng.next_u32() as usize % (self.number_of_genes() - start_index - 1);

        (start_index, end_index)
    }

    // Get genes from index range (including start and end index)
    pub fn get_genes_from_range(&self, start_index: usize, end_index: usize) -> Vec<Gene> {
        let mut genes_range = Vec::new();
        genes_range.extend_from_slice(&self.genes[start_index..end_index + 1]);
        genes_range
    }
}

impl From<&Vec<Gene>> for VecIndividual {
    fn from(genes: &Vec<Gene>) -> Self {
        VecIndividual {
            genes: genes.clone(),
        }
    }
}

impl From<Vec<Gene>> for VecIndividual {
    fn from(genes: Vec<Gene>) -> Self {
        VecIndividual { genes }
    }
}

impl From<&VecIndividual> for VecIndividual {
    fn from(individual: &VecIndividual) -> Self {
        VecIndividual {
            genes: individual.genes().clone(),
        }
    }
}
