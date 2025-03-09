pub fn extract_sub_data_seq(full_data_seq: Vec<u8>) -> Vec<u8> {
    full_data_seq
        .chunks(16)
        .map(|seq| seq[..10].to_vec())
        .flatten()
        .collect()
}
