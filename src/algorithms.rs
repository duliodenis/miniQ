use crate::{QuantumCircuit, QuantumError};
use std::f64::consts::PI;

pub fn phase_estimation_for_phase(
    num_counting_qubits: usize,
    phase: f64,
) -> Result<QuantumCircuit, QuantumError> {
    if num_counting_qubits == 0 || !phase.is_finite() {
        return Err(QuantumError::InvalidNumQubits);
    }

    let target = num_counting_qubits;
    let counting_qubits: Vec<usize> = (0..num_counting_qubits).collect();
    let mut circuit = QuantumCircuit::new(num_counting_qubits + 1)?;

    circuit.x(target)?;

    for &qubit in &counting_qubits {
        circuit.h(qubit)?;
    }

    for (power, &control) in counting_qubits.iter().enumerate() {
        let theta = 2.0 * PI * phase * (1usize << power) as f64;
        circuit.controlled_phase(control, target, theta)?;
    }

    circuit.inverse_qft(&counting_qubits)?;
    Ok(circuit)
}
