use super::super::encoding::base64;
use super::super::encoding::hex;

pub fn run(hex: &str) -> String {
    let bytes = hex::decode(hex);

    base64::encode(bytes.as_slice())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test() {
        let hex = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
        let base64 = run(hex);
        assert_eq!(
            "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t",
            base64
        );
    }
}
