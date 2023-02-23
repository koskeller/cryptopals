#![allow(dead_code)]
fn fixed_xor(a: &str, b: &str) -> String {
    let a = hex::decode(a).unwrap();
    let b = hex::decode(b).unwrap();
    let mut result = Vec::new();
    for (x, y) in a.iter().zip(b.iter()) {
        result.push(x ^ y);
    }
    hex::encode(result)
}

fn decode_xor_cipher(input: &str, key: u8) -> String {
    println!("key: {}", key);
    let input = hex::decode(input).unwrap();
    let mut result = Vec::new();
    for byte in input {
        result.push(byte ^ key);
    }
    // TODO: figure out why panics
    String::from_utf8(result).unwrap_or("".to_string())
}

/// Score a string based on the frequency of letters in the English language.
/// The score is the sum of the number of times each letter appears in the string.
fn score_string(input: &[u8]) -> u32 {
    let mut score = 0;
    for byte in input {
        match byte {
            b'A'..=b'Z' => score += 1,
            b'a'..=b'z' => score += 1,
            _ => (),
        }
        match byte {
            b'e' | b't' | b'a' | b'o' | b'i' | b'n' | b's' | b'h' | b'r' => score += 5,
            _ => (),
        }
    }
    score
}

fn find_single_byte_xor(input: &str) -> u8 {
    let input = hex::decode(input).unwrap();

    let mut char = 0;
    let mut max_score = 0;
    let mut result = Vec::new();
    for key in b' '..=b'z' {
        for byte in &input {
            result.push(byte ^ key);
        }
        let score = score_string(&result);
        if score > max_score {
            max_score = score;
            char = key;
        }
        result.clear();
    }
    char
}

fn find_single_byte_xor_in_file(filename: &str) -> String {
    let file = std::fs::read_to_string(filename).unwrap();

    let mut max_score = 0;
    let mut result = String::new();
    for line in file.lines() {
        let key = find_single_byte_xor(line);
        let s = decode_xor_cipher(line, key);
        let score = score_string(s.as_bytes());
        if score > max_score {
            max_score = score;
            result = s.clone();
        }
    }
    result
}

fn repeating_key_xor(input: &str, key: &[u8]) -> String {
    let mut result = Vec::new();
    for (i, byte) in input.chars().map(|c| c as u8).enumerate() {
        result.push(byte ^ key[i % key.len()]);
    }
    hex::encode(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fixed_xor() {
        let a = "1c0111001f010100061a024b53535009181c";
        let b = "686974207468652062756c6c277320657965";
        let result = "746865206b696420646f6e277420706c6179";
        assert_eq!(fixed_xor(a, b), result);
    }

    #[test]
    fn test_find_single_byte_xor() {
        let input = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
        assert_eq!(find_single_byte_xor(input), 88);
    }

    #[test]
    fn test_single_byte_xor() {
        let input = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
        let result = "Cooking MC's like a pound of bacon";
        assert_eq!(decode_xor_cipher(input, 88), result);
    }

    #[test]
    fn test_find_single_byte_xor_in_file() {
        let filename = "data/4.txt";
        let result = "Now that the party is jumping\n";
        assert_eq!(find_single_byte_xor_in_file(filename), result);
    }

    #[test]
    fn test_repeating_key_xor() {
        let input = "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";
        let key = "ICE";
        let expected =
            "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272\
                       a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f";
        assert_eq!(repeating_key_xor(input, key.as_bytes()), expected);
    }
}
