use crate::QuantumError;

pub fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let remainder = a % b;
        a = b;
        b = remainder;
    }
    a
}

pub fn mod_pow(base: u64, exponent: u64, modulus: u64) -> Result<u64, QuantumError> {
    if modulus == 0 {
        return Err(QuantumError::InvalidArithmeticInput);
    }

    let mut result = 1u128;
    let mut base = (base % modulus) as u128;
    let mut exponent = exponent;
    let modulus = modulus as u128;

    while exponent > 0 {
        if exponent & 1 == 1 {
            result = (result * base) % modulus;
        }
        base = (base * base) % modulus;
        exponent >>= 1;
    }

    Ok(result as u64)
}

pub fn continued_fraction_denominator(phase: f64, max_denominator: u64) -> Option<u64> {
    if !phase.is_finite() || phase < 0.0 || max_denominator == 0 {
        return None;
    }

    let mut value = phase.fract();
    let mut denominator_before_previous = 1u64;
    let mut previous_denominator = 0u64;
    let mut best_denominator = 1u64;

    for _ in 0..64 {
        let coefficient = value.floor() as u64;
        let next_denominator = coefficient
            .checked_mul(previous_denominator)?
            .checked_add(denominator_before_previous)?;

        if next_denominator > max_denominator {
            return Some(best_denominator);
        }

        best_denominator = next_denominator;

        let remainder = value - coefficient as f64;
        if remainder.abs() < 1e-12 {
            return Some(best_denominator);
        }

        denominator_before_previous = previous_denominator;
        previous_denominator = next_denominator;
        value = 1.0 / remainder;
    }

    Some(best_denominator)
}
