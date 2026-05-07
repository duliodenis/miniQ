use mini_q::QuantumCircuit;

fn main() -> anyhow::Result<()> {
    let n = 15;
    let a = 7;
    let initial_target = 3;
    let exponent = 3;

    let controls = [4, 5];
    let targets = [0, 1, 2, 3];
    let initial_basis = 0b11_0011;

    let mut qc = QuantumCircuit::from_basis_state(6, initial_basis)?;
    qc.modular_exponentiation(&controls, &targets, a, n)?;

    println!("N = {n}");
    println!("a = {a}");
    println!("controls encode exponent = {exponent}");
    println!("target starts as {initial_target}");
    println!("target maps to {initial_target} * {a}^{exponent} mod {n} = 9");
    qc.print_state(1e-12);

    Ok(())
}
