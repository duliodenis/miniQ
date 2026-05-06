use mini_q::postprocessing::{factor_from_period, find_period_classically, mod_pow};

fn main() -> anyhow::Result<()> {
    let n = 15;
    let a = 2;
    let max_period = 32;

    println!("N = {n}");
    println!("a = {a}");

    match find_period_classically(a, n, max_period)? {
        Some(period) => {
            println!("Classical period r = {period}");
            println!("a^r mod N = {}", mod_pow(a, period, n)?);
            match factor_from_period(a, n, period)? {
                Some((factor_1, factor_2)) => {
                    println!("Factors from r = {factor_1} and {factor_2}");
                }
                None => {
                    println!("Period did not produce nontrivial factors.");
                }
            }
        }
        None => {
            println!("No period found up to {max_period}.");
        }
    }

    Ok(())
}
