use anyhow::Result;
use clap::{Args, Parser, Subcommand};
use mini_q::{
    postprocessing::{factor_via_classical_period_finding, find_period_classically, mod_pow},
    QuantumCircuit,
};
use std::collections::BTreeMap;

#[derive(Debug, Parser)]
#[command(
    name = "miniq",
    about = "Run miniQ educational quantum circuit demos",
    version
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Create a two-qubit Bell state.
    Bell(ShotOptions),
    /// Put one qubit into an equal superposition.
    Superposition(ShotOptions),
    /// Swap |01> into |10>.
    Swap(DisplayOptions),
    /// Find the classical period used by the factor-15 walkthrough.
    Period,
    /// Show controlled modular exponentiation on a small register.
    Modexp(DisplayOptions),
}

#[derive(Debug, Args)]
struct ShotOptions {
    #[command(flatten)]
    display: DisplayOptions,
    /// Run repeated all-qubit measurements from fresh copies of the final state.
    #[arg(long, default_value_t = 0)]
    shots: usize,
}

#[derive(Debug, Args)]
struct DisplayOptions {
    /// Hide probabilities at or below this value.
    #[arg(long, default_value_t = 1e-12)]
    threshold: f64,
    /// Print the raw nonzero amplitudes after the probability table.
    #[arg(long)]
    state: bool,
    /// Print the recorded circuit operations.
    #[arg(long)]
    history: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Command::Bell(options) => {
            let mut qc = QuantumCircuit::new(2)?;
            qc.h(0)?;
            qc.cnot(0, 1)?;
            print_demo("Bell state", &qc, &options.display);
            print_shots(&qc, options.shots)?;
        }
        Command::Superposition(options) => {
            let mut qc = QuantumCircuit::new(1)?;
            qc.h(0)?;
            print_demo("Single-qubit superposition", &qc, &options.display);
            print_shots(&qc, options.shots)?;
        }
        Command::Swap(options) => {
            let mut qc = QuantumCircuit::new(2)?;
            qc.x(0)?;
            qc.swap(0, 1)?;
            print_demo("SWAP demo", &qc, &options);
        }
        Command::Period => {
            let n = 15;
            let a = 2;
            let max_period = 32;

            println!("miniQ period demo");
            println!();
            println!("N = {n}");
            println!("a = {a}");

            match find_period_classically(a, n, max_period)? {
                Some(period) => {
                    println!("Classical period r = {period}");
                    println!("a^r mod N = {}", mod_pow(a, period, n)?);
                    match factor_via_classical_period_finding(a, n, max_period)? {
                        Some((factor_1, factor_2)) => {
                            println!("Factors from r = {factor_1} and {factor_2}");
                        }
                        None => println!("Period did not produce nontrivial factors."),
                    }
                }
                None => println!("No period found up to {max_period}."),
            }
        }
        Command::Modexp(options) => {
            let n = 15;
            let a = 7;
            let initial_target = 3;
            let exponent = 3;
            let controls = [4, 5];
            let targets = [0, 1, 2, 3];

            let mut qc = QuantumCircuit::from_basis_state(6, 0b11_0011)?;
            qc.modular_exponentiation(&controls, &targets, a, n)?;

            println!("miniQ modular exponentiation demo");
            println!();
            println!("N = {n}");
            println!("a = {a}");
            println!("controls encode exponent = {exponent}");
            println!("target starts as {initial_target}");
            println!("target maps to {initial_target} * {a}^{exponent} mod {n} = 9");
            println!();
            print_demo("Final register state", &qc, &options);
        }
    }
    Ok(())
}

fn print_demo(title: &str, circuit: &QuantumCircuit, options: &DisplayOptions) {
    println!("miniQ {title}");
    println!("qubits: {}", circuit.num_qubits());
    println!("norm: {:.12}", circuit.norm());
    println!();
    print_probabilities(circuit, options.threshold);

    if options.state {
        println!();
        println!("Amplitudes");
        circuit.print_state(options.threshold);
    }

    if options.history {
        println!();
        print_history(circuit);
    }
}

fn print_probabilities(circuit: &QuantumCircuit, threshold: f64) {
    let probabilities = circuit.probabilities(threshold);
    let state_width = probabilities
        .iter()
        .map(|(label, _)| label.len())
        .max()
        .unwrap_or(5)
        .max("state".len())
        + 2;

    println!("Probabilities");
    println!("{:<state_width$}  {:>11}  bar", "state", "probability");
    println!("{:-<state_width$}  {:-<11}  {:-<32}", "", "", "");

    for (label, probability) in probabilities {
        let state = format!("|{label}>");
        println!(
            "{state:<state_width$}  {:>11.6}  {}",
            probability,
            probability_bar(probability)
        );
    }
}

fn print_shots(circuit: &QuantumCircuit, shots: usize) -> Result<()> {
    if shots == 0 {
        return Ok(());
    }

    let mut counts = BTreeMap::new();
    for _ in 0..shots {
        let mut trial = circuit.clone();
        let result = trial.measure_all()?;
        *counts.entry(result).or_insert(0usize) += 1;
    }

    println!();
    println!("Shots ({shots})");
    println!("{:<8}  {:>8}  {:>10}", "state", "count", "frequency");
    println!("{:-<8}  {:-<8}  {:-<10}", "", "", "");
    for (label, count) in counts {
        let frequency = count as f64 / shots as f64;
        println!("|{label}>  {count:>8}  {frequency:>10.4}");
    }

    Ok(())
}

fn print_history(circuit: &QuantumCircuit) {
    println!("Operations");
    for (index, operation) in circuit.operations().iter().enumerate() {
        println!("{:>2}. {:?}", index + 1, operation);
    }
}

fn probability_bar(probability: f64) -> String {
    let width = 32usize;
    let filled = (probability.clamp(0.0, 1.0) * width as f64).round() as usize;
    "#".repeat(filled)
}
