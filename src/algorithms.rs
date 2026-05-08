use crate::{
    postprocessing::{factor_from_period, recover_period_from_phase},
    QuantumCircuit, QuantumError,
};
use std::f64::consts::PI;

#[derive(Debug, Clone, PartialEq)]
pub struct ShorAttempt {
    pub work_value: usize,
    pub counting_value: usize,
    pub phase: f64,
    pub period: Option<u64>,
    pub factors: Option<(u64, u64)>,
}

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

pub fn try_factor_from_phase_sample(
    phase: f64,
    a: u64,
    n: u64,
    max_period: u64,
) -> Result<Option<(u64, u64)>, QuantumError> {
    if !phase.is_finite() || phase.fract().abs() < 1e-12 {
        return Ok(None);
    }

    let Some(period) = recover_period_from_phase(phase, a, n, max_period)? else {
        return Ok(None);
    };

    factor_from_period(a, n, period)
}

pub fn shor_factor_15_attempt() -> Result<ShorAttempt, QuantumError> {
    let n = 15;
    let a = 7;
    let counting = [4, 5, 6];
    let work = [0, 1, 2, 3];
    let max_period = 32;

    let mut circuit = QuantumCircuit::from_basis_state(7, 1)?;
    for &control in &counting {
        circuit.h(control)?;
    }
    circuit.modular_exponentiation(&counting, &work, a, n)?;

    let work_value = circuit.measure_register(&work)?;
    circuit.inverse_qft(&counting)?;
    let counting_value = circuit.measure_register(&counting)?;
    let phase = counting_value as f64 / (1usize << counting.len()) as f64;
    let period = if phase.fract().abs() < 1e-12 {
        None
    } else {
        recover_period_from_phase(phase, a, n, max_period)?
    };
    let factors = try_factor_from_phase_sample(phase, a, n, max_period)?;

    Ok(ShorAttempt {
        work_value,
        counting_value,
        phase,
        period,
        factors,
    })
}
