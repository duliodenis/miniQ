use thiserror::Error;

#[derive(Debug, Error, Clone, PartialEq)]
pub enum QuantumError {
    #[error("invalid qubit index {index}; circuit has {num_qubits} qubits")]
    InvalidQubit { index: usize, num_qubits: usize },
    #[error("qubit indices must be distinct, got {q1} and {q2}")]
    DuplicateQubit { q1: usize, q2: usize },
    #[error("invalid 2x2 gate matrix")]
    InvalidMatrix,
    #[error("invalid number of qubits")]
    InvalidNumQubits,
    #[error("state is not normalized; norm is {norm}")]
    StateNotNormalized { norm: f64 },
}
