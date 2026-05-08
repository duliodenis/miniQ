# miniQ

miniQ is a small educational quantum computer emulator written in Rust. It uses a full state-vector simulator, so an `n`-qubit circuit is represented by `2^n` complex amplitudes stored as `Vec<Complex64>`.

The simulator uses little-endian qubit indexing internally: qubit `0` is the least significant bit of the basis-state index. Displayed bitstrings are printed most-significant-bit first, so index `5` in a three-qubit circuit appears as `101`.

## Current Status

miniQ currently supports:

- State-vector simulation with direct gate application, no full `2^n x 2^n` matrices
- Single-qubit gates: X, Y, Z, H, S, T, Rx, Ry, Rz
- Two-qubit gates: CNOT, CZ, SWAP, controlled phase
- Measurement of one qubit, all qubits, or a selected little-endian register
- Operation history through `Operation`
- QFT and inverse QFT over selected registers
- Controlled basis permutations
- Controlled modular multiplication, modular-multiply powers, and modular exponentiation
- Phase-estimation and configurable order-finding helpers
- Postprocessing helpers for gcd, modular exponentiation, continued fractions, period recovery, and factor extraction
- A retrying educational Shor-style factor-15 path

## Example Progression

```sh
cargo test

cargo run --example bell_state
cargo run --example superposition
cargo run --example swap_demo
cargo run --example grover_2qubit_demo

cargo run --example phase_estimation_demo
cargo run --example modular_exponentiation_demo
cargo run --example period_finding_classical_demo

cargo run --example shor_order_finding_circuit_15
cargo run --example shor_work_measurement_15
cargo run --example shor_period_recovery_15
cargo run --example shor_factor_15
```

Additional Shor-related examples:

```sh
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

The circuit demos print compact probability tables by default. Add `--shots` to
sample repeated measurements from the final state, `--history` to show recorded
operations, or `--state` to show raw nonzero amplitudes:

```sh
cargo run --bin miniq -- bell --shots 1000 --history
cargo run --bin miniq -- superposition --shots 1000
cargo run --bin miniq -- swap --state --history
cargo run --bin miniq -- --help
```

## Guides

- [Educational Shor factor-15 walkthrough](docs/shor-factor-15.md)
- [Changelog](CHANGELOG.md)

## Limits

State-vector simulation is exact for small circuits, but memory doubles with every added qubit. A 30-qubit state needs over one billion complex amplitudes. miniQ is useful for learning and small algorithm demos, not for large-scale quantum workloads.

miniQ cannot factor RSA-896. RSA-scale Shor factoring requires far more qubits, fault tolerance, error correction, and coherent depth than a classical full state-vector emulator can provide. The `shor_factor_15` example is intentionally small-scale and educational.

## Future Work

Good next steps include making the order-finding API more ergonomic, adding more small-number examples, and exploring a richer terminal or browser UI. The project should continue to make a clear distinction between educational small-number Shor demos and real fault-tolerant quantum factoring.
