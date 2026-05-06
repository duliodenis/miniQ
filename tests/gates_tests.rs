use mini_q::QuantumCircuit;

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
    assert!((probability(&qc, "000") - 1.0).abs() < EPS);
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
