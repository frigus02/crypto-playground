use super::super::encoding::hex;

pub fn run(hex1: &str, hex2: &str) -> String {
    if hex1.len() != hex2.len() {
        panic!("Length are not equal ({} != {})!", hex1.len(), hex2.len());
    }

    let bytes1 = hex::decode(hex1);
    let bytes2 = hex::decode(hex2);
    let bytes_len = bytes1.len();

    let mut bytes3 = Vec::with_capacity(bytes_len);
    for i in 0..bytes_len {
        bytes3.push(bytes1[i] ^ bytes2[i]);
    }

    return hex::encode(bytes3.as_slice());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test() {
        let hex1 = "1c0111001f010100061a024b53535009181c";
        let hex2 = "686974207468652062756c6c277320657965";
        let hex3 = run(hex1, hex2);
        assert_eq!("746865206b696420646f6e277420706c6179", hex3);
    }
}
