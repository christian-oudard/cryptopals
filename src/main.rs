use std::iter::repeat;
use cryptopals::*;

fn main() {
    let ciphertext = hex_to_bytes(&"1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736".to_string()).unwrap();
    // for key in 0..=255 {
    let key = 88;
        let plaintext: Vec<u8> = xor(
            &ciphertext,
            &repeat(key).take(ciphertext.len()).collect(),
        );
        if let Ok(text) = String::from_utf8(plaintext) {
            println!("{}: {}", key, text.to_string());
        }
    // }
}
