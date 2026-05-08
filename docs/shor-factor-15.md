# Educational Shor Factor-15 Walkthrough

This guide explains the small-scale Shor-style factorization path included in miniQ. The goal is to show the structure of the algorithm on `N = 15`, not to provide cryptographic-scale factoring.

## What miniQ Demonstrates

The factor-15 path combines:

- a counting register prepared in superposition
- a work register initialized to `|1>`
- modular exponentiation for `a^x mod N`
- work-register measurement
- inverse QFT on the counting register
- phase-to-period recovery
- classical factor extraction with gcd

For the included examples, `N = 15` and `a = 7`. The period is `r = 4`, and successful postprocessing gives factors `3` and `5`.

## Example Sequence

Start with the modular exponentiation block:

```sh
cargo run --example modular_exponentiation_demo
```

Then inspect the order-finding circuit before measurement:

```sh
cargo run --example shor_order_finding_circuit_15
```

Measure the work register to see the counting register collapse into a periodic pattern:

```sh
cargo run --example shor_work_measurement_15
```

Apply inverse QFT, measure the counting register, recover a phase sample, and try to recover the period:

```sh
cargo run --example shor_period_recovery_15
```

Run the retrying factor-15 example:

```sh
cargo run --example shor_factor_15
```

Because measurement is probabilistic, some runs produce uninformative phase samples. The retrying example repeats the attempt until it finds nontrivial factors or reaches its attempt limit.

## Register Layout

miniQ uses little-endian register indexing. In the factor-15 path:

- work register: qubits `[0, 1, 2, 3]`
- counting register: qubits `[4, 5, 6]`

Displayed bitstrings are MSB-first, so the left side of a printed state label corresponds to the highest-index qubits.

## Why This Is Not RSA Factoring

This walkthrough runs on a classical state-vector simulator. The memory required by the simulator doubles with every added qubit. RSA-scale Shor factoring would require fault-tolerant quantum hardware, many logical and physical qubits, error correction, and far deeper coherent computation than miniQ can represent.

The factor-15 path is intentionally small-scale and educational.
