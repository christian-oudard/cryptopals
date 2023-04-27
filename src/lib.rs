use std::iter::zip;

#[derive(Debug)]
pub struct HexError;

fn hex_digit(c: u8) -> Result<u8, HexError> {
    match c {
        b'a'..=b'f' => Ok(c - b'a' + 10),
        b'0'..=b'9' => Ok(c - b'0'),
        _ => Err(HexError)
    }
}

pub fn hex_to_bytes(hex: &String) -> Result<Vec<u8>, HexError> {
    hex.as_bytes()
        .chunks(2)
        .map(|chunk| Ok(hex_digit(chunk[0])? << 4 | hex_digit(chunk[1])?))
        .collect()
}

const HEX_CHARS: &[u8; 16] = b"0123456789abcdef";

pub fn bytes_to_hex(bytes: &Vec<u8>) -> String {
    let mut result = Vec::new();
    for byte in bytes {
        let chars = [
            HEX_CHARS[(byte >> 4) as usize] as char,
            HEX_CHARS[(byte & 0b0000_1111) as usize] as char,
        ];
        result.extend_from_slice(&chars);
    }
    result.iter().collect()
}


#[derive(Debug)]
pub struct B64Error;

const BASE_64_CHARS: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
const BASE_64_PAD: u8 = b'=';


pub fn encode_b64(bytes: &Vec<u8>) -> String {
    let mut result: Vec<u8> = Vec::new();
    let mut padding = 0;
    for chunk in bytes.chunks(3) {
        let mut chunk = chunk.to_vec();
        padding = 3 - chunk.len();
        if chunk.len() < 3 {
            chunk.resize(3, 0x00);
        }
        let sextets = [
            chunk[0] >> 2,
            (chunk[0] & 0b0000_0011) << 4 | chunk[1] >> 4,
            (chunk[1] & 0b0000_1111) << 2 | chunk[2] >> 6,
            chunk[2] & 0b0011_1111,
        ];
        for sextet in sextets.iter() {
            result.push(BASE_64_CHARS[*sextet as usize])
        }
    }
    let len = result.len();
    for i in 0..padding {
        result[len - 1 - i] = b'=';
    }

    String::from_utf8(result).expect("Can encode string from base64 chars.")
}


fn b64_index(c: u8) -> Result<u8, B64Error> {
    match c {
        b'A'..=b'Z' => Ok(c - b'A'),
        b'a'..=b'z' => Ok(c - b'a' + 26),
        b'0'..=b'9' => Ok(c - b'0' + 52),
        b'+' => Ok(63),
        b'/' => Ok(64),
        _ => Err(B64Error)
    }
}


pub fn decode_b64(b64: &String) -> Result<Vec<u8>, B64Error> {
    let sextets: Vec<u8> = b64
        .bytes()
        .filter(|c| *c != BASE_64_PAD)
        .map(|c| Ok(b64_index(c)?))
        .collect::<Result<_, _>>()?;

    let mut result: Vec<u8> = vec![];
    let mut padding = 0;
    for chunk in sextets.as_slice().chunks(4) {
        let mut chunk = chunk.to_vec();
        padding = 4 - chunk.len();
        chunk.resize(4, 0x00);
        let bytes = [
            chunk[0] << 2 | (chunk[1] & 0b11_0000) >> 4,
            (chunk[1] & 0b00_1111) << 4 | (chunk[2] & 0b11_1100) >> 2,
            (chunk[2] & 0b00_0011) << 6 | chunk[3],
        ];
        result.extend_from_slice(&bytes);
    }
    result.truncate(result.len() - padding);
    Ok(result)
}


pub fn xor(a: &Vec<u8>, b: &Vec<u8>) -> Vec<u8> {
    assert_eq!(a.len(), b.len());
    zip(a, b).map(|(a, b)| a ^ b).collect()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex() {
        let hex = "0123456789abcdef".to_string();
        let bytes = vec![1, 35, 69, 103, 137, 171, 205, 239];
        assert_eq!(hex_to_bytes(&hex).unwrap(), bytes);
        assert_eq!(bytes_to_hex(&bytes), hex);
    }

    #[test]
    fn test_b64() {
        let bytes: Vec<u8> = (0..16).into_iter().collect();
        let b64 = "AAECAwQFBgcICQoLDA0ODw==".to_string();
        assert_eq!(encode_b64(&bytes), b64);
        assert_eq!(decode_b64(&b64).unwrap(), bytes);
    }

    #[test]
    fn test_xor() {
        let a = hex_to_bytes(&"1c0111001f010100061a024b53535009181c".to_string()).unwrap();
        let b = hex_to_bytes(&"686974207468652062756c6c277320657965".to_string()).unwrap();
        let c = hex_to_bytes(&"746865206b696420646f6e277420706c6179".to_string()).unwrap();
        assert_eq!(xor(&a, &b), c);
    }
}
