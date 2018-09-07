// https://en.wikipedia.org/wiki/Hexadecimal
// https://www.asciitable.com/

// 1 byte --> 2 digits, 4 bytes --> 8 digits
// 1 digit --> 4 bits

pub fn decode(hex: &str) -> Vec<u8> {
    let hex_bytes = hex.as_bytes();
    let hex_bytes_len = hex_bytes.len();

    let bytes_len = ((hex_bytes_len as f64) * 4.0 / 8.0).ceil() as usize;
    let mut bytes = Vec::with_capacity(bytes_len);

    for i in 0..bytes_len {
        let part1 = decode_value(hex_bytes[i * 2]) << 4;
        let part2 = if i * 2 + 1 < hex_bytes_len {
            decode_value(hex_bytes[i * 2 + 1])
        } else {
            0
        };

        bytes.push(part1 + part2);
    }

    return bytes;
}

fn decode_value(value: u8) -> u8 {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode_test() {
        let hex = "49276d";
        let bytes = decode(hex);
        assert_eq!(vec![73, 39, 109], bytes);
    }
}
