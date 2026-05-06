fn main() -> anyhow::Result<()> {
    println!("Shor's algorithm is planned for a later mini-q milestone.");
    println!("The first realistic target for this educational simulator will be factoring 15.");

    // Future steps:
    // 1. Choose N, such as 15.
    // 2. Choose a where gcd(a, N) = 1.
    // 3. Use quantum period finding for f(x) = a^x mod N.
    // 4. Apply inverse QFT to the counting register.
    // 5. Measure.
    // 6. Use continued fractions to recover the period r.
    // 7. Compute gcd(a^(r/2) - 1, N) and gcd(a^(r/2) + 1, N).
    Ok(())
}
