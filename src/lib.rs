//! miniQ is a small educational quantum computer emulator.
//!
//! It uses a full state-vector simulation model: an `n`-qubit circuit stores
//! `2^n` complex amplitudes. That makes the implementation easy to inspect and
//! useful for tiny algorithms, but memory grows exponentially.
//!
//! Qubit indexing is little-endian internally. Qubit `0` is the least
//! significant bit of a basis index. Displayed bitstrings are shown
//! most-significant-bit first.
//!
//! ```
//! use mini_q::QuantumCircuit;
//!
//! let mut qc = QuantumCircuit::new(2)?;
//! qc.h(0)?;
//! qc.cnot(0, 1)?;
//! assert_eq!(qc.probabilities(1e-12).len(), 2);
//! # Ok::<(), mini_q::QuantumError>(())
//! ```
//!
//! miniQ includes toy Shor-style building blocks for factoring `15`, but it is
//! not RSA-scale factoring software.

pub mod algorithms;
pub mod circuit;
pub mod error;
pub mod gates;
pub mod measurement;
pub mod postprocessing;
pub mod utils;

pub use circuit::{Operation, QuantumCircuit};
pub use error::QuantumError;
