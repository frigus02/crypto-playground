use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::prelude::BufRead;

const NOT_A_LETTER_PENALTY: f64 = 5.0;

pub fn score(base_frequency_map: &HashMap<u8, f64>, bytes: &[u8]) -> f64 {
    let byte_frequency = get_byte_frequency_map(bytes);
    let differences: Vec<f64> = byte_frequency
        .iter()
        .map(|(letter, frequency)| {
            base_frequency_map
                .get(letter)
                .map_or(frequency * NOT_A_LETTER_PENALTY, |base_frequency| {
                    (frequency - base_frequency).abs()
                })
        }).collect();

    return differences.iter().sum::<f64>() / differences.len() as f64;
}

pub fn get_en_letter_frequency_map() -> Result<HashMap<u8, f64>, io::Error> {
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

fn get_byte_frequency_map(bytes: &[u8]) -> HashMap<u8, f64> {
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
