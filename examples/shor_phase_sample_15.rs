use mini_q::postprocessing::{factor_from_period, recover_period_from_phase};

fn main() -> anyhow::Result<()> {
    let n = 15;
    let a = 7;
    let phase = 0.25;
    let max_period = 32;

    println!("N = {n}");
    println!("a = {a}");
    println!("phase sample = {phase}");

    match recover_period_from_phase(phase, a, n, max_period)? {
        Some(period) => {
            println!("recovered period r = {period}");
            match factor_from_period(a, n, period)? {
                Some((factor_1, factor_2)) => {
                    println!("factors = {factor_1} and {factor_2}");
                }
                None => {
                    println!("period did not produce nontrivial factors");
                }
            }
        }
        None => {
            println!("phase sample did not produce a valid period");
        }
    }

    Ok(())
}
