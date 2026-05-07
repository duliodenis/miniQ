use anyhow::Result;
use clap::{Parser, Subcommand};
use mini_q::{
    postprocessing::{factor_via_classical_period_finding, find_period_classically, mod_pow},
    QuantumCircuit,
};

#[derive(Debug, Parser)]
#[command(about = "Run small mini-q quantum circuit demos")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    Bell,
    Superposition,
    Swap,
    Period,
    Modexp,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Command::Bell => {
            let mut qc = QuantumCircuit::new(2)?;
            qc.h(0)?;
            qc.cnot(0, 1)?;
            println!("{:?}", qc.probabilities(1e-12));
        }
        Command::Superposition => {
            let mut qc = QuantumCircuit::new(1)?;
            qc.h(0)?;
            println!("{:?}", qc.probabilities(1e-12));
        }
        Command::Swap => {
            let mut qc = QuantumCircuit::new(2)?;
            qc.x(0)?;
            qc.swap(0, 1)?;
            qc.print_state(1e-12);
        }
        Command::Period => {
            let n = 15;
            let a = 2;
            let max_period = 32;

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
        Command::Modexp => {
            let n = 15;
            let a = 7;
            let initial_target = 3;
            let exponent = 3;
            let controls = [4, 5];
            let targets = [0, 1, 2, 3];

            let mut qc = QuantumCircuit::from_basis_state(6, 0b11_0011)?;
            qc.modular_exponentiation(&controls, &targets, a, n)?;

            println!("N = {n}");
            println!("a = {a}");
            println!("controls encode exponent = {exponent}");
            println!("target starts as {initial_target}");
            println!("target maps to {initial_target} * {a}^{exponent} mod {n} = 9");
            qc.print_state(1e-12);
        }
    }
    Ok(())
}
