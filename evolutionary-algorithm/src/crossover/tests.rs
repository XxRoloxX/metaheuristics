use super::{OrderedCrossover, SingleChildCrossoverOperator};
use crate::individual::VecIndividual;

#[test]
fn ordered_crossover() {
    let parent_a = VecIndividual::from(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    let parent_b = VecIndividual::from(vec![5, 7, 4, 9, 1, 3, 6, 2, 8]);

    let operator = OrderedCrossover {};

    let child = operator.crossover(&parent_a, &parent_b);

    let count = child
        .genes()
        .iter()
        .filter(|gene| parent_a.genes().contains(gene))
        .count();

    assert!(count == parent_a.genes().len());
}
