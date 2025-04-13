pub fn extract_final_grid(full_data_seq: Vec<u8>) -> Vec<Vec<u8>> {
    let grid: Vec<Vec<u8>> = full_data_seq
        .chunks(32)
        .map(|seq| seq[..10].to_vec())
        .take(16)
        .collect();
    let head = grid.iter().take(8);
    let tail = grid.iter().rev().take(8);
    let mixed = head.chain(tail).cloned().collect();
    let transposed = transpose(&mixed);
    transposed
}

pub fn transpose(matrix: &Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    (0..matrix[0].len())
        .map(|i| matrix.iter().map(|row| row[i]).collect())
        .collect()
}
