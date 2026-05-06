use anyhow::Result;
use clap::{Parser, Subcommand};
use mini_q::{
    postprocessing::{factor_from_period, find_period_classically, mod_pow},
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
                    match factor_from_period(a, n, period)? {
                        Some((factor_1, factor_2)) => {
                            println!("Factors from r = {factor_1} and {factor_2}");
                        }
                        None => println!("Period did not produce nontrivial factors."),
                    }
                }
                None => println!("No period found up to {max_period}."),
            }
        }
    }
    Ok(())
}
