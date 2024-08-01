use serde::Deserialize;
use serde::de::{self, Deserializer};

const BASE64_CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
const BASE64_PAD: u8 = b'=';

fn decode_base64(input: &str) -> Result<Vec<u8>, &'static str> {
    if input.len() % 4 != 0 {
        return Err("Invalid base64 length");
    }

    let mut output = Vec::new();
    let mut buffer = 0u32;
    let mut bits_collected = 0;

    for &byte in input.as_bytes() {
        if byte == BASE64_PAD {
            break;
        }

        let value = BASE64_CHARSET.iter().position(|&c| c == byte)
            .ok_or("Invalid base64 character")? as u32;

        buffer = (buffer << 6) | value;
        bits_collected += 6;

        if bits_collected >= 8 {
            bits_collected -= 8;
            output.push((buffer >> bits_collected) as u8 & 0xFF);
        }
    }

    Ok(output)
}

pub fn deserialize_base64<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    decode_base64(&s).map_err(de::Error::custom)
}