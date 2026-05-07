# miniQ

`miniQ` is a small educational quantum computer emulator written in Rust. It uses a full state-vector simulator, so an `n`-qubit circuit is represented by `2^n` complex amplitudes stored as `Vec<Complex64>`.

The simulator uses little-endian qubit indexing internally: qubit `0` is the least significant bit of the basis-state index. Displayed bitstrings are printed most-significant-bit first, so index `5` in a three-qubit circuit appears as `101`.

## What It Supports

- Single-qubit gates: X, Y, Z, H, S, T, Rx, Ry, Rz
- Two-qubit gates: CNOT, CZ, SWAP, controlled phase
- Controlled basis permutations for toy algorithm infrastructure
- Controlled modular multiplication, modular-multiply powers, and modular exponentiation
- Register transforms: QFT and inverse QFT
- Algorithm helper: phase estimation for a known controlled-phase eigenvalue
- Postprocessing helpers for `gcd`, modular exponentiation, and continued fractions
- Measurement with state collapse and renormalization
- Operation history through `Operation`
- Probability/state inspection helpers
- Construction helpers including `from_basis_state` and `num_qubits`

Gate and permutation application updates the state vector directly. It does not build full `2^n x 2^n` matrices.

## Memory Growth

State-vector simulation is exact for small circuits, but memory doubles with every added qubit. A 30-qubit state needs over one billion complex amplitudes. That is why this crate is useful for learning and small algorithm demos, not for large-scale quantum workloads.

## Running

```sh
cargo test
cargo run --example bell_state
cargo run --example superposition
cargo run --example swap_demo
cargo run --example grover_2qubit_demo
cargo run --example modular_exponentiation_demo
cargo run --example period_finding_classical_demo
cargo run --example phase_estimation_demo
cargo run --example shor_order_finding_circuit_15
cargo run --example shor_work_measurement_15
cargo run --example shor_period_recovery_15
cargo run --example shor_known_period_15
cargo run --example shor_phase_sample_15
cargo run --example shor_placeholder
```

The CLI also provides a few built-in demos:

```sh
cargo run --bin miniq -- bell
cargo run --bin miniq -- superposition
cargo run --bin miniq -- swap
cargo run --bin miniq -- period
cargo run --bin miniq -- modexp
```

## RSA-896

This emulator cannot factor RSA-896. Shor's algorithm for numbers of that size requires far more qubits, error correction, coherent depth, and memory than a classical full state-vector simulator can provide. The practical memory requirement grows exponentially, while RSA-scale factoring also needs fault-tolerant quantum hardware.

## Future Shor Support

QFT, inverse QFT, controlled basis permutations, controlled modular multiplication, modular-multiply powers, modular exponentiation, a small phase-estimation helper, and postprocessing helpers are now available as the first Shor-oriented building blocks. The postprocessing module also includes tiny classical period finding and factor-via-period helpers for checking small examples. The `shor_known_period_15` example demonstrates factor extraction when a period is available, while `shor_phase_sample_15` turns a supplied phase sample into a period and factors for `15`. Future work should connect modular exponentiation to phase estimation so the emulator can produce phase samples for modular functions.
