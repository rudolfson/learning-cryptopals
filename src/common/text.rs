/// Score the text based on how closely it matches the expected frequency of letters in English.
/// The lower the score, the closer the match.
pub fn score_englishness(text: &[u8]) -> f32 {
    let len = text.len();

    // count the number of occurrences of each letter
    let mut observed_count = [0; 256];
    for c in text {
        let c = c.to_ascii_uppercase();
        observed_count[c as usize] += 1;
    }

    // Run a chi-squared test: https://en.wikipedia.org/wiki/Chi-squared_test
    //
    // The chi-squared test is used to determine whether there is a significant difference
    // between the expected frequencies and the observed frequencies of the characters.
    let mut error_score = 0.0;
    for i in 0..255 {
        let expected_count = get_expected_frequency(i) * len as f32;
        error_score +=
            (expected_count - observed_count[i as usize] as f32).powi(2) / expected_count;
    }

    error_score.sqrt()
}

fn get_expected_frequency(c: u8) -> f32 {
    let c = c.to_ascii_uppercase();
    if c.is_ascii_uppercase() {
        const FREQ_TABLE: [f32; 26] = [
            0.08167, 0.01492, 0.02782, 0.04253, 0.12702, 0.02228, // A-F
            0.02015, 0.06094, 0.06966, 0.00153, 0.00772, 0.04025, // G-L
            0.02406, 0.06749, 0.07507, 0.01929, 0.00095, 0.05987, // M-R
            0.06327, 0.09056, 0.02758, 0.00978, 0.02360, 0.00150, // S-X
            0.01974, 0.00074, // Y-Z
        ];

        let index = c - b'A';
        return FREQ_TABLE[index as usize];
    }

    match c as char {
        ' ' => 0.15,
        '\'' => 0.01,
        ',' => 0.01,
        '.' => 0.01,
        '!' => 0.01,
        '?' => 0.01,
        _ => 0.0001,
    }
}

fn hamming_distance(a: &[u8], b: &[u8]) -> u32 {
    assert_eq!(a.len(), b.len());
    a.iter()
        .zip(b.iter())
        .map(|(a, b)| a ^ b)
        .map(|x| x.count_ones())
        .sum()
}
const MAX_KEYSIZE: usize = 40;
pub fn find_key_sizes(encrypted: &[u8]) -> Vec<usize> {
    assert!(encrypted.len() >= 4, "need at least 4 bytes to do this");
    let max_keysize = MAX_KEYSIZE.min(encrypted.len() / 2);
    let mut distances: Vec<(usize, f64)> = (2..=max_keysize)
        .map(|keysize| {
            (
                keysize,
                encrypted[0..keysize].to_vec(),
                encrypted[keysize..(2 * keysize)].to_vec(),
            )
        })
        .map(|(keysize, a, b)| (keysize, hamming_distance(&a, &b) as f64))
        .map(|(keysize, distance)| (keysize, distance / (keysize as f64)))
        .collect();
    distances.sort_by(|a, b| a.1.partial_cmp(&b.1).expect("could not compare"));

    todo!();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_score() {
        assert!(
            score_englishness("my name is bob".as_bytes())
                < score_englishness("awo02h4fobsdfdb".as_bytes())
        )
    }

    #[test]
    fn test_hamming_distance() {
        assert_eq!(
            hamming_distance("this is a test".as_bytes(), "wokka wokka!!!".as_bytes()),
            37
        );
    }
}
