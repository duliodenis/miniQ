use mini_q::{
    postprocessing::{factor_from_period, recover_period_from_phase},
    QuantumCircuit,
};

#[test]
fn shor_style_phase_sample_pipeline_factors_fifteen() {
    let period = recover_period_from_phase(0.25, 7, 15, 32).unwrap();
    assert_eq!(period, Some(4));

    let factors = factor_from_period(7, 15, period.unwrap()).unwrap();
    assert_eq!(factors, Some((3, 5)));
}

#[test]
fn modular_exponentiation_demo_state_maps_target_by_encoded_exponent() {
    let mut qc = QuantumCircuit::from_basis_state(6, 0b11_0011).unwrap();
    qc.modular_exponentiation(&[4, 5], &[0, 1, 2, 3], 7, 15)
        .unwrap();

    assert_eq!(qc.probabilities(1e-10), vec![("111001".to_string(), 1.0)]);
}

#[test]
fn shor_order_finding_circuit_entangles_counting_and_work_registers() {
    let mut qc = QuantumCircuit::from_basis_state(6, 1).unwrap();
    qc.h(4).unwrap();
    qc.h(5).unwrap();
    qc.modular_exponentiation(&[4, 5], &[0, 1, 2, 3], 7, 15)
        .unwrap();

    let probabilities = qc.probabilities(1e-10);
    let expected = [
        ("000001", 0.25),
        ("010111", 0.25),
        ("100100", 0.25),
        ("111101", 0.25),
    ];

    assert_eq!(probabilities.len(), expected.len());
    for ((actual_label, actual_probability), (expected_label, expected_probability)) in
        probabilities.into_iter().zip(expected)
    {
        assert_eq!(actual_label, expected_label);
        assert!((actual_probability - expected_probability).abs() < 1e-10);
    }
}
