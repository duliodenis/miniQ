use mini_q::QuantumCircuit;

fn main() -> anyhow::Result<()> {
    let mut qc = QuantumCircuit::new(1)?;
    qc.h(0)?;
    println!("{:?}", qc.probabilities(1e-12));
    Ok(())
}
