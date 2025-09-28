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

pub fn repeating_key_xor(text: &str, key: &mut RepeatingKey) -> String {
    let xored: Vec<u8> = text.bytes().map(|b| b ^ key.next_key()).collect();
    hex::encode(xored)
}

pub struct RepeatingKey {
    current_idx: usize,
    key: Vec<u8>,
}

impl RepeatingKey {
    pub fn from(key: &str) -> Self {
        Self {
            current_idx: 0,
            key: key.bytes().collect(),
        }
    }

    pub fn next_key(&mut self) -> u8 {
        let result = self.key[self.current_idx];
        self.current_idx += 1;
        if self.current_idx >= self.key.len() {
            self.current_idx = 0;
        }
        result
    }
}

#[cfg(test)]
mod test {
    use crate::common::xor::repeating_key_xor;

    use super::RepeatingKey;

    #[test]
    fn repeating_key() {
        let mut rk = RepeatingKey::from("ABC");
        assert_eq!(rk.next_key(), b'A');
        assert_eq!(rk.next_key(), b'B');
        assert_eq!(rk.next_key(), b'C');
        assert_eq!(rk.next_key(), b'A');
        assert_eq!(rk.next_key(), b'B');
        assert_eq!(rk.current_idx, 2);
    }

    #[test]
    fn test_repeating_key_xor() {
        let text = "Burning 'em, if you ain't quick and nimble
I go crazy when I hear a cymbal";
        let mut key = RepeatingKey::from("ICE");
        let result = repeating_key_xor(text, &mut key);

        assert_eq!(
            result,
            "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f"
        );
    }
}
