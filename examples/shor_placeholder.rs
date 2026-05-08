fn main() -> anyhow::Result<()> {
    println!("An educational Shor-style factor-15 demo is available as:");
    println!("cargo run --example shor_factor_15");
    println!("Full Shor/RSA-scale factoring is not implemented.");

    // Future work:
    // 1. Make the factor-15 path more robust across parameter choices.
    // 2. Add clearer circuit-building helpers for order finding.
    // 3. Keep targets educational; this state-vector emulator cannot scale to RSA-sized integers.
    Ok(())
}
