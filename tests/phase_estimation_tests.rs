use mini_q::QuantumCircuit;
use std::f64::consts::PI;

#[test]
fn phase_estimation_recovers_exact_one_quarter_phase() {
    let counting_qubits = [0, 1, 2];
    let target = 3;
    let phase = 0.25;

    let mut qc = QuantumCircuit::new(4).unwrap();
    qc.x(target).unwrap();

    for &qubit in &counting_qubits {
        qc.h(qubit).unwrap();
    }

    for (power, &control) in counting_qubits.iter().enumerate() {
        let theta = 2.0 * PI * phase * (1usize << power) as f64;
        qc.controlled_phase(control, target, theta).unwrap();
    }

    qc.inverse_qft(&counting_qubits).unwrap();

    let probabilities = qc.probabilities(1e-10);
    assert_eq!(probabilities.len(), 1);
    assert_eq!(probabilities[0].0, "1010");
    assert!((probabilities[0].1 - 1.0).abs() < 1e-10);
}
