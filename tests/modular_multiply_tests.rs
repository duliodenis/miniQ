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
fn controlled_modular_multiply_maps_values_below_modulus() {
    let mut qc = QuantumCircuit::from_basis_state(5, 0b1_0111).unwrap();
    qc.controlled_modular_multiply(4, &[0, 1, 2, 3], 2, 15)
        .unwrap();

    assert!((probability(&qc, "11110") - 1.0).abs() < EPS);
}

#[test]
fn controlled_modular_multiply_leaves_out_of_range_values_fixed() {
    let mut qc = QuantumCircuit::from_basis_state(5, 0b1_1111).unwrap();
    qc.controlled_modular_multiply(4, &[0, 1, 2, 3], 2, 15)
        .unwrap();

    assert!((probability(&qc, "11111") - 1.0).abs() < EPS);
}

#[test]
fn controlled_modular_multiply_does_nothing_when_control_is_zero() {
    let mut qc = QuantumCircuit::from_basis_state(5, 0b0_0111).unwrap();
    qc.controlled_modular_multiply(4, &[0, 1, 2, 3], 2, 15)
        .unwrap();

    assert!((probability(&qc, "00111") - 1.0).abs() < EPS);
}

#[test]
fn controlled_modular_multiply_rejects_non_coprime_multiplier() {
    let mut qc = QuantumCircuit::new(5).unwrap();

    assert_eq!(
        qc.controlled_modular_multiply(4, &[0, 1, 2, 3], 5, 15),
        Err(QuantumError::InvalidArithmeticInput)
    );
}

#[test]
fn controlled_modular_multiply_rejects_too_large_modulus() {
    let mut qc = QuantumCircuit::new(4).unwrap();

    assert_eq!(
        qc.controlled_modular_multiply(3, &[0, 1, 2], 2, 15),
        Err(QuantumError::InvalidArithmeticInput)
    );
}

#[test]
fn controlled_modular_multiply_rejects_control_target_overlap() {
    let mut qc = QuantumCircuit::new(4).unwrap();

    assert_eq!(
        qc.controlled_modular_multiply(3, &[0, 1, 3], 2, 7),
        Err(QuantumError::DuplicateQubit { q1: 3, q2: 3 })
    );
}

#[test]
fn controlled_modular_multiply_records_operation() {
    let mut qc = QuantumCircuit::from_basis_state(5, 0b1_0111).unwrap();
    qc.controlled_modular_multiply(4, &[0, 1, 2, 3], 2, 15)
        .unwrap();

    assert_eq!(
        qc.operations().last(),
        Some(&Operation::ControlledModularMultiply {
            control: 4,
            targets: vec![0, 1, 2, 3],
            multiplier: 2,
            modulus: 15,
        })
    );
}

#[test]
fn controlled_modular_multiply_power_uses_modular_power_as_multiplier() {
    let mut qc = QuantumCircuit::from_basis_state(5, 0b1_0011).unwrap();
    qc.controlled_modular_multiply_power(4, &[0, 1, 2, 3], 7, 2, 15)
        .unwrap();

    assert!((probability(&qc, "11100") - 1.0).abs() < EPS);
}

#[test]
fn controlled_modular_multiply_power_does_nothing_when_control_is_zero() {
    let mut qc = QuantumCircuit::from_basis_state(5, 0b0_0011).unwrap();
    qc.controlled_modular_multiply_power(4, &[0, 1, 2, 3], 7, 2, 15)
        .unwrap();

    assert!((probability(&qc, "00011") - 1.0).abs() < EPS);
}

#[test]
fn controlled_modular_multiply_power_records_operation() {
    let mut qc = QuantumCircuit::from_basis_state(5, 0b1_0011).unwrap();
    qc.controlled_modular_multiply_power(4, &[0, 1, 2, 3], 7, 2, 15)
        .unwrap();

    assert_eq!(
        qc.operations().last(),
        Some(&Operation::ControlledModularMultiplyPower {
            control: 4,
            targets: vec![0, 1, 2, 3],
            base: 7,
            power: 2,
            modulus: 15,
        })
    );
}

#[test]
fn controlled_modular_multiply_power_rejects_invalid_modulus() {
    let mut qc = QuantumCircuit::new(5).unwrap();

    assert_eq!(
        qc.controlled_modular_multiply_power(4, &[0, 1, 2, 3], 7, 2, 0),
        Err(QuantumError::InvalidArithmeticInput)
    );
}
