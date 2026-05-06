pub fn basis_label(index: usize, num_qubits: usize) -> String {
    (0..num_qubits)
        .rev()
        .map(|bit| if (index >> bit) & 1 == 1 { '1' } else { '0' })
        .collect()
}
