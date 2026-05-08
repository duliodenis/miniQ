use mini_q::{
    algorithms::try_factor_from_phase_sample, postprocessing::recover_period_from_phase,
    QuantumCircuit,
};

fn main() -> anyhow::Result<()> {
    let n = 15;
    let a = 7;
    let counting = [4, 5, 6];
    let work = [0, 1, 2, 3];
    let max_period = 32;
    let max_attempts = 20;

    println!("Toy Shor factoring demo");
    println!("N = {n}");
    println!("a = {a}");

    for attempt in 1..=max_attempts {
        let mut qc = QuantumCircuit::from_basis_state(7, 1)?;
        for &control in &counting {
            qc.h(control)?;
        }
        qc.modular_exponentiation(&counting, &work, a, n)?;

        let work_value = qc.measure_register(&work)?;
        qc.inverse_qft(&counting)?;
        let counting_value = qc.measure_register(&counting)?;
        let phase = counting_value as f64 / (1usize << counting.len()) as f64;
        let period = recover_period_from_phase(phase, a, n, max_period)?;
        let factors = try_factor_from_phase_sample(phase, a, n, max_period)?;

        println!(
            "Attempt {attempt}: work={work_value}, count={counting_value}, phase={phase}, period={period:?}, factors={factors:?}"
        );

        if let Some((factor_1, factor_2)) = factors {
            println!("Success: {n} = {factor_1} * {factor_2}");
            return Ok(());
        }
    }

    println!("No nontrivial factors found; rerun the example.");
    Ok(())
}
