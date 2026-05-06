use anyhow::Result;
use clap::{Parser, Subcommand};
use mini_q::QuantumCircuit;

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
    }
    Ok(())
}
