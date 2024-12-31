use crate::individual::VecIndividual;

#[test]
fn random_index_test() {
    let individual = VecIndividual::from(&vec![1, 2, 3, 4, 5, 6]);

    for _ in 0..100 {
        let (start, end) = individual.random_gene_range_indexes();
        assert!(start < end, "start index is smaller than end index");
        assert!(
            start < individual.number_of_genes() - 1,
            "start index is not the last one"
        );
        assert!(
            end < individual.number_of_genes(),
            "end index is smaller than length"
        );
    }
}
