use crate::{
    gates,
    postprocessing::{gcd, mod_pow},
    utils::basis_label,
    QuantumError,
};
use num_complex::Complex64;

#[derive(Debug, Clone, PartialEq)]
pub enum Operation {
    X {
        target: usize,
    },
    Y {
        target: usize,
    },
    Z {
        target: usize,
    },
    H {
        target: usize,
    },
    S {
        target: usize,
    },
    T {
        target: usize,
    },
    Rx {
        target: usize,
        theta: f64,
    },
    Ry {
        target: usize,
        theta: f64,
    },
    Rz {
        target: usize,
        theta: f64,
    },
    CNot {
        control: usize,
        target: usize,
    },
    Cz {
        control: usize,
        target: usize,
    },
    Swap {
        q1: usize,
        q2: usize,
    },
    ControlledPhase {
        control: usize,
        target: usize,
        theta: f64,
    },
    ControlledBasisPermutation {
        control: usize,
        targets: Vec<usize>,
        permutation: Vec<usize>,
    },
    ControlledModularMultiply {
        control: usize,
        targets: Vec<usize>,
        multiplier: u64,
        modulus: u64,
    },
    ControlledModularMultiplyPower {
        control: usize,
        targets: Vec<usize>,
        base: u64,
        power: u64,
        modulus: u64,
    },
    ModularExponentiation {
        controls: Vec<usize>,
        targets: Vec<usize>,
        base: u64,
        modulus: u64,
    },
    Qft {
        qubits: Vec<usize>,
    },
    InverseQft {
        qubits: Vec<usize>,
    },
    Measure {
        target: usize,
        result: u8,
    },
    MeasureAll {
        result: String,
    },
}

#[derive(Debug, Clone)]
pub struct QuantumCircuit {
    num_qubits: usize,
    state: Vec<Complex64>,
    operations: Vec<Operation>,
}

impl QuantumCircuit {
    pub fn new(num_qubits: usize) -> Result<Self, QuantumError> {
        if num_qubits == 0 || num_qubits >= usize::BITS as usize {
            return Err(QuantumError::InvalidNumQubits);
        }

        let len = Self::state_len(num_qubits)?;
        let mut state = Vec::new();
        state
            .try_reserve_exact(len)
            .map_err(|_| QuantumError::InvalidNumQubits)?;
        state.resize(len, Complex64::new(0.0, 0.0));
        state[0] = Complex64::new(1.0, 0.0);

        Ok(Self {
            num_qubits,
            state,
            operations: Vec::new(),
        })
    }

    pub fn from_basis_state(num_qubits: usize, basis_index: usize) -> Result<Self, QuantumError> {
        let len = Self::state_len(num_qubits)?;
        if basis_index >= len {
            return Err(QuantumError::InvalidQubit {
                index: basis_index,
                num_qubits,
            });
        }

        let mut circuit = Self::new(num_qubits)?;
        circuit.state[0] = Complex64::new(0.0, 0.0);
        circuit.state[basis_index] = Complex64::new(1.0, 0.0);
        Ok(circuit)
    }

    pub fn x(&mut self, target: usize) -> Result<(), QuantumError> {
        self.apply_single_qubit_gate(gates::x(), target)?;
        self.operations.push(Operation::X { target });
        Ok(())
    }

    pub fn y(&mut self, target: usize) -> Result<(), QuantumError> {
        self.apply_single_qubit_gate(gates::y(), target)?;
        self.operations.push(Operation::Y { target });
        Ok(())
    }

    pub fn z(&mut self, target: usize) -> Result<(), QuantumError> {
        self.apply_single_qubit_gate(gates::z(), target)?;
        self.operations.push(Operation::Z { target });
        Ok(())
    }

    pub fn h(&mut self, target: usize) -> Result<(), QuantumError> {
        self.apply_single_qubit_gate(gates::h(), target)?;
        self.operations.push(Operation::H { target });
        Ok(())
    }

