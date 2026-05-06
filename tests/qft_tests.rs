use mini_q::{Operation, QuantumCircuit, QuantumError};
use num_complex::Complex64;

const EPS: f64 = 1e-10;

fn assert_complex_close(actual: Complex64, expected: Complex64) {
    assert!(
        (actual - expected).norm() < EPS,
        "actual {actual:?}, expected {expected:?}"
    );
}

fn assert_states_close(actual: &[Complex64], expected: &[Complex64]) {
    assert_eq!(actual.len(), expected.len());
    for (&actual, &expected) in actual.iter().zip(expected) {
        assert_complex_close(actual, expected);
    }
}

#[test]
fn qft_on_one_qubit_matches_hadamard() {
    let mut qc = QuantumCircuit::new(1).unwrap();
    qc.qft(&[0]).unwrap();

    let expected = 1.0 / 2.0_f64.sqrt();
    assert_complex_close(qc.state()[0], Complex64::new(expected, 0.0));
    assert_complex_close(qc.state()[1], Complex64::new(expected, 0.0));
}

#[test]
fn qft_on_two_qubit_basis_one_has_expected_phases() {
    let mut qc = QuantumCircuit::new(2).unwrap();
    qc.x(0).unwrap();
    qc.qft(&[0, 1]).unwrap();

    assert_complex_close(qc.state()[0], Complex64::new(0.5, 0.0));
    assert_complex_close(qc.state()[1], Complex64::new(0.0, 0.5));
    assert_complex_close(qc.state()[2], Complex64::new(-0.5, 0.0));
    assert_complex_close(qc.state()[3], Complex64::new(0.0, -0.5));
}

#[test]
fn inverse_qft_reverses_qft_on_full_register() {
    let mut qc = QuantumCircuit::new(3).unwrap();
    qc.h(0).unwrap();
    qc.rx(1, 0.37).unwrap();
    qc.ry(2, -0.91).unwrap();
    qc.controlled_phase(0, 2, 0.42).unwrap();
    let original = qc.state().to_vec();

    qc.qft(&[0, 1, 2]).unwrap();
    qc.inverse_qft(&[0, 1, 2]).unwrap();

    assert_states_close(qc.state(), &original);
    qc.assert_normalized(EPS).unwrap();
}

#[test]
fn inverse_qft_reverses_qft_on_subset() {
    let mut qc = QuantumCircuit::new(3).unwrap();
    qc.h(0).unwrap();
    qc.x(1).unwrap();
    qc.ry(2, 0.6).unwrap();
    let original = qc.state().to_vec();

    qc.qft(&[0, 1]).unwrap();
    qc.inverse_qft(&[0, 1]).unwrap();

    assert_states_close(qc.state(), &original);
}

#[test]
fn qft_operations_are_recorded_as_high_level_steps() {
    let mut qc = QuantumCircuit::new(2).unwrap();

    qc.qft(&[0, 1]).unwrap();
    qc.inverse_qft(&[0, 1]).unwrap();

    assert_eq!(
        qc.operations(),
        &[
            Operation::Qft { qubits: vec![0, 1] },
            Operation::InverseQft { qubits: vec![0, 1] },
        ]
    );
}

#[test]
fn qft_rejects_empty_qubit_list() {
    let mut qc = QuantumCircuit::new(2).unwrap();
    assert_eq!(qc.qft(&[]), Err(QuantumError::InvalidNumQubits));
}

#[test]
fn qft_rejects_duplicate_qubits() {
    let mut qc = QuantumCircuit::new(2).unwrap();
    assert_eq!(
        qc.qft(&[0, 0]),
        Err(QuantumError::DuplicateQubit { q1: 0, q2: 0 })
    );
}
