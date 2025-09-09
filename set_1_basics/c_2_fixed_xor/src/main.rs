fn fixed_xor(hex_str1: &str, hex_str2: &str) -> String {
    let bytes1 = hex::decode(hex_str1).unwrap();
    let bytes2 = hex::decode(hex_str2).unwrap();
    assert_eq!(bytes1.len(), bytes2.len());
    let xored_bytes: Vec<u8> = bytes1
        .iter()
        .zip(bytes2.iter())
        .map(|(a, b)| a ^ b)
        .collect();
    hex::encode(xored_bytes)
}

fn main() {
    let input1 = String::from("1c0111001f010100061a024b53535009181c");
    let input2 = String::from("686974207468652062756c6c277320657965");
    println!("fixed xor -> {}", fixed_xor(&input1, &input2));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn challange_2_fixed_xor() {
        let input1 = String::from("1c0111001f010100061a024b53535009181c");
        let input2 = String::from("686974207468652062756c6c277320657965");
        let expected = String::from("746865206b696420646f6e277420706c6179");
        assert_eq!(fixed_xor(&input1, &input2), expected);
    }
}
