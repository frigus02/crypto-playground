use super::super::analysis::letter_frequency::{get_en_letter_frequency_map, score};
use super::super::encoding::hex;
use super::super::encoding::xor;
use std::cmp::Ordering;

const ASCII_SPACE: u8 = 32;
const ASCII_DEL: u8 = 127;

pub struct Info {
    pub key: u8,
    pub plain_text: String,
    pub score: f64,
}

pub fn run(hex: &str) -> Result<Info, String> {
    let en_letter_frequency_map = get_en_letter_frequency_map()
        .map_err(|err| format!("reading letter_frequency_en failed: {}", err))?;
    let bytes = hex::decode(hex);
    let key_range = ASCII_SPACE..ASCII_DEL;

    let (key, decoded_bytes, score) = key_range
        .into_iter()
        .map(|key| {
            let decoded_bytes = xor::encode(&bytes, &vec![key]);
            let score = score(&en_letter_frequency_map, &decoded_bytes);

            (key, decoded_bytes, score)
        }).min_by(|(_, _, score_x), (_, _, score_y)| {
            score_x.partial_cmp(score_y).unwrap_or(Ordering::Greater)
        }).ok_or(String::from("score collection empty"))?;

    return match String::from_utf8(decoded_bytes) {
        Ok(plain_text) => Ok(Info {
            key,
            plain_text,
            score,
        }),
        Err(err) => Err(format!("decoding failed: {}", err)),
    };
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
