use mini_q::{utils::basis_label, QuantumCircuit};

#[test]
fn basis_labels_are_displayed_most_significant_bit_first() {
    assert_eq!(basis_label(5, 3), "101");
    assert_eq!(basis_label(1, 3), "001");
}

#[test]
fn probabilities_are_reported_for_nonzero_states() {
    let mut qc = QuantumCircuit::new(2).unwrap();
    qc.h(0).unwrap();

    let probabilities = qc.probabilities(1e-12);
    assert_eq!(probabilities.len(), 2);
    assert_eq!(probabilities[0].0, "00");
    assert_eq!(probabilities[1].0, "01");
    assert!((probabilities[0].1 - 0.5).abs() < 1e-10);
    assert!((probabilities[1].1 - 0.5).abs() < 1e-10);
}
