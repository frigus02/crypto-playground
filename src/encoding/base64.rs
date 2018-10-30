// https://en.wikipedia.org/wiki/Base64
// https://www.asciitable.com/

// 4 bytes --> 6 digits
// 1 digit --> 6 bits

pub fn encode(bytes: &[u8]) -> String {
    let bytes_len = bytes.len();

    let base64_bytes_len = (bytes_len as f64 * 8.0 / 6.0).ceil() as usize;
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

        base64_bytes.push(encode_value(value));
    }

    return String::from_utf8(base64_bytes).unwrap();
}

fn encode_value(value: u8) -> u8 {
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

pub fn decode(base64: &str) -> Vec<u8> {
    let base64_bytes = base64.as_bytes();
    let base64_bytes_len = base64_bytes.len();

    let bytes_len = (base64_bytes_len as f64 * 6.0 / 8.0).ceil() as usize;
    let mut bytes = Vec::with_capacity(bytes_len);

    for i in 0..bytes_len {
        let first_index = (i as f64 / 6.0 * 8.0).floor() as usize;
        let mut part1 = decode_value(base64_bytes[first_index]);
        let mut part2 = if first_index + 1 < base64_bytes_len {
            decode_value(base64_bytes[first_index + 1])
        } else {
            0
        };

        if i % 3 == 0 {
            part1 = part1 << 2;
            part2 = part2 >> 4;
        } else if i % 3 == 1 {
            part1 = part1 << 4;
            part2 = part2 >> 2;
        } else {
            part1 = part1 << 6;
        }

        bytes.push(part1 + part2);
    }

    return bytes;
}

fn decode_value(value: u8) -> u8 {
    return if value >= 65 && value <= 90 {
        value - 65 // "A" - "Z"
    } else if value >= 97 && value <= 122 {
        value - 71 // "a" - "z"
    } else if value >= 48 && value <= 57 {
        value + 4 // "0" - "9"
    } else if value == 43 {
        62 // "+"
    } else if value == 47 {
        63 // "/"
    } else {
        panic!("Invalid base64 value: {}", value);
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_test() {
        let bytes = vec![73, 39, 109];
        let base64 = encode(bytes.as_slice());
        assert_eq!("SSdt", base64);
    }

    #[test]
    fn decode_test() {
        let base64 = "SSdt";
        let bytes = decode(base64);
        assert_eq!(vec![73, 39, 109], bytes);
    }
}
