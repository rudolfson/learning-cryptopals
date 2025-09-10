use base64::{Engine, prelude::BASE64_STANDARD_NO_PAD};

use crate::common::{self, xor::brute_force_single_byte_xor_cipher};

pub fn challenge1() {
    println!("Running challenge 1");
    let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    let expected = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";

    let bytes = hex::decode(input.as_bytes()).unwrap();
    let result = BASE64_STANDARD_NO_PAD.encode(bytes);

    assert_eq!(expected, result);
}

pub fn challenge2() {
    println!("Running challenge 2");
    let a = "1c0111001f010100061a024b53535009181c";
    let b = "686974207468652062756c6c277320657965";

    let xored = common::xor::fixed_xor_hex(a, b);

    assert_eq!("746865206b696420646f6e277420706c6179", xored);
}

pub fn challenge3() {
    println!("Running challenge 3");
    let encrypted = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    let data = hex::decode(encrypted).expect("could not convert hex string to bytes");
    match brute_force_single_byte_xor_cipher(&data) {
        Some((key, decrypted, _score)) => println!(
            "decrypted message: {}\nkey was: {}",
            String::from_utf8_lossy(&decrypted).into_owned(),
            key as char
        ),
        None => println!("something went wrong"),
    }
}
