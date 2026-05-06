use mini_q::QuantumCircuit;

fn main() -> anyhow::Result<()> {
    let mut qc = QuantumCircuit::new(2)?;
    qc.h(0)?;
    qc.cnot(0, 1)?;
    qc.print_state(1e-12);
    println!("{:?}", qc.probabilities(1e-12));
    println!("Measurement: {}", qc.measure_all()?);
    Ok(())
}
