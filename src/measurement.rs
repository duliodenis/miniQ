use crate::{utils::basis_label, Operation, QuantumCircuit, QuantumError};
use rand::Rng;

pub fn measure(circuit: &mut QuantumCircuit, target: usize) -> Result<u8, QuantumError> {
    circuit.validate_qubit(target)?;
    let mask = 1usize << target;
    let probability_one: f64 = circuit
        .state()
        .iter()
        .enumerate()
        .filter(|(index, _)| *index & mask != 0)
        .map(|(_, amplitude)| amplitude.norm_sqr())
        .sum();

    let draw = rand::thread_rng().gen::<f64>();
    let result = u8::from(draw < probability_one);
    let keep_one = result == 1;
    let keep_probability = if keep_one {
        probability_one
    } else {
        1.0 - probability_one
    };
    let scale = if keep_probability > 0.0 {
        1.0 / keep_probability.sqrt()
    } else {
        0.0
    };

    for (index, amplitude) in circuit.state_mut().iter_mut().enumerate() {
        if (index & mask != 0) == keep_one {
            *amplitude *= scale;
        } else {
            *amplitude = Default::default();
        }
    }

    circuit.push_operation(Operation::Measure { target, result });
    Ok(result)
}

pub fn measure_all(circuit: &mut QuantumCircuit) -> Result<String, QuantumError> {
    let draw = rand::thread_rng().gen::<f64>();
    let mut cumulative = 0.0;
    let mut measured_index = circuit.state().len() - 1;

    for (index, amplitude) in circuit.state().iter().enumerate() {
        cumulative += amplitude.norm_sqr();
        if draw <= cumulative {
            measured_index = index;
            break;
        }
    }

    for (index, amplitude) in circuit.state_mut().iter_mut().enumerate() {
        *amplitude = if index == measured_index {
            num_complex::Complex64::new(1.0, 0.0)
        } else {
            num_complex::Complex64::new(0.0, 0.0)
        };
    }

    let result = basis_label(measured_index, circuit.num_qubits());
    circuit.push_operation(Operation::MeasureAll {
        result: result.clone(),
    });
    Ok(result)
}

pub fn measure_register(
    circuit: &mut QuantumCircuit,
    qubits: &[usize],
) -> Result<usize, QuantumError> {
    validate_register(circuit, qubits)?;
    let register_len = 1usize
        .checked_shl(qubits.len() as u32)
        .ok_or(QuantumError::InvalidNumQubits)?;
    let mut probabilities = vec![0.0; register_len];

    for (index, amplitude) in circuit.state().iter().enumerate() {
        probabilities[extract_register_value(index, qubits)] += amplitude.norm_sqr();
    }

    let draw = rand::thread_rng().gen::<f64>();
    let mut cumulative = 0.0;
    let mut result = register_len - 1;
    for (value, probability) in probabilities.iter().enumerate() {
        cumulative += probability;
        if draw <= cumulative {
            result = value;
            break;
        }
    }

    let keep_probability = probabilities[result];
    let scale = if keep_probability > 0.0 {
        1.0 / keep_probability.sqrt()
    } else {
        0.0
    };

    for (index, amplitude) in circuit.state_mut().iter_mut().enumerate() {
        if extract_register_value(index, qubits) == result {
            *amplitude *= scale;
        } else {
            *amplitude = Default::default();
        }
    }

    circuit.push_operation(Operation::MeasureRegister {
        qubits: qubits.to_vec(),
        result,
    });
    Ok(result)
}

fn validate_register(circuit: &QuantumCircuit, qubits: &[usize]) -> Result<(), QuantumError> {
    if qubits.is_empty() {
        return Err(QuantumError::InvalidNumQubits);
    }

    for (position, &qubit) in qubits.iter().enumerate() {
        circuit.validate_qubit(qubit)?;
        if let Some(&duplicate) = qubits[..position].iter().find(|&&seen| seen == qubit) {
            return Err(QuantumError::DuplicateQubit {
                q1: duplicate,
                q2: qubit,
            });
        }
    }

    Ok(())
}

fn extract_register_value(index: usize, qubits: &[usize]) -> usize {
    qubits
        .iter()
        .enumerate()
        .fold(0usize, |value, (position, &qubit)| {
            value | (((index >> qubit) & 1) << position)
        })
}
