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
