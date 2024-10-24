use crate::individual::VecIndividual;

pub trait NeighborOperator {
    fn get_neighborhood(&self, individual: &VecIndividual) -> Vec<VecIndividual>;
    fn name(&self) -> String;
}

pub struct SwapNeighborhoodOperator {
    neighborhood_size: u16,
}
impl SwapNeighborhoodOperator {
    pub fn new(neighborhood_size: u16) -> Self {
        SwapNeighborhoodOperator { neighborhood_size }
    }
}

impl NeighborOperator for SwapNeighborhoodOperator {
    fn name(&self) -> String {
        format!("swap ({})", self.neighborhood_size)
    }
    fn get_neighborhood(&self, individual: &VecIndividual) -> Vec<VecIndividual> {
        (0..self.neighborhood_size)
            .map(|_| {
                let start_index = individual.random_gene_index();
                let end_index = individual.random_gene_index();

                let mut neighbor = individual.clone();
                neighbor.genes_mut().swap(start_index, end_index);
                neighbor
            })
            .collect::<Vec<VecIndividual>>()
    }
}

pub struct InverseNeighborhoodOperator {
    neighborhood_size: u16,
}

impl InverseNeighborhoodOperator {
    pub fn new(neighborhood_size: u16) -> Self {
        InverseNeighborhoodOperator { neighborhood_size }
    }
}

impl NeighborOperator for InverseNeighborhoodOperator {
    fn name(&self) -> String {
        format!("inverse ({})", self.neighborhood_size)
    }

    fn get_neighborhood(&self, individual: &VecIndividual) -> Vec<VecIndividual> {
        (0..self.neighborhood_size)
            .map(|_| {
                let (start_index, end_index) = individual.random_gene_range_indexes();
                let mut neighbor = individual.clone();
                for index in start_index..(start_index + end_index) / 2 {
                    neighbor.genes_mut().swap(index, end_index - index);
                }
                neighbor
            })
            .collect::<Vec<VecIndividual>>()
    }
}
