use mini_q::{Operation, QuantumCircuit};

#[test]
fn measurement_collapses_and_remains_normalized() {
    let mut qc = QuantumCircuit::new(1).unwrap();
    qc.h(0).unwrap();

    let result = qc.measure(0).unwrap();
    qc.assert_normalized(1e-10).unwrap();

    let probabilities = qc.probabilities(0.0);
    let measured_label = result.to_string();
    assert_eq!(
        probabilities
            .into_iter()
            .find(|(label, _)| label.as_str() == measured_label)
            .map(|(_, probability)| probability),
        Some(1.0)
    );
    assert_eq!(
        qc.operations().last(),
        Some(&Operation::Measure { target: 0, result })
    );
}

#[test]
fn measure_all_collapses_to_returned_basis_state() {
    let mut qc = QuantumCircuit::new(2).unwrap();
    qc.h(0).unwrap();
    qc.cnot(0, 1).unwrap();

    let result = qc.measure_all().unwrap();
    qc.assert_normalized(1e-10).unwrap();
    assert!(result == "00" || result == "11");
    assert_eq!(qc.probabilities(0.0), vec![(result, 1.0)]);
}
