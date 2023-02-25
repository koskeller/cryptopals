use anyhow::Result;
use openssl::symm::{self, Cipher};

pub fn aes_ecb_encrypt(data: &[u8], key: &[u8]) -> Result<Vec<u8>> {
    let cipher = Cipher::aes_128_ecb();
    let iv: Vec<u8> = vec![];
    Ok(symm::encrypt(cipher, key, Some(&iv), data)?)
}

pub fn aes_ecb_decrypt(data: &[u8], key: &[u8]) -> Result<Vec<u8>> {
    let cipher = Cipher::aes_128_ecb();
    let iv: Vec<u8> = vec![];
    Ok(symm::decrypt(cipher, key, Some(&iv), data)?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::encoding;

    #[test]
    fn test_decrypt_aes_ecb() {
        let key = b"YELLOW SUBMARINE";
        let data = std::fs::read_to_string("data/7.txt")
            .unwrap()
            .replace('\n', "");
        let data = encoding::base64_decode(data).unwrap();

        assert!(aes_ecb_decrypt(&data, key)
            .unwrap()
            .starts_with(b"I'm back and I'm ringin"));
    }
}
