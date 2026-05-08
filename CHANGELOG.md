# Changelog

## 0.1.0

Initial miniQ milestone.

### Added

- Full state-vector simulator using `Vec<Complex64>`.
- Little-endian qubit/register indexing with MSB-first displayed bitstrings.
- Core gates: X, Y, Z, H, S, T, Rx, Ry, Rz, CNOT, CZ, SWAP, controlled phase.
- Measurement for single qubits, all qubits, and selected registers.
- Operation history through `Operation`.
- QFT and inverse QFT.
- Controlled basis permutations.
- Controlled modular multiplication, modular-multiply powers, and modular exponentiation.
- Phase-estimation and configurable order-finding helpers.
- Classical postprocessing helpers for gcd, modular exponentiation, continued fractions, period recovery, and factor extraction.
- Educational Shor factor-15 examples, including a retrying factorization path.
- CLI demos for Bell, superposition, swap, classical period finding, and modular exponentiation.

### Limits

- miniQ is a classical full state-vector emulator, so memory grows as `2^n`.
- The Shor examples are small-scale educational demonstrations.
- RSA-scale factoring, fault tolerance, and error correction are not implemented.
