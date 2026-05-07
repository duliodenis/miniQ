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
fn modular_exponentiation_control_01_applies_base_to_first_power() {
    let mut qc = QuantumCircuit::from_basis_state(6, 0b01_0011).unwrap();
    qc.modular_exponentiation(&[4, 5], &[0, 1, 2, 3], 7, 15)
        .unwrap();

    assert!((probability(&qc, "010110") - 1.0).abs() < EPS);
}

#[test]
fn modular_exponentiation_control_10_applies_base_to_second_power() {
    let mut qc = QuantumCircuit::from_basis_state(6, 0b10_0011).unwrap();
    qc.modular_exponentiation(&[4, 5], &[0, 1, 2, 3], 7, 15)
        .unwrap();

    assert!((probability(&qc, "101100") - 1.0).abs() < EPS);
}

#[test]
fn modular_exponentiation_control_11_composes_to_third_power() {
    let mut qc = QuantumCircuit::from_basis_state(6, 0b11_0011).unwrap();
    qc.modular_exponentiation(&[4, 5], &[0, 1, 2, 3], 7, 15)
        .unwrap();

    assert!((probability(&qc, "111001") - 1.0).abs() < EPS);
}

#[test]
fn modular_exponentiation_all_controls_zero_leaves_target_fixed() {
    let mut qc = QuantumCircuit::from_basis_state(6, 0b00_0011).unwrap();
    qc.modular_exponentiation(&[4, 5], &[0, 1, 2, 3], 7, 15)
        .unwrap();

    assert!((probability(&qc, "000011") - 1.0).abs() < EPS);
}

#[test]
fn modular_exponentiation_rejects_control_target_overlap() {
    let mut qc = QuantumCircuit::new(5).unwrap();

    assert_eq!(
        qc.modular_exponentiation(&[3, 4], &[0, 1, 2, 3], 7, 15),
        Err(QuantumError::DuplicateQubit { q1: 3, q2: 3 })
    );
}

#[test]
fn modular_exponentiation_rejects_duplicate_controls() {
    let mut qc = QuantumCircuit::new(6).unwrap();

    assert_eq!(
        qc.modular_exponentiation(&[4, 4], &[0, 1, 2, 3], 7, 15),
        Err(QuantumError::DuplicateQubit { q1: 4, q2: 4 })
    );
}

#[test]
fn modular_exponentiation_records_only_high_level_operation() {
    let mut qc = QuantumCircuit::from_basis_state(6, 0b11_0011).unwrap();
    qc.modular_exponentiation(&[4, 5], &[0, 1, 2, 3], 7, 15)
        .unwrap();

    assert_eq!(
        qc.operations(),
        &[Operation::ModularExponentiation {
            controls: vec![4, 5],
            targets: vec![0, 1, 2, 3],
            base: 7,
            modulus: 15,
        }]
    );
}
