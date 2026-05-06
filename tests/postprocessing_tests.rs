use mini_q::{postprocessing, QuantumError};

#[test]
fn gcd_finds_common_factor() {
    assert_eq!(postprocessing::gcd(21, 15), 3);
    assert_eq!(postprocessing::gcd(15, 21), 3);
    assert_eq!(postprocessing::gcd(17, 13), 1);
}

#[test]
fn mod_pow_computes_modular_exponentiation() {
    assert_eq!(postprocessing::mod_pow(7, 4, 15), Ok(1));
    assert_eq!(postprocessing::mod_pow(2, 10, 17), Ok(4));
}

#[test]
fn mod_pow_rejects_zero_modulus() {
    assert_eq!(
        postprocessing::mod_pow(7, 4, 0),
        Err(QuantumError::InvalidArithmeticInput)
    );
}

#[test]
fn continued_fraction_denominator_recovers_quarter() {
    assert_eq!(
        postprocessing::continued_fraction_denominator(0.25, 32),
        Some(4)
    );
}

#[test]
fn continued_fraction_denominator_recovers_thirds() {
    assert_eq!(
        postprocessing::continued_fraction_denominator(1.0 / 3.0, 32),
        Some(3)
    );
    assert_eq!(
        postprocessing::continued_fraction_denominator(2.0 / 3.0, 32),
        Some(3)
    );
}

#[test]
fn continued_fraction_denominator_rejects_invalid_inputs() {
    assert_eq!(
        postprocessing::continued_fraction_denominator(f64::NAN, 32),
        None
    );
    assert_eq!(
        postprocessing::continued_fraction_denominator(0.25, 0),
        None
    );
}

#[test]
fn factor_from_period_factors_fifteen_with_known_period() {
    assert_eq!(
        postprocessing::factor_from_period(7, 15, 4),
        Ok(Some((3, 5)))
    );
}

#[test]
fn factor_from_period_rejects_odd_period() {
    assert_eq!(
        postprocessing::factor_from_period(7, 15, 3),
        Err(QuantumError::InvalidArithmeticInput)
    );
}

#[test]
fn factor_from_period_rejects_non_period() {
    assert_eq!(
        postprocessing::factor_from_period(7, 15, 2),
        Err(QuantumError::InvalidArithmeticInput)
    );
}

#[test]
fn recover_period_from_phase_recovers_full_period_from_quarter_phase() {
    assert_eq!(
        postprocessing::recover_period_from_phase(0.25, 7, 15, 32),
        Ok(Some(4))
    );
}

#[test]
fn recover_period_from_phase_checks_multiples_of_reduced_denominator() {
    assert_eq!(
        postprocessing::recover_period_from_phase(0.5, 7, 15, 32),
        Ok(Some(4))
    );
}

#[test]
fn recover_period_from_phase_returns_none_when_no_candidate_validates() {
    assert_eq!(
        postprocessing::recover_period_from_phase(0.25, 2, 21, 3),
        Ok(None)
    );
}

#[test]
fn recover_period_from_phase_rejects_invalid_arithmetic_inputs() {
    assert_eq!(
        postprocessing::recover_period_from_phase(0.25, 7, 0, 32),
        Err(QuantumError::InvalidArithmeticInput)
    );
    assert_eq!(
        postprocessing::recover_period_from_phase(0.25, 5, 15, 32),
        Err(QuantumError::InvalidArithmeticInput)
    );
}

#[test]
fn find_period_classically_finds_smallest_period() {
    assert_eq!(
        postprocessing::find_period_classically(7, 15, 32),
        Ok(Some(4))
    );
    assert_eq!(
        postprocessing::find_period_classically(2, 15, 32),
        Ok(Some(4))
    );
}

#[test]
fn find_period_classically_returns_none_when_max_period_is_too_small() {
    assert_eq!(postprocessing::find_period_classically(7, 15, 3), Ok(None));
}

#[test]
fn find_period_classically_rejects_invalid_inputs() {
    assert_eq!(
        postprocessing::find_period_classically(5, 15, 32),
        Err(QuantumError::InvalidArithmeticInput)
    );
    assert_eq!(
        postprocessing::find_period_classically(7, 15, 0),
        Err(QuantumError::InvalidArithmeticInput)
    );
}
