use mini_q::{Operation, QuantumCircuit, QuantumError};

const EPS: f64 = 1e-10;

fn probability(qc: &QuantumCircuit, label: &str) -> f64 {
    qc.probabilities(0.0)
        .into_iter()
        .find(|(basis, _)| basis.as_str() == label)
        .map(|(_, probability)| probability)
        .unwrap_or(0.0)
}

#[test]
fn controlled_basis_permutation_can_apply_controlled_x() {
    let mut qc = QuantumCircuit::from_basis_state(2, 0b01).unwrap();
    qc.apply_controlled_basis_permutation(0, &[1], &[1, 0])
        .unwrap();

    assert!((probability(&qc, "11") - 1.0).abs() < EPS);
}

#[test]
fn controlled_basis_permutation_does_nothing_when_control_is_zero() {
    let mut qc = QuantumCircuit::from_basis_state(2, 0b00).unwrap();
    qc.apply_controlled_basis_permutation(0, &[1], &[1, 0])
        .unwrap();

    assert!((probability(&qc, "00") - 1.0).abs() < EPS);
}

#[test]
fn controlled_basis_permutation_supports_two_target_registers() {
    let mut qc = QuantumCircuit::from_basis_state(3, 0b011).unwrap();
    qc.apply_controlled_basis_permutation(0, &[1, 2], &[0, 2, 1, 3])
        .unwrap();

    assert!((probability(&qc, "101") - 1.0).abs() < EPS);
}

#[test]
fn controlled_basis_permutation_rejects_control_target_overlap() {
    let mut qc = QuantumCircuit::new(2).unwrap();

    assert_eq!(
        qc.apply_controlled_basis_permutation(0, &[0], &[1, 0]),
        Err(QuantumError::DuplicateQubit { q1: 0, q2: 0 })
    );
}

#[test]
fn controlled_basis_permutation_rejects_invalid_length() {
    let mut qc = QuantumCircuit::new(3).unwrap();

    assert_eq!(
        qc.apply_controlled_basis_permutation(0, &[1, 2], &[0, 1]),
        Err(QuantumError::InvalidPermutation)
    );
}

#[test]
fn controlled_basis_permutation_rejects_non_bijection() {
    let mut qc = QuantumCircuit::new(2).unwrap();

    assert_eq!(
        qc.apply_controlled_basis_permutation(0, &[1], &[1, 1]),
        Err(QuantumError::InvalidPermutation)
    );
}

#[test]
fn controlled_basis_permutation_records_operation() {
    let mut qc = QuantumCircuit::from_basis_state(2, 0b01).unwrap();
    qc.apply_controlled_basis_permutation(0, &[1], &[1, 0])
        .unwrap();

    assert_eq!(
        qc.operations().last(),
        Some(&Operation::ControlledBasisPermutation {
            control: 0,
            targets: vec![1],
            permutation: vec![1, 0],
        })
    );
}
