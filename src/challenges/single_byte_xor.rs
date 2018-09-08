use super::super::encoding::hex;

pub fn run(hex: &str, encoding_byte: u8) -> String {
    let bytes = hex::decode(hex);
    let bytes_len = bytes.len();

    let mut encoded_bytes = Vec::with_capacity(bytes_len);
    for i in 0..bytes_len {
        encoded_bytes.push(bytes[i] ^ encoding_byte);
    }

    return hex::encode(encoded_bytes.as_slice());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test() {
        let hex = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
        let encoding_byte = 88u8; // "X"
        let encoded = run(hex, encoding_byte);

        let plain_text = String::from_utf8(hex::decode(&encoded)).unwrap();

        assert_eq!("Cooking MC's like a pound of bacon", plain_text);
    }
}
