use mini_q::postprocessing::{
    factor_from_period, find_period_classically, gcd, mod_pow, recover_period_from_phase,
};

fn main() -> anyhow::Result<()> {
    let n = 15;
    let a = 7;
    let period = 4;

    println!("N = {n}");
    println!("a = {a}");
    println!("Known period r = {period}");
    println!(
        "Classically verified period r = {:?}",
        find_period_classically(a, n, 32)?
    );
    println!("gcd(a, N) = {}", gcd(a, n));
    println!("a^r mod N = {}", mod_pow(a, period, n)?);
    println!(
        "Recovered r from phase 1/4 = {:?}",
        recover_period_from_phase(0.25, a, n, 32)?
    );

    match factor_from_period(a, n, period)? {
        Some((factor_1, factor_2)) => {
            println!("Factors: {factor_1} and {factor_2}");
        }
        None => {
            println!("Known period did not produce nontrivial factors.");
        }
    }

    Ok(())
}
