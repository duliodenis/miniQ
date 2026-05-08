use mini_q::algorithms::shor_factor_15_attempt;

fn main() -> anyhow::Result<()> {
    let n = 15;
    let a = 7;
    let attempt = shor_factor_15_attempt()?;

    println!("N = {n}");
    println!("a = {a}");
    println!("Measured work-register value = {}", attempt.work_value);
    println!("Counting measurement = {}", attempt.counting_value);
    println!("Phase estimate = {}", attempt.phase);

    match attempt.period {
        Some(period) => {
            println!("Recovered period r = {period}");
            println!("Factors = {:?}", attempt.factors);
        }
        None => {
            println!("Phase sample did not recover a nontrivial period; rerun the example.");
        }
    }

    Ok(())
}
