use crate::common;

pub fn fixed_xor_hex(a: &str, b: &str) -> String {
    let a = hex::decode(a).unwrap();
    let b = hex::decode(b).unwrap();
    let xored = fixed_xor_bytes(&a, &b);
    hex::encode(xored)
}

pub fn fixed_xor_bytes(a: &[u8], b: &[u8]) -> Vec<u8> {
    assert!(a.len() == b.len());

    std::iter::zip(a.iter(), b.iter())
        .map(|(a, b)| a ^ b)
        .collect()
}

pub fn brute_force_single_byte_xor_cipher(encrypted: &[u8]) -> Option<(u8, Vec<u8>, f32)> {
    let mut min_score = 1_000_000f32;
    let mut found_result: Vec<u8> = vec![];
    let mut found_key = 0u8;
    for key in 0u8..=255u8 {
        let keys = vec![key; encrypted.len()];
        let decrypted = fixed_xor_bytes(encrypted, &keys);
        let score = common::text::score_englishness(&decrypted);
        if score < min_score {
            min_score = score;
            found_result = decrypted;
            found_key = key;
        }
    }
    match found_result.len() {
        0 => None,
        _ => Some((found_key, found_result, min_score)),
    }
}
