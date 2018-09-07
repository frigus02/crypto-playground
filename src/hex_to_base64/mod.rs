// https://en.wikipedia.org/wiki/Hexadecimal
// https://en.wikipedia.org/wiki/Base64
// https://www.asciitable.com/

// hex: 1 byte --> 2 digits, 4 bytes --> 8 digits
// hex: 1 digit --> 4 bits

// base64: 4 bytes --> 6 digits
// base64: 1 digit --> 6 bits

pub fn run(hex: &str) -> String {
    let bytes = decode_hex(hex);

    return encode_base64(bytes.as_slice());
}

fn decode_hex(hex: &str) -> Vec<u8> {
    let hex_bytes = hex.as_bytes();
    let hex_bytes_len = hex_bytes.len();

    let bytes_len = ((hex_bytes_len as f64) * 4.0 / 8.0).ceil() as usize;
    let mut bytes = Vec::with_capacity(bytes_len);

    for i in 0..bytes_len {
        let part1 = decode_hex_value(hex_bytes[i * 2]) << 4;
        let part2 = if i * 2 + 1 < hex_bytes_len {
            decode_hex_value(hex_bytes[i * 2 + 1])
        } else {
            0
        };

        bytes.push(part1 + part2);
    }

    return bytes;
}

fn decode_hex_value(value: u8) -> u8 {
    return if value >= 48 && value <= 57 {
        value - 48 // "0" - "9"
    } else if value >= 65 && value <= 70 {
        value - 55 // "A" - "F"
    } else if value >= 97 && value <= 102 {
        value - 87 // "a" - "f"
    } else {
        panic!("Invalid hex value: {}", value);
    };
}

fn encode_base64(bytes: &[u8]) -> String {
    let bytes_len = bytes.len();

    let base64_bytes_len = ((bytes_len as f64) * 8.0 / 6.0).ceil() as usize;
    let mut base64_bytes = Vec::with_capacity(base64_bytes_len);

    for i in 0..base64_bytes_len {
        let used_bits = i * 6;
        let mut value = 0;
        for bit_index in 0..6 {
            let pick_from_index = used_bits + (5 - bit_index);
            let byte_index = pick_from_index / 8;
            let byte_offset = 7 - (pick_from_index % 8);
            if byte_index < bytes_len {
                let bit = (bytes[byte_index] >> byte_offset) & 1;
                value += bit << bit_index;
            }
        }

        base64_bytes.push(encode_base64_value(value));
    }

    return String::from_utf8(base64_bytes).unwrap();
}

fn encode_base64_value(value: u8) -> u8 {
    return if value <= 25 {
        value + 65 // "A" - "Z"
    } else if value >= 26 && value <= 51 {
        value + 71 // "a" - "z"
    } else if value >= 52 && value <= 61 {
        value - 4 // "0" - "9"
    } else if value == 62 {
        43 // "+"
    } else if value == 63 {
        47 // "/"
    } else {
        panic!("Invalid base64 value: {}", value);
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode_hex_test() {
        let hex = "49276d";
        let bytes = decode_hex(hex);
        assert_eq!(vec![73, 39, 109], bytes);
    }

    #[test]
    fn encode_base64_test() {
        let bytes = vec![73, 39, 109];
        let base64 = encode_base64(bytes.as_slice());
        assert_eq!("SSdt", base64);
    }

    #[test]
    fn run_test() {
        let hex = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
        let base64 = run(hex);
        assert_eq!(
            "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t",
            base64
        );
    }
}
