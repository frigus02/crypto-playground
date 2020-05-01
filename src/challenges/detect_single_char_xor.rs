use super::super::analysis::letter_frequency::{get_best_xor_score, get_en_letter_frequency_map};
use super::super::encoding::hex;

pub struct Info {
    pub key: u8,
    pub plain_text: String,
    pub score: f64,
}

pub fn run(hex: &str) -> Result<Info, String> {
    let en_letter_frequency_map = get_en_letter_frequency_map()
        .map_err(|err| format!("reading letter_frequency_en failed: {}", err))?;
    let bytes = hex::decode(hex);
    let score = get_best_xor_score(&en_letter_frequency_map, &bytes)?;

    match String::from_utf8(score.decoded_bytes) {
        Ok(plain_text) => Ok(Info {
            key: score.key,
            plain_text,
            score: score.score,
        }),
        Err(err) => Err(format!("decoding failed: {}", err)),
    }
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
        assert_eq!(b'X', info.key);
        assert_eq!("Cooking MC's like a pound of bacon", info.plain_text);
    }
}
