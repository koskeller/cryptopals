use crate::{freq_analysis, vigenere, xor};

fn find_single_byte_xor_in_file(filename: &str) -> String {
    let file = std::fs::read_to_string(filename).unwrap();

    let mut max_score = 0.0;
    let mut result = String::new();
    for line in file.lines() {
        let line = hex::decode(line).unwrap();
        let key = xor::find_single_byte_xor_key(&line);
        let s = xor::single_key_xor(&line, key);
        let s = std::str::from_utf8(&s).unwrap_or("");
        let score = freq_analysis::score_by_trigraph(s);
        if score > max_score {
            max_score = score;
            result = s.to_string();
        }
    }
    result
}

fn break_repeatig_key_xor(cipher: &[u8]) -> String {
    let keysize = vigenere::find_key_size(cipher);

    let mut key = vec![];
    for n in 0..keysize {
        let mut i = n;
        let mut block = Vec::new();
        while cipher.len() > i + n {
            block.push(cipher[i]);
            i += keysize;
        }
        key.push(xor::find_single_byte_xor_key(&block));
    }

    let result = xor::repeating_key_xor(cipher, &key);
    String::from_utf8(result).unwrap()
}

fn detect_aes_in_ecb_mode(data: String) -> String {
    let mut max_score = 0;
    let mut result: &str = "";
    for line in data.lines() {
        let data = hex::decode(line).unwrap();
        let chunks: Vec<_> = data.chunks(16).collect();
        for i in 0..chunks.len() {
            let mut score = 0;
            for n in i..chunks.len() {
                if chunks[i] == chunks[n] {
                    score += 1;
                }
            }
            if score > max_score {
                max_score = score;
                result = line;
            }
        }
    }

    String::from(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::encoding;

    #[test]
    fn test_find_single_byte_xor_in_file() {
        let filename = "data/4.txt";
        let result = "Now that the party is jumping\n";
        assert_eq!(find_single_byte_xor_in_file(filename), result);
    }

    #[test]
    fn test_break_repeatig_key_xor() {
        let payload = std::fs::read_to_string("data/6.txt")
            .unwrap()
            .replace('\n', "");
        let cipher = encoding::base64_decode(payload).unwrap();

        assert!(break_repeatig_key_xor(&cipher).starts_with("I'm back and I'm ringin"));
    }

    #[test]
    fn test_detect_aes_in_ecb_mode() {
        let data = std::fs::read_to_string("data/8.txt").unwrap();
        assert!(detect_aes_in_ecb_mode(data).starts_with("d880619740a8a19b78"));
    }
}
