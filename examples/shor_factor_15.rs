use mini_q::algorithms::shor_factor_15_attempt;

fn main() -> anyhow::Result<()> {
    let n = 15;
    let a = 7;
    let max_attempts = 20;

    println!("Educational Shor factoring demo");
    println!("N = {n}");
    println!("a = {a}");

    for attempt in 1..=max_attempts {
        let result = shor_factor_15_attempt()?;

        println!(
            "Attempt {attempt}: work={}, count={}, phase={}, period={:?}, factors={:?}",
            result.work_value, result.counting_value, result.phase, result.period, result.factors
        );

        if let Some((factor_1, factor_2)) = result.factors {
            println!("Success: {n} = {factor_1} * {factor_2}");
            return Ok(());
        }
    }

    println!("No nontrivial factors found; rerun the example.");
    Ok(())
}
