#![allow(dead_code, unused_variables)]

use base64::Engine;

fn fixed_xor(a: &str, b: &str) -> String {
    let a = hex::decode(a).unwrap();
    let b = hex::decode(b).unwrap();
    let mut result = Vec::new();
    for (x, y) in a.iter().zip(b.iter()) {
        result.push(x ^ y);
    }
    hex::encode(result)
}

fn decode_xor_cipher(input: &[u8], key: u8) -> String {
    let mut result = Vec::new();
    for byte in input {
        result.push(byte ^ key);
    }
    // TODO: figure out why panics
    String::from_utf8(result).unwrap_or("".to_string())
}

fn decore_repeating_key_cipher(input: &[u8], key: &[u8]) -> String {
    let mut result = Vec::new();
    for (i, byte) in input.iter().enumerate() {
        result.push(byte ^ key[i % key.len()]);
    }
    String::from_utf8(result).unwrap()
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

fn find_single_byte_xor(input: &[u8]) -> u8 {
    let mut char = 0;
    let mut max_score = 0;
    let mut result = Vec::new();
    for key in b' '..=b'z' {
        for byte in input {
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
        let line = hex::decode(line).unwrap();
        let key = find_single_byte_xor(&line);
        let s = decode_xor_cipher(&line, key);
        let score = score_string(s.as_bytes());
        if score > max_score {
            max_score = score;
            result = s.clone();
        }
    }
    result
}

fn repeating_key_xor(input: &[u8], key: &[u8]) -> String {
    let mut result = Vec::new();
    for (i, byte) in input.iter().enumerate() {
        result.push(byte ^ key[i % key.len()]);
    }
    hex::encode(result)
}

fn humming_distance(a: &[u8], b: &[u8]) -> u8 {
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

fn find_key_size(cipher: &[u8]) -> usize {
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

fn break_repeatig_key_xor(cipher: &[u8]) -> String {
    let keysize = find_key_size(cipher);

    let mut key = vec![];
    for n in 0..keysize {
        let mut i = n;
        let mut block = Vec::new();
        while cipher.len() > i + n {
            block.push(cipher[i]);
            i += keysize;
        }
        key.push(find_single_byte_xor(&block));
    }

    decore_repeating_key_cipher(cipher, &key)
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
        let input =
            hex::decode("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736")
                .unwrap();
        assert_eq!(find_single_byte_xor(&input), 88);
    }

    #[test]
    fn test_single_byte_xor() {
        let input =
            hex::decode("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736")
                .unwrap();
        let result = "Cooking MC's like a pound of bacon";
        assert_eq!(decode_xor_cipher(&input, 88), result);
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
        assert_eq!(
            repeating_key_xor(input.as_bytes(), key.as_bytes()),
            expected
        );
    }

    #[test]
    fn test_humming_distance() {
        let a = b"this is a test";
        let b = b"wokka wokka!!!";
        assert_eq!(humming_distance(a, b), 37);
    }

    // #[test]
    // fn test_decode_repeating_key_cipher() {
    //     let input = "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272\
    //                    a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f";
    //     let input = base64::engine::general_purpose::STANDARD
    //         .decode(input)
    //         .unwrap();
    //     let key = "ICE";
    //     let expected =
    //         "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";
    //     assert_eq!(
    //         decore_repeating_key_cipher(&input, key.as_bytes()),
    //         input
    //     );
    // }
    //
    #[test]
    fn test_find_key_size() {
        let file = std::fs::read_to_string("data/6.txt")
            .unwrap()
            .replace('\n', "");
        let cipher = base64::engine::general_purpose::STANDARD
            .decode(file)
            .unwrap();

        assert_eq!(find_key_size(&cipher), 29);
    }

    #[test]
    fn test_break_repeatig_key_xor() {
        let file = std::fs::read_to_string("data/6.txt")
            .unwrap()
            .replace('\n', "");
        let cipher = base64::engine::general_purpose::STANDARD
            .decode(file)
            .unwrap();

        assert_eq!(break_repeatig_key_xor(&cipher), "I'm back and I'm ringin' the bell \nA rockin' on the mike while the fly girls yell \nIn ecstasy in the back of me \nWell that's my DJ Deshay cuttin' all them Z's \nHittin' hard and the girlies goin' crazy \nVanilla's on the mike, man I'm not lazy. \n\nI'm lettin' my drug kick in \nIt controls my mouth and I begin \nTo just let it flow, let my concepts go \nMy posse's to the side yellin', Go Vanilla Go! \n\nSmooth 'cause that's the way I will be \nAnd if you don't give a damn, then \nWhy you starin' at me \nSo get off 'cause I control the stage \nThere's no dissin' allowed \nI'm in my own phase \nThe girlies sa y they love me and that is ok \nAnd I can dance better than any kid n' play \n\nStage 2 -- Yea the one ya' wanna listen to \nIt's off my head so let the beat play through \nSo I can funk it up and make it sound good \n1-2-3 Yo -- Knock on some wood \nFor good luck, I like my rhymes atrocious \nSupercalafragilisticexpialidocious \nI'm an effect and that you can bet \nI can take a fly girl and make her wet. \n\nI'm like Samson -- Samson to Delilah \nThere's no denyin', You can try to hang \nBut you'll keep tryin' to get my style \nOver and over, practice makes perfect \nBut not if you're a loafer. \n\nYou'll get nowhere, no place, no time, no girls \nSoon -- Oh my God, homebody, you probably eat \nSpaghetti with a spoon! Come on and say it! \n\nVIP. Vanilla Ice yep, yep, I'm comin' hard like a rhino \nIntoxicating so you stagger like a wino \nSo punks stop trying and girl stop cryin' \nVanilla Ice is sellin' and you people are buyin' \n'Cause why the freaks are jockin' like Crazy Glue \nMovin' and groovin' trying to sing along \nAll through the ghetto groovin' this here song \nNow you're amazed by the VIP posse. \n\nSteppin' so hard like a German Nazi \nStartled by the bases hittin' ground \nThere's no trippin' on mine, I'm just gettin' down \nSparkamatic, I'm hangin' tight like a fanatic \nYou trapped me once and I thought that \nYou might have it \nSo step down and lend me your ear \n'89 in my time! You, '90 is my year. \n\nYou're weakenin' fast, YO! and I can tell it \nYour body's gettin' hot, so, so I can smell it \nSo don't be mad and don't be sad \n'Cause the lyrics belong to ICE, You can call me Dad \nYou're pitchin' a fit, so step back and endure \nLet the witch doctor, Ice, do the dance to cure \nSo come up close and don't be square \nYou wanna battle me -- Anytime, anywhere \n\nYou thought that I was weak, Boy, you're dead wrong \nSo come on, everybody and sing this song \n\nSay -- Play that funky music Say, go white boy, go white boy go \nplay that funky music Go white boy, go white boy, go \nLay down and boogie and play that funky music till you die. \n\nPlay that funky music Come on, Come on, let me hear \nPlay that funky music white boy you say it, say it \nPlay that funky music A little louder now \nPlay that funky music, white boy Come on, Come on, Come on \nPlay that funky music \n");
    }
}
