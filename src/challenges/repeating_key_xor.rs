use super::super::encoding::hex;
use super::super::encoding::xor;

pub fn run(plain_text: &str, encoding_bytes: &[u8]) -> String {
    let bytes = plain_text.as_bytes();
    let result = xor::encode(&bytes, encoding_bytes);

    hex::encode(&result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test() {
        let hex = "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";
        let encoding_bytes = vec![73u8, 67u8, 69u8]; // "ICE"
        let encoded = run(hex, &encoding_bytes);

        assert_eq!("0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f", encoded);
    }
}