    pub fn s(&mut self, target: usize) -> Result<(), QuantumError> {
        self.apply_single_qubit_gate(gates::s(), target)?;
        self.operations.push(Operation::S { target });
        Ok(())
    }

    pub fn t(&mut self, target: usize) -> Result<(), QuantumError> {
        self.apply_single_qubit_gate(gates::t(), target)?;
        self.operations.push(Operation::T { target });
        Ok(())
    }

    pub fn rx(&mut self, target: usize, theta: f64) -> Result<(), QuantumError> {
        self.apply_single_qubit_gate(gates::rx(theta), target)?;
        self.operations.push(Operation::Rx { target, theta });
        Ok(())
    }

    pub fn ry(&mut self, target: usize, theta: f64) -> Result<(), QuantumError> {
        self.apply_single_qubit_gate(gates::ry(theta), target)?;
        self.operations.push(Operation::Ry { target, theta });
        Ok(())
    }

    pub fn rz(&mut self, target: usize, theta: f64) -> Result<(), QuantumError> {
        self.apply_single_qubit_gate(gates::rz(theta), target)?;
        self.operations.push(Operation::Rz { target, theta });
        Ok(())
    }

    pub fn cnot(&mut self, control: usize, target: usize) -> Result<(), QuantumError> {
        self.validate_pair(control, target)?;
        let control_mask = 1usize << control;
        let target_mask = 1usize << target;

        for index in 0..self.state.len() {
            if index & control_mask != 0 && index & target_mask == 0 {
                let flipped = index | target_mask;
                self.state.swap(index, flipped);
            }
        }

        self.operations.push(Operation::CNot { control, target });
        Ok(())
    }

    pub fn cz(&mut self, control: usize, target: usize) -> Result<(), QuantumError> {
        self.validate_pair(control, target)?;
        let control_mask = 1usize << control;
        let target_mask = 1usize << target;

        for (index, amplitude) in self.state.iter_mut().enumerate() {
            if index & control_mask != 0 && index & target_mask != 0 {
                *amplitude = -*amplitude;
            }
        }

        self.operations.push(Operation::Cz { control, target });
        Ok(())
    }

    pub fn swap(&mut self, q1: usize, q2: usize) -> Result<(), QuantumError> {
        self.validate_pair(q1, q2)?;
        let q1_mask = 1usize << q1;
        let q2_mask = 1usize << q2;

        for index in 0..self.state.len() {
            let q1_bit = index & q1_mask != 0;
            let q2_bit = index & q2_mask != 0;
            if !q1_bit && q2_bit {
                let swapped = index ^ q1_mask ^ q2_mask;
                self.state.swap(index, swapped);
            }
        }

        self.operations.push(Operation::Swap { q1, q2 });
        Ok(())
    }

    pub fn controlled_phase(
        &mut self,
        control: usize,
        target: usize,
        theta: f64,
    ) -> Result<(), QuantumError> {
        self.validate_pair(control, target)?;
        self.apply_controlled_phase_unchecked(control, target, theta);

        self.operations.push(Operation::ControlledPhase {
            control,
            target,
            theta,
        });
        Ok(())
    }

    pub fn qft(&mut self, qubits: &[usize]) -> Result<(), QuantumError> {
        self.validate_qubits(qubits)?;

        for target_index in (0..qubits.len()).rev() {
            let target = qubits[target_index];
            self.apply_single_qubit_gate(gates::h(), target)?;

            for (control_index, &control) in qubits.iter().enumerate().take(target_index) {
                let theta =
                    std::f64::consts::PI / (1usize << (target_index - control_index)) as f64;
                self.apply_controlled_phase_unchecked(control, target, theta);
            }
        }

        self.reverse_qubit_order_unchecked(qubits);
        self.operations.push(Operation::Qft {
            qubits: qubits.to_vec(),
        });
        Ok(())
    }

