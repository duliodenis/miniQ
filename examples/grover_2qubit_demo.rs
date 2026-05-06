use mini_q::QuantumCircuit;

fn main() -> anyhow::Result<()> {
    let marked_state = "11";
    let mut qc = QuantumCircuit::new(2)?;

    qc.h(0)?;
    qc.h(1)?;

    // Oracle for |11>: flip the phase only when both qubits are 1.
    qc.cz(0, 1)?;

    // Two-qubit diffusion operator: H^n X^n CZ X^n H^n.
    qc.h(0)?;
    qc.h(1)?;
    qc.x(0)?;
    qc.x(1)?;
    qc.cz(0, 1)?;
    qc.x(0)?;
    qc.x(1)?;
    qc.h(0)?;
    qc.h(1)?;

    println!("Marked state: {marked_state}");
    qc.print_state(1e-12);
    println!("{:?}", qc.probabilities(1e-12));
    Ok(())
}
