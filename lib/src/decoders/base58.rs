use std::collections::HashMap;

const BASE58_ALPHABET: &[u8] = b"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";

pub fn decode_base58(input: &str) -> Result<Vec<u8>, &'static str> {
    // Create a map from characters to their positions in the Base58 alphabet
    let base58_map: HashMap<u8, u32> = BASE58_ALPHABET
        .iter()
        .enumerate()
        .map(|(i, &c)| (c, i as u32))
        .collect();

    let mut output = Vec::new();
    let mut buffer = 0u64;
    let mut bits_collected = 0;

    for &byte in input.as_bytes() {
        let value = base58_map
            .get(&byte)
            .ok_or("Invalid Base58 character")? as &u32;

        buffer = (buffer * 58) + (*value as u64);
        bits_collected += 58;

        while bits_collected >= 8 {
            bits_collected -= 8;
            output.push((buffer >> bits_collected) as u8);
        }
    }

    Ok(output)
}