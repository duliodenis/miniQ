use mini_q::postprocessing::{factor_from_period, recover_period_from_phase};

#[test]
fn shor_style_phase_sample_pipeline_factors_fifteen() {
    let period = recover_period_from_phase(0.25, 7, 15, 32).unwrap();
    assert_eq!(period, Some(4));

    let factors = factor_from_period(7, 15, period.unwrap()).unwrap();
    assert_eq!(factors, Some((3, 5)));
}
