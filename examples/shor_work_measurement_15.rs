use mini_q::QuantumCircuit;

fn main() -> anyhow::Result<()> {
    let n = 15;
    let a = 7;
    let controls = [4, 5, 6];
    let targets = [0, 1, 2, 3];

    let mut qc = QuantumCircuit::from_basis_state(7, 1)?;
    for &control in &controls {
        qc.h(control)?;
    }
    qc.modular_exponentiation(&controls, &targets, a, n)?;

    let work_value = qc.measure_register(&targets)?;

    println!("N = {n}");
    println!("a = {a}");
    println!("Measured work-register value = {work_value}");
    println!("Collapsed counting/work state:");
    qc.print_state(1e-12);
    println!("{:?}", qc.probabilities(1e-12));

    Ok(())
}