    pub fn inverse_qft(&mut self, qubits: &[usize]) -> Result<(), QuantumError> {
        self.validate_qubits(qubits)?;

        self.reverse_qubit_order_unchecked(qubits);

        for target_index in 0..qubits.len() {
            let target = qubits[target_index];

            for control_index in (0..target_index).rev() {
                let control = qubits[control_index];
                let theta =
                    -std::f64::consts::PI / (1usize << (target_index - control_index)) as f64;
                self.apply_controlled_phase_unchecked(control, target, theta);
            }

            self.apply_single_qubit_gate(gates::h(), target)?;
        }

        self.operations.push(Operation::InverseQft {
            qubits: qubits.to_vec(),
        });
        Ok(())
    }

    pub fn apply_controlled_basis_permutation(
        &mut self,
        control: usize,
        targets: &[usize],
        permutation: &[usize],
    ) -> Result<(), QuantumError> {
        self.validate_qubit(control)?;
        self.validate_qubits(targets)?;
        if targets.contains(&control) {
            return Err(QuantumError::DuplicateQubit {
                q1: control,
                q2: control,
            });
        }

        self.validate_permutation(targets.len(), permutation)?;

        self.apply_controlled_basis_permutation_unchecked(control, targets, permutation);
        self.operations.push(Operation::ControlledBasisPermutation {
            control,
            targets: targets.to_vec(),
            permutation: permutation.to_vec(),
        });
        Ok(())
    }

    pub fn controlled_modular_multiply(
        &mut self,
        control: usize,
        targets: &[usize],
        multiplier: u64,
        modulus: u64,
    ) -> Result<(), QuantumError> {
        self.validate_qubit(control)?;
        self.validate_qubits(targets)?;
        if targets.contains(&control) {
            return Err(QuantumError::DuplicateQubit {
                q1: control,
                q2: control,
            });
        }

        self.apply_controlled_modular_multiply_unchecked(control, targets, multiplier, modulus)?;
        self.operations.push(Operation::ControlledModularMultiply {
            control,
            targets: targets.to_vec(),
            multiplier,
            modulus,
        });
        Ok(())
    }

    pub fn controlled_modular_multiply_power(
        &mut self,
        control: usize,
        targets: &[usize],
        base: u64,
        power: u64,
        modulus: u64,
    ) -> Result<(), QuantumError> {
        let multiplier = mod_pow(base, power, modulus)?;
        self.validate_qubit(control)?;
        self.validate_qubits(targets)?;
        if targets.contains(&control) {
            return Err(QuantumError::DuplicateQubit {
                q1: control,
                q2: control,
            });
        }

        self.apply_controlled_modular_multiply_unchecked(control, targets, multiplier, modulus)?;
        self.operations
            .push(Operation::ControlledModularMultiplyPower {
                control,
                targets: targets.to_vec(),
                base,
                power,
                modulus,
            });
        Ok(())
    }

    pub fn modular_exponentiation(
        &mut self,
        controls: &[usize],
        targets: &[usize],
        base: u64,
        modulus: u64,
    ) -> Result<(), QuantumError> {
        self.validate_qubits(controls)?;
        self.validate_qubits(targets)?;
        for &control in controls {
            if targets.contains(&control) {
                return Err(QuantumError::DuplicateQubit {
                    q1: control,
                    q2: control,
                });
            }
        }

        for (power_index, &control) in controls.iter().enumerate() {
            let power = 1u64
                .checked_shl(power_index as u32)
                .ok_or(QuantumError::InvalidArithmeticInput)?;
            let multiplier = mod_pow(base, power, modulus)?;
            self.apply_controlled_modular_multiply_unchecked(
                control, targets, multiplier, modulus,
            )?;
        }

        self.operations.push(Operation::ModularExponentiation {
            controls: controls.to_vec(),
            targets: targets.to_vec(),
            base,
            modulus,
        });
        Ok(())
    }

