# Specification: Small Quantum Computer Emulator in Rust

Project Name

mini-q

Purpose

Build a small educational quantum computer emulator in Rust using a full state-vector simulation model.

The emulator must support basic quantum gates, measurement, entanglement, circuit inspection, and enough structure to later implement toy versions of Shor’s algorithm, such as factoring 15 or 21.

This emulator is not intended to factor real RSA numbers. It is designed to help us understand and test small quantum algorithms.

This implementation uses a full state-vector simulator: an `n`-qubit circuit stores `2^n` complex amplitudes in `Vec<Complex64>`. That model is exact and easy to inspect, but memory doubles for each additional qubit. It is appropriate for tiny examples such as Bell states, two-qubit Grover, and future toy Shor demos for numbers like 15 or 21.

It cannot factor RSA-896. Factoring RSA-896 would require a fault-tolerant quantum implementation with many logical and physical qubits, long coherent computations, and resources far beyond a classical full state-vector emulator.

Run the project with:

```sh
cargo test
cargo run --example bell_state
cargo run --example superposition
cargo run --example swap_demo
cargo run --example grover_2qubit_demo
cargo run --example phase_estimation_demo
cargo run --example shor_known_period_15
cargo run --example shor_placeholder
cargo run --bin miniq -- bell
```

Future Shor support now has its first building blocks: QFT, inverse QFT over selected qubits, a small phase-estimation helper for known controlled-phase eigenvalues, and postprocessing helpers for gcd, modular exponentiation, continued fractions, phase-to-period recovery, and factor extraction from a known period. The `shor_known_period_15` example factors 15 from a recovered or supplied period. Remaining work includes controlled modular multiplication and modular exponentiation circuits so the emulator can produce phase samples for modular functions.

⸻

Language and Runtime

Required Language

Rust stable edition 2021 or newer.

Recommended:

edition = "2021"

Required Crates

Use:

num-complex = "0.4"
rand = "0.8"
anyhow = "1"
thiserror = "1"

Optional:

