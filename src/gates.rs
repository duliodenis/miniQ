use num_complex::Complex64;
use std::f64::consts::{FRAC_1_SQRT_2, PI};

pub fn x() -> [[Complex64; 2]; 2] {
    [
        [Complex64::new(0.0, 0.0), Complex64::new(1.0, 0.0)],
        [Complex64::new(1.0, 0.0), Complex64::new(0.0, 0.0)],
    ]
}

pub fn y() -> [[Complex64; 2]; 2] {
    [
        [Complex64::new(0.0, 0.0), Complex64::new(0.0, -1.0)],
        [Complex64::new(0.0, 1.0), Complex64::new(0.0, 0.0)],
    ]
}

pub fn z() -> [[Complex64; 2]; 2] {
    [
        [Complex64::new(1.0, 0.0), Complex64::new(0.0, 0.0)],
        [Complex64::new(0.0, 0.0), Complex64::new(-1.0, 0.0)],
    ]
}

pub fn h() -> [[Complex64; 2]; 2] {
    [
        [
            Complex64::new(FRAC_1_SQRT_2, 0.0),
            Complex64::new(FRAC_1_SQRT_2, 0.0),
        ],
        [
            Complex64::new(FRAC_1_SQRT_2, 0.0),
            Complex64::new(-FRAC_1_SQRT_2, 0.0),
        ],
    ]
}

pub fn s() -> [[Complex64; 2]; 2] {
    [
        [Complex64::new(1.0, 0.0), Complex64::new(0.0, 0.0)],
        [Complex64::new(0.0, 0.0), Complex64::new(0.0, 1.0)],
    ]
}

pub fn t() -> [[Complex64; 2]; 2] {
    [
        [Complex64::new(1.0, 0.0), Complex64::new(0.0, 0.0)],
        [
            Complex64::new(0.0, 0.0),
            Complex64::from_polar(1.0, PI / 4.0),
        ],
    ]
}

pub fn rx(theta: f64) -> [[Complex64; 2]; 2] {
    let c = (theta / 2.0).cos();
    let s = (theta / 2.0).sin();
    [
        [Complex64::new(c, 0.0), Complex64::new(0.0, -s)],
        [Complex64::new(0.0, -s), Complex64::new(c, 0.0)],
    ]
}

pub fn ry(theta: f64) -> [[Complex64; 2]; 2] {
    let c = (theta / 2.0).cos();
    let s = (theta / 2.0).sin();
    [
        [Complex64::new(c, 0.0), Complex64::new(-s, 0.0)],
        [Complex64::new(s, 0.0), Complex64::new(c, 0.0)],
    ]
}

pub fn rz(theta: f64) -> [[Complex64; 2]; 2] {
    [
        [
            Complex64::from_polar(1.0, -theta / 2.0),
            Complex64::new(0.0, 0.0),
        ],
        [
            Complex64::new(0.0, 0.0),
            Complex64::from_polar(1.0, theta / 2.0),
        ],
    ]
}
