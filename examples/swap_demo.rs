use mini_q::QuantumCircuit;

fn main() -> anyhow::Result<()> {
    let mut qc = QuantumCircuit::new(2)?;
    qc.x(0)?;
    qc.swap(0, 1)?;
    qc.print_state(1e-12);
    Ok(())
}
