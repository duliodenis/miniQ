use mini_q::QuantumCircuit;

fn main() -> anyhow::Result<()> {
    let n = 15;
    let a = 7;
    let controls = [4, 5];
    let targets = [0, 1, 2, 3];

    let mut qc = QuantumCircuit::from_basis_state(6, 1)?;
    qc.h(controls[0])?;
    qc.h(controls[1])?;
    qc.modular_exponentiation(&controls, &targets, a, n)?;

    println!("N = {n}");
    println!("a = {a}");
    println!("Counting register is in superposition over exponents 0..3");
    println!("Work register starts as |1>");
    qc.print_state(1e-12);
    println!("{:?}", qc.probabilities(1e-12));

    Ok(())
}
