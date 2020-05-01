pub fn encode(bytes: &[u8], encoding: &[u8]) -> Vec<u8> {
    let bytes_len = bytes.len();
    let encoding_len = encoding.len();

    let mut encoded_bytes = Vec::with_capacity(bytes_len);
    for i in 0..bytes_len {
        encoded_bytes.push(bytes[i] ^ encoding[i % encoding_len]);
    }

    encoded_bytes
}

#[cfg(test)]
mod tests {
    use super::super::hex;
    use super::*;

    #[test]
    fn encode_fixed_length_test() {
        let input = hex::decode("1c0111001f010100061a024b53535009181c");
        let encoding = hex::decode("686974207468652062756c6c277320657965");
        let result = hex::encode(&encode(&input, &encoding));
        assert_eq!("746865206b696420646f6e277420706c6179", result);
    }

    #[test]
    fn encode_single_byte_test() {
        let input = hex::decode("1c0111001f010100061a024b53535009181c");
        let encoding = [104u8];
        let result = hex::encode(&encode(&input, &encoding));
        assert_eq!("74697968776969686e726a233b3b38617074", result);
    }

    #[test]
    fn encode_repeating_key_test() {
        let input = hex::decode("1c0111001f010100061a024b53535009181c");
        let encoding = [104u8, 105u8, 116u8];
        let result = hex::encode(&encode(&input, &encoding));
        assert_eq!("746865687675696972726b3f3b3a24617168", result);
    }
}
