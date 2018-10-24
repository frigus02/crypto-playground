pub fn get(bytes_a: &[u8], bytes_b: &[u8]) -> Result<u8, String> {
    if bytes_a.len() != bytes_b.len() {
        return Err(String::from("byte slices don't have the same length"));
    }

    let bytes = bytes_a.iter().zip(bytes_b);
    let mut distance: u8 = 0;
    for (byte_a, byte_b) in bytes {
        for shift in 0..8 {
            let mask = 1 << shift;
            if byte_a & mask != byte_b & mask {
                distance += 1;
            }
        }
    }

    return Ok(distance);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_test() {
        let bytes_a = "this is a test";
        let bytes_b = "wokka wokka!!!";
        let hemming_distance = get(bytes_a.as_bytes(), bytes_b.as_bytes());
        assert!(hemming_distance.is_ok());
        assert_eq!(37, hemming_distance.unwrap());
    }
}
