use crate::{
    postprocessing::{factor_from_period, recover_period_from_phase},
    QuantumCircuit, QuantumError,
};
use std::f64::consts::PI;

/// Result from one stochastic toy Shor factor-15 attempt.
#[derive(Debug, Clone, PartialEq)]
pub struct ShorAttempt {
    /// Measured little-endian work-register value.
    pub work_value: usize,
    /// Measured little-endian counting-register value.
    pub counting_value: usize,
    /// Phase estimate `counting_value / 2^num_counting_qubits`.
    pub phase: f64,
    /// Recovered period when the phase sample is useful.
    pub period: Option<u64>,
    /// Nontrivial factors when period postprocessing succeeds.
    pub factors: Option<(u64, u64)>,
}

/// Build a tiny phase-estimation circuit for a known controlled-phase eigenvalue.
///
/// This helper is educational: it estimates a supplied phase rather than a
/// generic unitary.
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

/// Try to recover factors from a single phase sample.
///
/// A phase of `0` is treated as uninformative and returns `Ok(None)`.
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

/// Run one stochastic toy Shor-style attempt for factoring `15`.
///
/// The attempt prepares the order-finding circuit for `a = 7, N = 15`,
/// measures the work register, applies inverse QFT to the counting register,
/// measures the phase sample, and attempts classical postprocessing.
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
