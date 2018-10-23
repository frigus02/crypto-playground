use super::super::encoding::hex;
use super::super::encoding::xor;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::prelude::BufRead;

const ASCII_SPACE: u8 = 32;
const ASCII_DEL: u8 = 127;
const NOT_A_LETTER_PENALTY: f64 = 5.0;

pub struct Info {
    pub key: u8,
    pub plain_text: String,
    pub score: f64,
}

pub fn run(hex: &str) -> Result<Info, String> {
    let en_letter_frequency = get_en_letter_frequency()
        .map_err(|err| format!("reading letter_frequency_en failed: {}", err))?;
    let bytes = hex::decode(hex);
    let key_range = ASCII_SPACE..ASCII_DEL;

    let (score, key) = key_range
        .into_iter()
        .map(|key| {
            let encoded_bytes = xor::encode(&bytes, &vec![key]);

            let byte_frequency = get_byte_frequency(&encoded_bytes);
            let differences: Vec<f64> = byte_frequency
                .iter()
                .map(|(letter, frequency)| {
                    en_letter_frequency
                        .get(letter)
                        .map_or(frequency * NOT_A_LETTER_PENALTY, |en_frequency| {
                            (frequency - en_frequency).abs()
                        })
                }).collect();

            let score = differences.iter().sum::<f64>() / differences.len() as f64;

            (score, key)
        }).min_by(|(score_x, _), (score_y, _)| {
            score_x.partial_cmp(score_y).unwrap_or(Ordering::Greater)
        }).ok_or(String::from("score collection empty"))?;

    let decoded_bytes = xor::encode(&bytes, &vec![key]);
    return match String::from_utf8(decoded_bytes) {
        Ok(plain_text) => Ok(Info {
            key,
            plain_text,
            score,
        }),
        Err(err) => Err(format!("decoding failed: {}", err)),
    };
}

fn get_en_letter_frequency() -> Result<HashMap<u8, f64>, io::Error> {
    let file = File::open("res/letter_frequency_en.txt")?;
    let reader = io::BufReader::new(file);
    let mut scores = HashMap::new();

    for line in reader.lines() {
        let line = line?;
        let line = line.trim();
        if line.len() == 0 {
            continue;
        }

        let parts: Vec<&str> = line.split(' ').collect();
        if parts.len() != 2 || parts[0].len() != 1 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "line does not contain space separated letter and frequency",
            ));
        }

        let letter = parts[0].as_bytes()[0];
        let frequency = parts[1]
            .parse::<f64>()
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "cannot parse frequency"))?;

        scores.insert(letter, frequency / 100.0);
    }

    return Ok(scores);
}

fn get_byte_frequency(bytes: &[u8]) -> HashMap<u8, f64> {
    let mut scores = bytes.iter().fold(HashMap::new(), |mut acc, byte| {
        acc.entry(byte.clone())
            .and_modify(|count| *count += 1.0)
            .or_insert(1.0);
        acc
    });

    let len = bytes.len() as f64;
    for frequency in scores.values_mut() {
        *frequency = *frequency / len;
    }

    return scores;
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
