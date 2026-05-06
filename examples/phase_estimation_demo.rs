use mini_q::QuantumCircuit;
use std::f64::consts::PI;

fn main() -> anyhow::Result<()> {
    let counting_qubits = [0, 1, 2];
    let target = 3;
    let phase = 0.25;

    let mut qc = QuantumCircuit::new(4)?;
    qc.x(target)?;

    for &qubit in &counting_qubits {
        qc.h(qubit)?;
    }

    for (power, &control) in counting_qubits.iter().enumerate() {
        let theta = 2.0 * PI * phase * (1usize << power) as f64;
        qc.controlled_phase(control, target, theta)?;
    }

    qc.inverse_qft(&counting_qubits)?;

    println!("Expected phase: {phase}");
    println!("Expected counting-register estimate: 010");
    qc.print_state(1e-12);
    println!("{:?}", qc.probabilities(1e-12));

    Ok(())
}
