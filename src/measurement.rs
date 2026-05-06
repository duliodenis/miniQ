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
