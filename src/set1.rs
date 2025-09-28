use std::fs::read_to_string;

use base64::{Engine, prelude::BASE64_STANDARD_NO_PAD};

use crate::common::{
    self,
    xor::{self, RepeatingKey, brute_force_single_byte_xor_cipher},
};

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

pub fn challenge4() {
    println!("Running challenge 4");
    let content = read_to_string("data/set1challenge4.txt").expect("could not read file");
    let mut min_score = f32::MAX;
    let mut found: Vec<u8> = vec![];
    for line in content.lines() {
        let data = hex::decode(line).expect("could not convert hex string to bytes");
        match brute_force_single_byte_xor_cipher(&data) {
            Some((_key, decrypted, score)) => {
                if score < min_score {
                    min_score = score;
                    found = decrypted;
                }
            }
            None => println!("something went wrong"),
        }
    }
    println!(
        "Found message: {}",
        String::from_utf8_lossy(&found).into_owned()
    );
}

pub fn challenge5() {
    println!("Running challenge 5");
    let text = "Burning 'em, if you ain't quick and nimble
I go crazy when I hear a cymbal";
    let mut key = RepeatingKey::from("ICE");
    let result = xor::repeating_key_xor(text, &mut key);

    assert_eq!(
        result,
        "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f"
    )
}

pub fn challenge6() {}
