use mini_q::{
    postprocessing::{factor_from_period, recover_period_from_phase},
    QuantumCircuit,
};

fn main() -> anyhow::Result<()> {
    let n = 15;
    let a = 7;
    let counting = [4, 5, 6];
    let work = [0, 1, 2, 3];
    let max_period = 32;

    let mut qc = QuantumCircuit::from_basis_state(7, 1)?;
    for &control in &counting {
        qc.h(control)?;
    }
    qc.modular_exponentiation(&counting, &work, a, n)?;

    let work_value = qc.measure_register(&work)?;
    qc.inverse_qft(&counting)?;
    let counting_value = qc.measure_register(&counting)?;
    let phase = counting_value as f64 / (1usize << counting.len()) as f64;

    println!("N = {n}");
    println!("a = {a}");
    println!("Measured work-register value = {work_value}");
    println!("Counting measurement = {counting_value}");
    println!("Phase estimate = {phase}");

    match recover_period_from_phase(phase, a, n, max_period)? {
        Some(period) => {
            println!("Recovered period r = {period}");
            println!("Factors = {:?}", factor_from_period(a, n, period)?);
        }
        None => {
            println!("Phase sample did not recover a nontrivial period; rerun the example.");
        }
    }

    Ok(())
}
