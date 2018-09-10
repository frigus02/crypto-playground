use super::super::encoding::hex;
use super::super::encoding::xor;

const ASCII_SPACE: u8 = 32;
const ASCII_NUM_0: u8 = 48;
const ASCII_NUM_9: u8 = 57;
const ASCII_UPPER_A: u8 = 65;
const ASCII_UPPER_Z: u8 = 90;
const ASCII_LOWER_A: u8 = 97;
const ASCII_LOWER_Z: u8 = 122;
const ASCII_DEL: u8 = 127;

pub struct Info {
    pub key: u8,
    pub plain_text: String,
}

pub fn run(hex: &str) -> Result<Info, &'static str> {
    let bytes = hex::decode(hex);

    for key in ASCII_SPACE..ASCII_DEL {
        let encoded_bytes = xor::encode(&bytes, &vec![key]);
        if is_ascii_char(&encoded_bytes[0])
            && is_in_ascii_range(&encoded_bytes)
            && get_ascii_char_sign_ratio(&encoded_bytes) < 0.5
            && get_ascii_char_space_ratio(&encoded_bytes) > 0.05
        {
            let plain_text = String::from_utf8(encoded_bytes).unwrap();
            return Ok(Info { key, plain_text });
        }
    }

    return Err("could not find decoding char");
}

fn is_in_ascii_range(bytes: &[u8]) -> bool {
    for byte in bytes {
        if byte >= &ASCII_DEL {
            return false;
        }
    }

    return true;
}

fn is_ascii_char(byte: &u8) -> bool {
    return (byte >= &ASCII_UPPER_A && byte <= &ASCII_UPPER_Z)
        || (byte >= &ASCII_LOWER_A && byte <= &ASCII_LOWER_Z);
}

fn is_ascii_sign(byte: &u8) -> bool {
    return byte < &ASCII_NUM_0
        || (byte > &ASCII_NUM_9 && byte < &ASCII_UPPER_A)
        || (byte > &ASCII_UPPER_Z && byte < &ASCII_LOWER_A)
        || byte > &ASCII_LOWER_Z;
}

fn get_ascii_char_sign_ratio(bytes: &[u8]) -> f32 {
    let bytes_len = bytes.len();
    let signs = bytes.iter().filter(|x| is_ascii_sign(x)).count();
    let chars = bytes_len - signs;
    return signs as f32 / chars as f32;
}

fn get_ascii_char_space_ratio(bytes: &[u8]) -> f32 {
    let bytes_len = bytes.len();
    let spaces = bytes.iter().filter(|x| x == &&ASCII_SPACE).count();
    let chars = bytes_len - spaces;
    return spaces as f32 / chars as f32;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test() {
        let hex = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
        let result = run(hex);
        assert!(result.is_ok());

        let info = result.unwrap();
        assert_eq!('X' as u8, info.key);
        assert_eq!("Cooking MC's like a pound of bacon", info.plain_text);
    }
}
