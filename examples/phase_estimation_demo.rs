use mini_q::algorithms::phase_estimation_for_phase;

fn main() -> anyhow::Result<()> {
    let phase = 0.25;
    let qc = phase_estimation_for_phase(3, phase)?;

    println!("Expected phase: {phase}");
    println!("Expected counting-register estimate: 010");
    qc.print_state(1e-12);
    println!("{:?}", qc.probabilities(1e-12));

    Ok(())
}
