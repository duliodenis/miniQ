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
