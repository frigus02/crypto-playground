use super::super::encoding::hex;
use super::super::encoding::xor;

pub fn run(hex1: &str, hex2: &str) -> String {
    if hex1.len() != hex2.len() {
        panic!("Length are not equal ({} != {})!", hex1.len(), hex2.len());
    }

    let bytes1 = hex::decode(hex1);
    let bytes2 = hex::decode(hex2);
    let result = xor::encode(&bytes1, &bytes2);

    return hex::encode(&result);
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
