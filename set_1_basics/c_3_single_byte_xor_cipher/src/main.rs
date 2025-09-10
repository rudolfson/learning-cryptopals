use std::{ops::Div, string::FromUtf8Error};

///
/// a ^ 0 = 0 → a = 0
/// a ^ 0 = 1 → a = 1
/// a ^ 1 = 0 → a = 1
/// a ^ 1 = 1 → a = 0
///
fn decrypt_single_byte_xor_cipher(
    encrypted_hex_str: &str,
    key: u8,
) -> Result<String, FromUtf8Error> {
    let decrypted_bytes: Vec<u8> = hex::decode(encrypted_hex_str)
        .unwrap()
        .iter()
        .map(|b| b ^ key)
        .collect();
    String::from_utf8(decrypted_bytes)
}

fn find_key(encrypted_hex_str: &str) -> (char, String) {
    let (key, text, _) = ('A'..='Z')
        .map(|key| {
            (
                key,
                decrypt_single_byte_xor_cipher(encrypted_hex_str, key as u8),
            )
        })
        .filter_map(|(key, result)| match result {
            Ok(text) => {
                let chi = chi2(&text);
                Some((key, text, chi))
            }
            Err(_) => None,
        })
        //.for_each(|(key, text, chi)| println!("{key} → {text} ; chi {chi}"));
        .min_by(|a, b| a.2.partial_cmp(&b.2).unwrap())
        .unwrap();
    (key, text)
}

fn main() {
    // challenge 3
    let (key, text) =
        find_key("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736");
    println!("i guess the key is {key} and the decrypted text is {text}");
}

// http://en.algoritmy.net/article/40379/Letter-frequency-English
const ENGLISH_FREQ: [f64; 26] = [
    0.08167, 0.01492, 0.02782, 0.04253, 0.12702, 0.02228, 0.02015, // A-G
    0.06094, 0.06966, 0.00153, 0.00772, 0.04025, 0.02406, 0.06749, // H-N
    0.07507, 0.01929, 0.00095, 0.05987, 0.06327, 0.09056, 0.02758, // O-U
    0.00978, 0.02360, 0.00150, 0.01974, 0.00074, // V-Z
];

fn chi2(text: &str) -> f64 {
    let mut counts: Vec<u32> = vec![0; 26];

    for c in text.chars() {
        if c.is_ascii_alphabetic() {
            let idx = c.to_ascii_lowercase() as u8 - b'a';
            if let Some(count) = counts.get_mut(idx as usize) {
                *count += 1;
            }
        }
    }

    let len: f64 = counts.iter().sum::<u32>() as f64;
    counts
        .iter()
        .zip(ENGLISH_FREQ.iter())
        .map(|(count, freq)| {
            let observed_count = *count as f64;
            let expected_count: f64 = len * freq;
            let diff = observed_count - expected_count;
            diff.powi(2).div(expected_count)
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_count_chars() {
        let counts = count_chars("abee");
        assert_eq!(counts.get(&'a'), Some(&1));
        assert_eq!(counts.get(&'b'), Some(&1));
        assert_eq!(counts.get(&'c'), None);
        assert_eq!(counts.get(&'e'), Some(&2));
    }
}
