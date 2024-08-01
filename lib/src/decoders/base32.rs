const BASE32_ALPHABET: &[u8] = b"abcdefghijklmnopqrstuvwxyz234567";
const BASE32_LOOKUP: [i8; 256] = {
    let mut lookup = [-1i8; 256];
    let mut i = 0;
    while i < 32 {
        lookup[BASE32_ALPHABET[i] as usize] = i as i8;
        i += 1;
    }
    lookup
};

pub fn decode_base32(input: &str) -> Result<Vec<u8>, &'static str> {
    // Count the padding bytes
    let mut end = input.len();
    while end > 0 && input.as_bytes()[end - 1] == b'=' {
        end -= 1;
    }

    // Allocate the output
    let output_length = (end * 5 / 8) as usize;
    let mut output = vec![0u8; output_length];

    // Parse the data
    let mut bits = 0;
    let mut buffer = 0u32;
    let mut written = 0;

    for i in 0..end {
        let byte = input.as_bytes()[i];
        let value = BASE32_LOOKUP[byte as usize];
        if value == -1 {
            return Err("Non-Base32 character");
        }
        let value = value as u32;

        // Append the bits to the buffer
        buffer = (buffer << 5) | value;
        bits += 5;

        // Write out some bits if the buffer has a byte's worth
        if bits >= 8 {
            bits -= 8;
            output[written] = (buffer >> bits) as u8;
            written += 1;
        }
    }

    // Verify that we have received just enough bits
    if bits >= 5 || ((buffer << (8 - bits)) & 0xff) != 0 {
        return Err("Unexpected end of data");
    }

    Ok(output)
}