    pub fn apply_single_qubit_gate(
        &mut self,
        matrix: [[Complex64; 2]; 2],
        target: usize,
    ) -> Result<(), QuantumError> {
        self.validate_qubit(target)?;
        if matrix
            .iter()
            .flatten()
            .any(|value| !value.re.is_finite() || !value.im.is_finite())
        {
            return Err(QuantumError::InvalidMatrix);
        }

        let target_mask = 1usize << target;
        for base in (0..self.state.len()).filter(|index| *index & target_mask == 0) {
            let one_index = base | target_mask;
            let zero_amp = self.state[base];
            let one_amp = self.state[one_index];

            self.state[base] = matrix[0][0] * zero_amp + matrix[0][1] * one_amp;
            self.state[one_index] = matrix[1][0] * zero_amp + matrix[1][1] * one_amp;
        }

        Ok(())
    }

    pub fn measure(&mut self, target: usize) -> Result<u8, QuantumError> {
        crate::measurement::measure(self, target)
    }

    pub fn measure_all(&mut self) -> Result<String, QuantumError> {
        crate::measurement::measure_all(self)
    }

    pub fn state(&self) -> &[Complex64] {
        &self.state
    }

    pub fn probabilities(&self, threshold: f64) -> Vec<(String, f64)> {
        self.state
            .iter()
            .enumerate()
            .filter_map(|(index, amplitude)| {
                let probability = amplitude.norm_sqr();
                (probability > threshold)
                    .then(|| (basis_label(index, self.num_qubits), probability))
            })
            .collect()
    }

    pub fn print_state(&self, threshold: f64) {
        for (index, amplitude) in self.state.iter().enumerate() {
            if amplitude.norm_sqr() > threshold {
                println!("|{}>: {}", basis_label(index, self.num_qubits), amplitude);
            }
        }
    }

    pub fn norm(&self) -> f64 {
        self.state
            .iter()
            .map(|amplitude| amplitude.norm_sqr())
            .sum()
    }

    pub fn assert_normalized(&self, tolerance: f64) -> Result<(), QuantumError> {
        let norm = self.norm();
        if (norm - 1.0).abs() <= tolerance {
            Ok(())
        } else {
            Err(QuantumError::StateNotNormalized { norm })
        }
    }

    pub fn operations(&self) -> &[Operation] {
        &self.operations
    }

    pub fn num_qubits(&self) -> usize {
        self.num_qubits
    }

    pub(crate) fn state_mut(&mut self) -> &mut [Complex64] {
        &mut self.state
    }

    pub(crate) fn push_operation(&mut self, operation: Operation) {
        self.operations.push(operation);
    }

    pub(crate) fn validate_qubit(&self, index: usize) -> Result<(), QuantumError> {
        if index < self.num_qubits {
            Ok(())
        } else {
            Err(QuantumError::InvalidQubit {
                index,
                num_qubits: self.num_qubits,
            })
        }
    }

    fn validate_pair(&self, q1: usize, q2: usize) -> Result<(), QuantumError> {
        self.validate_qubit(q1)?;
        self.validate_qubit(q2)?;
        if q1 == q2 {
            Err(QuantumError::DuplicateQubit { q1, q2 })
        } else {
            Ok(())
        }
    }

    fn state_len(num_qubits: usize) -> Result<usize, QuantumError> {
        if num_qubits == 0 || num_qubits >= usize::BITS as usize {
            return Err(QuantumError::InvalidNumQubits);
        }

        1usize
            .checked_shl(num_qubits as u32)
            .ok_or(QuantumError::InvalidNumQubits)
    }

    fn validate_qubits(&self, qubits: &[usize]) -> Result<(), QuantumError> {
        if qubits.is_empty() {
            return Err(QuantumError::InvalidNumQubits);
        }

        for (position, &qubit) in qubits.iter().enumerate() {
            self.validate_qubit(qubit)?;
            if let Some(&duplicate) = qubits[..position].iter().find(|&&seen| seen == qubit) {
                return Err(QuantumError::DuplicateQubit {
                    q1: duplicate,
                    q2: qubit,
                });
            }
        }

        Ok(())
    }

