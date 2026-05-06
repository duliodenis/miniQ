use mini_q::QuantumCircuit;

fn probability(qc: &QuantumCircuit, label: &str) -> f64 {
    qc.probabilities(0.0)
        .into_iter()
        .find(|(basis, _)| basis.as_str() == label)
        .map(|(_, probability)| probability)
        .unwrap_or(0.0)
}

#[test]
fn bell_state_has_only_correlated_outcomes() {
    let mut qc = QuantumCircuit::new(2).unwrap();
    qc.h(0).unwrap();
    qc.cnot(0, 1).unwrap();

    assert!((probability(&qc, "00") - 0.5).abs() < 1e-10);
    assert!((probability(&qc, "11") - 0.5).abs() < 1e-10);
    assert!(probability(&qc, "01") < 1e-10);
    assert!(probability(&qc, "10") < 1e-10);
}

#[test]
fn bell_measurement_sampling_never_returns_anticorrelated_states() {
    let mut count_00 = 0;
    let mut count_11 = 0;

    for _ in 0..1_000 {
        let mut qc = QuantumCircuit::new(2).unwrap();
        qc.h(0).unwrap();
        qc.cnot(0, 1).unwrap();
        match qc.measure_all().unwrap().as_str() {
            "00" => count_00 += 1,
            "11" => count_11 += 1,
            other => panic!("unexpected Bell measurement result: {other}"),
        }
    }

    assert!(count_00 >= 300, "00 count was {count_00}");
    assert!(count_11 >= 300, "11 count was {count_11}");
}
