use super::freq_analysis;

pub fn single_key_xor(input: &[u8], key: u8) -> Vec<u8> {
    let mut result = Vec::new();
    for byte in input {
        result.push(byte ^ key);
    }
    result
}

pub fn repeating_key_xor(input: &[u8], key: &[u8]) -> Vec<u8> {
    let mut result = Vec::new();
    for (i, byte) in input.iter().enumerate() {
        result.push(byte ^ key[i % key.len()]);
    }
    result
}

pub fn find_single_byte_xor_key(input: &[u8]) -> u8 {
    let mut key = 0;
    let mut max_score = 0.0;
    for candidate in b' '..=b'z' {
        let result = single_key_xor(input, candidate);
        let score = freq_analysis::score_by_freq(std::str::from_utf8(&result).unwrap_or(""));
        if score > max_score {
            max_score = score;
            key = candidate;
        }
    }
    key
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_single_byte_xor_key() {
        let input =
            hex::decode("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736")
                .unwrap();
        assert_eq!(find_single_byte_xor_key(&input), 88);
    }

    #[test]
    fn test_single_byte_xor() {
        let input =
            hex::decode("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736")
                .unwrap();
        let result = "Cooking MC's like a pound of bacon";
        let god = single_key_xor(&input, 88);
        assert_eq!(std::str::from_utf8(&god).unwrap(), result);
    }

    #[test]
    fn test_repeating_key_xor() {
        let input = "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";
        let key = "ICE";
        let expected =
            "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272\
                       a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f";
        let got = repeating_key_xor(input.as_bytes(), key.as_bytes());
        let got = hex::encode(got);
        assert_eq!(got, expected);
    }
}
