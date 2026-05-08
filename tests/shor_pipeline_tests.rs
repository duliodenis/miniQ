use mini_q::{
    algorithms::try_factor_from_phase_sample,
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
fn try_factor_from_phase_sample_factors_fifteen_for_useful_phase() {
    assert_eq!(
        try_factor_from_phase_sample(0.25, 7, 15, 32),
        Ok(Some((3, 5)))
    );
}

#[test]
fn try_factor_from_phase_sample_returns_none_for_unhelpful_phase() {
    assert_eq!(try_factor_from_phase_sample(0.0, 7, 15, 32), Ok(None));
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

#[test]
fn measuring_work_register_collapses_counting_register_to_periodic_pair() {
    let mut qc = QuantumCircuit::from_basis_state(7, 1).unwrap();
    for control in [4, 5, 6] {
        qc.h(control).unwrap();
    }
    qc.modular_exponentiation(&[4, 5, 6], &[0, 1, 2, 3], 7, 15)
        .unwrap();

    let work_value = qc.measure_register(&[0, 1, 2, 3]).unwrap();
    let probabilities = qc.probabilities(1e-10);

    assert!([1, 4, 7, 13].contains(&work_value));
    assert_eq!(probabilities.len(), 2);
    for (_, probability) in &probabilities {
        assert!((probability - 0.5).abs() < 1e-10);
    }

    let exponents: Vec<usize> = probabilities
        .iter()
        .map(|(label, _)| {
            let index = usize::from_str_radix(label, 2).unwrap();
            (index >> 4) & 0b111
        })
        .collect();
    assert_eq!(exponents[1] - exponents[0], 4);
}

#[test]
fn inverse_qft_on_periodic_counting_pair_has_expected_phase_support() {
    let mut qc = QuantumCircuit::new(7).unwrap();
    qc.h(6).unwrap();
    qc.x(0).unwrap();

    qc.inverse_qft(&[4, 5, 6]).unwrap();

    let probabilities = qc.probabilities(1e-10);
    let expected = [
        ("0000001", 0.25),
        ("0100001", 0.25),
        ("1000001", 0.25),
        ("1100001", 0.25),
    ];

    assert_eq!(probabilities.len(), expected.len());
    for ((actual_label, actual_probability), (expected_label, expected_probability)) in
        probabilities.into_iter().zip(expected)
    {
        assert_eq!(actual_label, expected_label);
        assert!((actual_probability - expected_probability).abs() < 1e-10);
    }
}
