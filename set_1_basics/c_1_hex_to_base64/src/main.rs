use base64::{Engine, prelude::BASE64_STANDARD_NO_PAD};

fn hex_to_base64(hex_str: &str) -> String {
    let bytes = hex::decode(hex_str).unwrap();
    BASE64_STANDARD_NO_PAD.encode(bytes)
}

fn main() {
    let input = String::from(
        "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d",
    );
    println!("hex to base64 → {}", hex_to_base64(&input));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn challenge_1_hex_to_base64() {
        let input = String::from(
            "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d",
        );
        let expected =
            String::from("SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t");
        assert_eq!(hex_to_base64(&input), expected);
    }
}
