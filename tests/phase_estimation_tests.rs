use mini_q::{algorithms::phase_estimation_for_phase, QuantumError};

#[test]
fn phase_estimation_recovers_exact_one_quarter_phase() {
    let qc = phase_estimation_for_phase(3, 0.25).unwrap();

    let probabilities = qc.probabilities(1e-10);
    assert_eq!(probabilities.len(), 1);
    assert_eq!(probabilities[0].0, "1010");
    assert!((probabilities[0].1 - 1.0).abs() < 1e-10);
}

#[test]
fn phase_estimation_rejects_zero_counting_qubits() {
    assert!(matches!(
        phase_estimation_for_phase(0, 0.25),
        Err(QuantumError::InvalidNumQubits)
    ));
}

#[test]
fn phase_estimation_rejects_non_finite_phase() {
    assert!(matches!(
        phase_estimation_for_phase(3, f64::NAN),
        Err(QuantumError::InvalidNumQubits)
    ));
}
