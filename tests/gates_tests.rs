use mini_q::{Operation, QuantumCircuit, QuantumError};
use num_complex::Complex64;
use std::f64::consts::PI;

const EPS: f64 = 1e-10;

fn probability(qc: &QuantumCircuit, label: &str) -> f64 {
    qc.probabilities(0.0)
        .into_iter()
        .find(|(basis, _)| basis.as_str() == label)
        .map(|(_, probability)| probability)
        .unwrap_or(0.0)
}

#[test]
fn initial_state_is_zero_basis_state() {
    let qc = QuantumCircuit::new(3).unwrap();
    assert_eq!(qc.num_qubits(), 3);
    assert!((probability(&qc, "000") - 1.0).abs() < EPS);
}

#[test]
fn from_basis_state_initializes_requested_basis_state() {
    let qc = QuantumCircuit::from_basis_state(3, 5).unwrap();

    assert_eq!(qc.num_qubits(), 3);
    assert!(qc.operations().is_empty());
    assert!((probability(&qc, "101") - 1.0).abs() < EPS);
}

#[test]
fn from_basis_state_rejects_out_of_range_basis_index() {
    assert!(matches!(
        QuantumCircuit::from_basis_state(3, 8),
        Err(QuantumError::InvalidQubit {
            index: 8,
            num_qubits: 3
        })
    ));
}

#[test]
fn x_gate_flips_single_qubit() {
    let mut qc = QuantumCircuit::new(1).unwrap();
    qc.x(0).unwrap();
    assert!((probability(&qc, "1") - 1.0).abs() < EPS);
}

#[test]
fn hadamard_creates_equal_superposition() {
    let mut qc = QuantumCircuit::new(1).unwrap();
    qc.h(0).unwrap();
    assert!((probability(&qc, "0") - 0.5).abs() < EPS);
    assert!((probability(&qc, "1") - 0.5).abs() < EPS);
}

#[test]
fn swap_moves_basis_state_between_qubits() {
    let mut qc = QuantumCircuit::new(2).unwrap();
    qc.x(0).unwrap();
    qc.swap(0, 1).unwrap();
    assert!((probability(&qc, "10") - 1.0).abs() < EPS);
}

#[test]
fn invalid_qubit_returns_error() {
    let mut qc = QuantumCircuit::new(2).unwrap();
    assert!(qc.h(99).is_err());
}

#[test]
fn invalid_cnot_rejects_duplicate_qubits() {
    let mut qc = QuantumCircuit::new(2).unwrap();
    assert!(qc.cnot(0, 0).is_err());
}

#[test]
fn rx_pi_flips_probability_to_one_with_phase() {
    let mut qc = QuantumCircuit::new(1).unwrap();
    qc.rx(0, PI).unwrap();

    assert!(probability(&qc, "0") < EPS);
    assert!((probability(&qc, "1") - 1.0).abs() < EPS);
    assert!(qc.state()[1].re.abs() < EPS);
    assert!((qc.state()[1].im + 1.0).abs() < EPS);
}

#[test]
fn ry_pi_flips_probability_to_one() {
    let mut qc = QuantumCircuit::new(1).unwrap();
    qc.ry(0, PI).unwrap();

    assert!(probability(&qc, "0") < EPS);
    assert!((probability(&qc, "1") - 1.0).abs() < EPS);
    assert!((qc.state()[1].re - 1.0).abs() < EPS);
    assert!(qc.state()[1].im.abs() < EPS);
}

#[test]
fn rz_pi_applies_phase_without_changing_probabilities() {
    let mut qc = QuantumCircuit::new(1).unwrap();
    qc.rz(0, PI).unwrap();

    assert!((probability(&qc, "0") - 1.0).abs() < EPS);
    assert!(qc.state()[0].re.abs() < EPS);
    assert!((qc.state()[0].im + 1.0).abs() < EPS);
}

#[test]
fn controlled_phase_only_phases_joint_one_state() {
    let mut qc = QuantumCircuit::new(2).unwrap();
    qc.x(0).unwrap();
    qc.x(1).unwrap();
    qc.controlled_phase(0, 1, PI / 2.0).unwrap();

    assert!((probability(&qc, "11") - 1.0).abs() < EPS);
    assert!(qc.state()[3].re.abs() < EPS);
    assert!((qc.state()[3].im - 1.0).abs() < EPS);
}

#[test]
fn apply_single_qubit_gate_rejects_non_finite_matrix() {
    let mut qc = QuantumCircuit::new(1).unwrap();
    let matrix = [
        [Complex64::new(f64::NAN, 0.0), Complex64::new(0.0, 0.0)],
        [Complex64::new(0.0, 0.0), Complex64::new(1.0, 0.0)],
    ];

    assert_eq!(
        qc.apply_single_qubit_gate(matrix, 0),
        Err(QuantumError::InvalidMatrix)
    );
}

#[test]
fn operation_history_tracks_gates_in_order() {
    let mut qc = QuantumCircuit::new(2).unwrap();
    qc.h(0).unwrap();
    qc.cnot(0, 1).unwrap();
    qc.rz(1, PI / 4.0).unwrap();
    qc.swap(0, 1).unwrap();

    assert_eq!(
        qc.operations(),
        &[
            Operation::H { target: 0 },
            Operation::CNot {
                control: 0,
                target: 1
            },
            Operation::Rz {
                target: 1,
                theta: PI / 4.0
            },
            Operation::Swap { q1: 0, q2: 1 },
        ]
    );
}