    fn validate_permutation(
        &self,
        num_targets: usize,
        permutation: &[usize],
    ) -> Result<(), QuantumError> {
        let len = 1usize
            .checked_shl(num_targets as u32)
            .ok_or(QuantumError::InvalidPermutation)?;
        if permutation.len() != len {
            return Err(QuantumError::InvalidPermutation);
        }

        let mut seen = vec![false; len];
        for &value in permutation {
            if value >= len || seen[value] {
                return Err(QuantumError::InvalidPermutation);
            }
            seen[value] = true;
        }

        Ok(())
    }

    fn apply_controlled_phase_unchecked(&mut self, control: usize, target: usize, theta: f64) {
        let phase = Complex64::from_polar(1.0, theta);
        let control_mask = 1usize << control;
        let target_mask = 1usize << target;

        for (index, amplitude) in self.state.iter_mut().enumerate() {
            if index & control_mask != 0 && index & target_mask != 0 {
                *amplitude *= phase;
            }
        }
    }

    fn apply_controlled_basis_permutation_unchecked(
        &mut self,
        control: usize,
        targets: &[usize],
        permutation: &[usize],
    ) {
        let control_mask = 1usize << control;
        let mut next_state = vec![Complex64::new(0.0, 0.0); self.state.len()];

        for (index, &amplitude) in self.state.iter().enumerate() {
            let destination = if index & control_mask == 0 {
                index
            } else {
                let target_value = Self::extract_register_value(index, targets);
                let mapped_value = permutation[target_value];
                Self::replace_register_value(index, targets, mapped_value)
            };
            next_state[destination] += amplitude;
        }

        self.state = next_state;
    }

    fn apply_controlled_modular_multiply_unchecked(
        &mut self,
        control: usize,
        targets: &[usize],
        multiplier: u64,
        modulus: u64,
    ) -> Result<(), QuantumError> {
        let register_len = 1usize
            .checked_shl(targets.len() as u32)
            .ok_or(QuantumError::InvalidNumQubits)?;
        let modulus_usize =
            usize::try_from(modulus).map_err(|_| QuantumError::InvalidArithmeticInput)?;

        if modulus < 2 || modulus_usize > register_len || gcd(multiplier, modulus) != 1 {
            return Err(QuantumError::InvalidArithmeticInput);
        }

        let mut permutation: Vec<usize> = (0..register_len).collect();
        for (value, mapped) in permutation.iter_mut().enumerate().take(modulus_usize) {
            *mapped = ((multiplier as u128 * value as u128) % modulus as u128) as usize;
        }
        self.validate_permutation(targets.len(), &permutation)?;

        self.apply_controlled_basis_permutation_unchecked(control, targets, &permutation);
        Ok(())
    }

    fn reverse_qubit_order_unchecked(&mut self, qubits: &[usize]) {
        for index in 0..(qubits.len() / 2) {
            self.swap_unchecked(qubits[index], qubits[qubits.len() - 1 - index]);
        }
    }

    fn swap_unchecked(&mut self, q1: usize, q2: usize) {
        let q1_mask = 1usize << q1;
        let q2_mask = 1usize << q2;

        for index in 0..self.state.len() {
            let q1_bit = index & q1_mask != 0;
            let q2_bit = index & q2_mask != 0;
            if !q1_bit && q2_bit {
                let swapped = index ^ q1_mask ^ q2_mask;
                self.state.swap(index, swapped);
            }
        }
    }

    fn extract_register_value(index: usize, targets: &[usize]) -> usize {
        targets
            .iter()
            .enumerate()
            .fold(0usize, |value, (position, &qubit)| {
                value | (((index >> qubit) & 1) << position)
            })
    }

    fn replace_register_value(index: usize, targets: &[usize], value: usize) -> usize {
        targets
            .iter()
            .enumerate()
            .fold(index, |updated, (position, &qubit)| {
                let mask = 1usize << qubit;
                if (value >> position) & 1 == 1 {
                    updated | mask
                } else {
                    updated & !mask
                }
            })
    }
}
