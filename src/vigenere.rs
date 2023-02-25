use super::encoding;

pub fn humming_distance(a: &[u8], b: &[u8]) -> u8 {
    let mut distance = 0;
    for (x, y) in a.iter().zip(b.iter()) {
        let mut xor = x ^ y;
        while xor > 0 {
            distance += xor & 1;
            xor >>= 1;
        }
    }
    distance
}

pub fn find_key_size(cipher: &[u8]) -> usize {
    const KEYSIZE_MIN: usize = 2;
    const KEYSIZE_MAX: usize = 40;

    let mut scores = Vec::new();
    for keysize in KEYSIZE_MIN..=KEYSIZE_MAX {
        let mut score = 0.0;
        let mut a = 0;
        let mut b = keysize;
        let mut i = 0;
        while b + keysize < cipher.len() {
            let chunk1 = &cipher[a..b];
            let chunk2 = &cipher[b..b + keysize];

            let distance = humming_distance(chunk1, chunk2) / keysize as u8;
            score += distance as f64;

            a += keysize;
            b += keysize;
            i += 1;
        }
        score /= i as f64;
        scores.push((keysize, score));
    }

    scores.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    scores[0].0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_humming_distance() {
        let a = b"this is a test";
        let b = b"wokka wokka!!!";
        assert_eq!(humming_distance(a, b), 37);
    }

    #[test]
    fn test_find_key_size() {
        let file = std::fs::read_to_string("data/6.txt")
            .unwrap()
            .replace('\n', "");
        let cipher = encoding::decode(file).unwrap();

        assert_eq!(find_key_size(&cipher), 29);
    }
}
