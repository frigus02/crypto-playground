pub fn run(hex: &str) -> &str {
    return hex;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let hex = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
        let base64 = run(hex);
        assert_eq!("SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t", base64);
    }
}
