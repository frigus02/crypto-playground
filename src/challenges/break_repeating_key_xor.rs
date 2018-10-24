use super::super::encoding::base64;

pub struct Info {
    pub key: Vec<u8>,
    pub plain_text: String,
}

pub fn run(base64: &str) -> Result<Info, String> {
    let bytes = base64::decode(base64);

    let key = vec![1, 2, 3];
    let plain_text = String::from("hello");
    return Ok(Info { key, plain_text });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test() {
        let base64 = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
        let result = run(base64);
        assert!(result.is_ok());

        let info = result.unwrap();
        assert_eq!(vec![1, 2, 3], info.key);
        assert_eq!("Cooking MC's like a pound of bacon", info.plain_text);
    }
}