clap = { version = "4", features = ["derive"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"

Do not use existing quantum computing libraries.

⸻

Core Design

Use a full state-vector simulator.

For n qubits, store:

2^n complex amplitudes

The state vector type should be:

Vec<Complex64>

where:

use num_complex::Complex64;

The initial state should be:

|00...0>

with amplitude 1.0 + 0.0i.

⸻

Recommended Project Structure

mini-q/
  Cargo.toml
  README.md
  src/
    lib.rs
    circuit.rs
    gates.rs
    error.rs
    measurement.rs
    utils.rs
    bin/
      miniq.rs
  examples/
    bell_state.rs
    superposition.rs
    swap_demo.rs
    grover_2qubit_demo.rs
    shor_placeholder.rs
  tests/
    gates_tests.rs
    measurement_tests.rs
    entanglement_tests.rs
    probabilities_tests.rs

⸻

Public API

Expose the main type:

pub struct QuantumCircuit

from:

mini_q::QuantumCircuit

Example:

use mini_q::QuantumCircuit;
fn main() -> anyhow::Result<()> {
    let mut qc = QuantumCircuit::new(2)?;
    qc.h(0)?;
    qc.cnot(0, 1)?;
    qc.print_state(1e-12);
    println!("{:?}", qc.probabilities(1e-12));
    Ok(())
}

Additional construction helpers:

pub fn from_basis_state(num_qubits: usize, basis_index: usize) -> Result<Self, QuantumError>;
pub fn num_qubits(&self) -> usize;

⸻

Qubit Indexing Rule

Use little-endian indexing internally.

This means:

qubit 0 = least significant bit
qubit 1 = next bit
qubit n - 1 = most significant bit

For example:

basis index 5 = binary 101

means:

qubit 0 = 1
qubit 1 = 0
qubit 2 = 1

When displaying bitstrings, show them in normal human-readable order:

most significant bit first

So index 5 in a 3-qubit circuit displays as:

101

⸻

Main Struct

pub struct QuantumCircuit {
    num_qubits: usize,
    state: Vec<Complex64>,
    operations: Vec<Operation>,
}

The internal fields may be private, but provide read-only accessors.

⸻

Operation History

Define:

#[derive(Debug, Clone, PartialEq)]
pub enum Operation {
    X { target: usize },
    Y { target: usize },
    Z { target: usize },
    H { target: usize },
    S { target: usize },
    T { target: usize },
    Rx { target: usize, theta: f64 },
    Ry { target: usize, theta: f64 },
    Rz { target: usize, theta: f64 },
    CNot { control: usize, target: usize },
    Cz { control: usize, target: usize },
    Swap { q1: usize, q2: usize },
    ControlledPhase { control: usize, target: usize, theta: f64 },
    Qft { qubits: Vec<usize> },
    InverseQft { qubits: Vec<usize> },
    Measure { target: usize, result: u8 },
    MeasureAll { result: String },
}

Expose:

pub fn operations(&self) -> &[Operation]

⸻

Constructor

impl QuantumCircuit {
    pub fn new(num_qubits: usize) -> Result<Self, QuantumError>
}

Requirements:

* Reject num_qubits == 0.
* Reject overly large values that would overflow 2^n.
* Allocate state vector of size 1usize << num_qubits.
* Initialize state vector to all zeros.
* Set state[0] = Complex64::new(1.0, 0.0).

Acceptance:

let qc = QuantumCircuit::new(3)?;

Initial state probability:

|000> = 1.0

⸻

Error Handling

Create:

pub enum QuantumError

using thiserror.

Required error cases:

InvalidQubit { index: usize, num_qubits: usize }
DuplicateQubit { q1: usize, q2: usize }
InvalidMatrix
InvalidNumQubits
StateNotNormalized { norm: f64 }

Every gate method should return:

Result<(), QuantumError>

Measurement should return:

Result<u8, QuantumError>

or:

Result<String, QuantumError>

⸻

Required Gates

Single-Qubit Gates

Implement:

pub fn x(&mut self, target: usize) -> Result<(), QuantumError>;
pub fn y(&mut self, target: usize) -> Result<(), QuantumError>;
pub fn z(&mut self, target: usize) -> Result<(), QuantumError>;
pub fn h(&mut self, target: usize) -> Result<(), QuantumError>;
pub fn s(&mut self, target: usize) -> Result<(), QuantumError>;
pub fn t(&mut self, target: usize) -> Result<(), QuantumError>;
pub fn rx(&mut self, target: usize, theta: f64) -> Result<(), QuantumError>;
pub fn ry(&mut self, target: usize, theta: f64) -> Result<(), QuantumError>;
pub fn rz(&mut self, target: usize, theta: f64) -> Result<(), QuantumError>;

Two-Qubit Gates

Implement:

pub fn cnot(&mut self, control: usize, target: usize) -> Result<(), QuantumError>;
pub fn cz(&mut self, control: usize, target: usize) -> Result<(), QuantumError>;
pub fn swap(&mut self, q1: usize, q2: usize) -> Result<(), QuantumError>;
pub fn controlled_phase(
    &mut self,
    control: usize,
    target: usize,
    theta: f64
) -> Result<(), QuantumError>;

pub fn qft(&mut self, qubits: &[usize]) -> Result<(), QuantumError>;
pub fn inverse_qft(&mut self, qubits: &[usize]) -> Result<(), QuantumError>;

Low-Level Gate Application

Implement:

pub fn apply_single_qubit_gate(
    &mut self,
    matrix: [[Complex64; 2]; 2],
    target: usize
) -> Result<(), QuantumError>;

This should update the state vector in place or using a temporary vector.

⸻

Gate Definitions

X Gate

[[0, 1],
 [1, 0]]

Y Gate

[[0, -i],
 [i,  0]]

Z Gate

[[1,  0],
 [0, -1]]

Hadamard Gate

1/sqrt(2) * [[1,  1],
             [1, -1]]

S Gate

[[1, 0],
 [0, i]]

T Gate

[[1, 0],
 [0, exp(i*pi/4)]]

Rotation Gates

Rx(theta) =
[[cos(theta/2), -i sin(theta/2)],
 [-i sin(theta/2), cos(theta/2)]]
Ry(theta) =
[[cos(theta/2), -sin(theta/2)],
 [sin(theta/2),  cos(theta/2)]]
Rz(theta) =
[[exp(-i theta/2), 0],
 [0, exp(i theta/2)]]

⸻

Measurement

Measure One Qubit

Implement:

pub fn measure(&mut self, target: usize) -> Result<u8, QuantumError>;

Behavior:

* Validate target qubit.
* Compute probability that the target qubit is 0.
* Compute probability that the target qubit is 1.
* Randomly choose 0 or 1 according to those probabilities.
* Collapse the full state vector consistently with the outcome.
* Renormalize the state vector.
* Append operation history:

Operation::Measure { target, result }

⸻

Measure All Qubits

Implement:

pub fn measure_all(&mut self) -> Result<String, QuantumError>;

Behavior:

* Compute probability for every basis state.
* Randomly choose one basis state according to the probability distribution.
* Collapse the full state vector to that basis state.
* Return the measured bitstring.
* Append operation history:

Operation::MeasureAll { result }

⸻

State Inspection

Implement:

pub fn state(&self) -> &[Complex64];

Implement:

pub fn num_qubits(&self) -> usize;

Implement:

pub fn probabilities(&self, threshold: f64) -> Vec<(String, f64)>;

Example output for a Bell state:

vec![
    ("00".to_string(), 0.5),
    ("11".to_string(), 0.5),
]

Implement:

pub fn print_state(&self, threshold: f64);

Example output:

|00>: 0.7071067811865475+0i
|11>: 0.7071067811865475+0i

Implement utility:

pub fn basis_label(index: usize, num_qubits: usize) -> String;

⸻

Normalization

Implement:

pub fn norm(&self) -> f64;
pub fn assert_normalized(&self, tolerance: f64) -> Result<(), QuantumError>;

The state vector norm means:

sum of squared magnitudes of all amplitudes

It should remain approximately:

1.0

Recommended tolerance:

1e-10

⸻

Examples

Example 1: Bell State

File:

examples/bell_state.rs
use mini_q::QuantumCircuit;
fn main() -> anyhow::Result<()> {
    let mut qc = QuantumCircuit::new(2)?;
    qc.h(0)?;
    qc.cnot(0, 1)?;
    qc.print_state(1e-12);
    println!("{:?}", qc.probabilities(1e-12));
    println!("Measurement: {}", qc.measure_all()?);
    Ok(())
}

Acceptance:

* Only 00 and 11 have nonzero probabilities.
* Each has probability approximately 0.5.

⸻

Example 2: Superposition

File:

examples/superposition.rs
use mini_q::QuantumCircuit;
fn main() -> anyhow::Result<()> {
    let mut qc = QuantumCircuit::new(1)?;
    qc.h(0)?;
    println!("{:?}", qc.probabilities(1e-12));
    Ok(())
}

Acceptance:

[("0", 0.5), ("1", 0.5)]

⸻

Example 3: Swap Demo

File:

examples/swap_demo.rs
use mini_q::QuantumCircuit;
fn main() -> anyhow::Result<()> {
    let mut qc = QuantumCircuit::new(2)?;
    qc.x(0)?;
    qc.swap(0, 1)?;
    qc.print_state(1e-12);
    Ok(())
}

Acceptance:

Final state:

|10>: 1.0

⸻

Example 4: Two-Qubit Grover Demo

File:

examples/grover_2qubit_demo.rs

Implement a tiny two-qubit Grover demo that marks one state, applies a diffusion operator, and shows amplified probability.

Acceptance:

* The marked state has the dominant probability after one Grover iteration.

⸻

Example 5: Shor Placeholder

File:

examples/shor_placeholder.rs

This should compile and print a message explaining that Shor’s algorithm will be added in a later milestone.

Include comments explaining the future steps:

1. Choose N, such as 15.
2. Choose a where gcd(a, N) = 1.
3. Use quantum period finding for f(x) = a^x mod N.
4. Apply inverse QFT to the counting register.
5. Measure.
6. Use continued fractions to recover the period r.
7. Compute gcd(a^(r/2) - 1, N) and gcd(a^(r/2) + 1, N).

⸻

Future Shor’s Algorithm Requirements

Do not implement full Shor’s algorithm in this first milestone.

However, design the API so a later milestone can add:

pub fn qft(&mut self, qubits: &[usize]) -> Result<(), QuantumError>;
pub fn inverse_qft(&mut self, qubits: &[usize]) -> Result<(), QuantumError>;
pub fn controlled_unitary(...);
pub fn modular_exponentiation(...);

A later milestone should include:

examples/shor_factor_15.rs

The first realistic Shor target should be:

15

not RSA-896.

⸻

CLI

Create an optional CLI binary:

src/bin/miniq.rs

The CLI should support:

cargo run --bin miniq -- bell
cargo run --bin miniq -- superposition
cargo run --bin miniq -- swap

Use clap if convenient.

Acceptance:

cargo run --bin miniq -- bell

prints Bell state probabilities.

⸻

Testing Requirements

Use Rust integration and unit tests.

Run:

cargo test

Required Tests

Initial State

let qc = QuantumCircuit::new(3)?;

Expected:

|000> probability = 1.0

X Gate

let mut qc = QuantumCircuit::new(1)?;
qc.x(0)?;

Expected:

|1> probability = 1.0

Hadamard Gate

let mut qc = QuantumCircuit::new(1)?;
qc.h(0)?;

Expected:

|0> probability ≈ 0.5
|1> probability ≈ 0.5

Bell State

let mut qc = QuantumCircuit::new(2)?;
qc.h(0)?;
qc.cnot(0, 1)?;

Expected:

|00> probability ≈ 0.5
|11> probability ≈ 0.5

Measurement Collapse

After measuring a qubit:

* The state vector collapses.
* The state remains normalized.

Swap

let mut qc = QuantumCircuit::new(2)?;
qc.x(0)?;
qc.swap(0, 1)?;

Expected:

|10> probability = 1.0

Invalid Qubit

let mut qc = QuantumCircuit::new(2)?;
let result = qc.h(99);
assert!(result.is_err());

Invalid CNOT

let mut qc = QuantumCircuit::new(2)?;
let result = qc.cnot(0, 0);
assert!(result.is_err());

Bell Measurement Sampling

Run Bell state preparation 1,000 times.

Acceptance:

* Results must only be 00 or 11.
* There must be no 01 or 10.
* Counts should be reasonably balanced, for example each outcome appears at least 300 times.

⸻

Performance Requirements

The emulator should avoid constructing full 2^n x 2^n matrices.

Instead, apply gates directly to the state vector.

For a single-qubit gate:

* Iterate over basis-state pairs that differ only in the target qubit.
* Update the pair of amplitudes.

For example:

|...0...> and |...1...>

For CNOT:

* Iterate over the state vector.
* When the control bit is 1 and target bit is 0, swap amplitudes with the state where the target bit is flipped.

This is much faster and more memory-efficient than matrix multiplication.

⸻

Acceptance Criteria

The implementation is accepted when:

1. cargo test passes.
2. cargo run --example bell_state prints a Bell state with approximately equal 00 and 11 probabilities.
3. cargo run --example superposition prints approximately 0.5 probabilities for 0 and 1.
4. cargo run --example swap_demo prints final state |10>.
5. Bell state measurement over 1,000 trials never returns 01 or 10.
6. No existing quantum-computing crate is used.
7. The README explains:
    * What the emulator is.
    * How state-vector simulation works.
    * Why memory scales as 2^n.
    * How to run examples and tests.
    * Why this cannot factor RSA-896.
    * How future Shor’s algorithm support will be added.
8. Gate application avoids building full 2^n x 2^n matrices.

⸻
