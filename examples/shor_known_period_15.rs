use mini_q::postprocessing::{factor_from_period, gcd, mod_pow};

fn main() -> anyhow::Result<()> {
    let n = 15;
    let a = 7;
    let period = 4;

    println!("N = {n}");
    println!("a = {a}");
    println!("Known period r = {period}");
    println!("gcd(a, N) = {}", gcd(a, n));
    println!("a^r mod N = {}", mod_pow(a, period, n)?);

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
