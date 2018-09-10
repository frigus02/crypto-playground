use super::super::encoding::hex;
use super::super::encoding::xor;

pub fn run(hex: &str, encoding_byte: u8) -> String {
    let bytes = hex::decode(hex);
    let result = xor::encode(&bytes, &vec![encoding_byte]);

    return hex::encode(&result);
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
