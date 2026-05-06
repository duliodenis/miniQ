# mini-q

`mini-q` is a small educational quantum computer emulator written in Rust. It uses a full state-vector simulator, so an `n`-qubit circuit is represented by `2^n` complex amplitudes stored as `Vec<Complex64>`.

The simulator uses little-endian qubit indexing internally: qubit `0` is the least significant bit of the basis-state index. Displayed bitstrings are printed most-significant-bit first, so index `5` in a three-qubit circuit appears as `101`.

## What It Supports

- Single-qubit gates: X, Y, Z, H, S, T, Rx, Ry, Rz
- Two-qubit gates: CNOT, CZ, SWAP, controlled phase
- Register transforms: QFT and inverse QFT
- Algorithm helper: phase estimation for a known controlled-phase eigenvalue
- Postprocessing helpers for `gcd`, modular exponentiation, and continued fractions
- Measurement with state collapse and renormalization
- Operation history through `Operation`
- Probability/state inspection helpers
- Construction helpers including `from_basis_state` and `num_qubits`

Gate application updates the state vector directly. It does not build full `2^n x 2^n` matrices.

## Memory Growth

State-vector simulation is exact for small circuits, but memory doubles with every added qubit. A 30-qubit state needs over one billion complex amplitudes. That is why this crate is useful for learning and small algorithm demos, not for large-scale quantum workloads.

## Running

```sh
cargo test
cargo run --example bell_state
cargo run --example superposition
cargo run --example swap_demo
cargo run --example grover_2qubit_demo
cargo run --example phase_estimation_demo
cargo run --example shor_placeholder
```

The CLI also provides a few built-in demos:

```sh
cargo run --bin miniq -- bell
cargo run --bin miniq -- superposition
cargo run --bin miniq -- swap
```

## RSA-896

This emulator cannot factor RSA-896. Shor's algorithm for numbers of that size requires far more qubits, error correction, coherent depth, and memory than a classical full state-vector simulator can provide. The practical memory requirement grows exponentially, while RSA-scale factoring also needs fault-tolerant quantum hardware.

## Future Shor Support

QFT, inverse QFT, a small phase-estimation helper, and classical postprocessing helpers are now available as the first Shor-oriented building blocks. Future work should add controlled modular multiplication and modular exponentiation circuits. The first realistic target should be a toy example such as factoring `15`, not RSA-sized integers.
