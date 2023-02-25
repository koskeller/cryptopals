use super::{encoding, freq_analysis, vigenere, xor};

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

#[cfg(test)]
mod tests {
    use super::*;

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
        let cipher = encoding::decode(payload).unwrap();

        assert_eq!(break_repeatig_key_xor(&cipher), "I'm back and I'm ringin' the bell \nA rockin' on the mike while the fly girls yell \nIn ecstasy in the back of me \nWell that's my DJ Deshay cuttin' all them Z's \nHittin' hard and the girlies goin' crazy \nVanilla's on the mike, man I'm not lazy. \n\nI'm lettin' my drug kick in \nIt controls my mouth and I begin \nTo just let it flow, let my concepts go \nMy posse's to the side yellin', Go Vanilla Go! \n\nSmooth 'cause that's the way I will be \nAnd if you don't give a damn, then \nWhy you starin' at me \nSo get off 'cause I control the stage \nThere's no dissin' allowed \nI'm in my own phase \nThe girlies sa y they love me and that is ok \nAnd I can dance better than any kid n' play \n\nStage 2 -- Yea the one ya' wanna listen to \nIt's off my head so let the beat play through \nSo I can funk it up and make it sound good \n1-2-3 Yo -- Knock on some wood \nFor good luck, I like my rhymes atrocious \nSupercalafragilisticexpialidocious \nI'm an effect and that you can bet \nI can take a fly girl and make her wet. \n\nI'm like Samson -- Samson to Delilah \nThere's no denyin', You can try to hang \nBut you'll keep tryin' to get my style \nOver and over, practice makes perfect \nBut not if you're a loafer. \n\nYou'll get nowhere, no place, no time, no girls \nSoon -- Oh my God, homebody, you probably eat \nSpaghetti with a spoon! Come on and say it! \n\nVIP. Vanilla Ice yep, yep, I'm comin' hard like a rhino \nIntoxicating so you stagger like a wino \nSo punks stop trying and girl stop cryin' \nVanilla Ice is sellin' and you people are buyin' \n'Cause why the freaks are jockin' like Crazy Glue \nMovin' and groovin' trying to sing along \nAll through the ghetto groovin' this here song \nNow you're amazed by the VIP posse. \n\nSteppin' so hard like a German Nazi \nStartled by the bases hittin' ground \nThere's no trippin' on mine, I'm just gettin' down \nSparkamatic, I'm hangin' tight like a fanatic \nYou trapped me once and I thought that \nYou might have it \nSo step down and lend me your ear \n'89 in my time! You, '90 is my year. \n\nYou're weakenin' fast, YO! and I can tell it \nYour body's gettin' hot, so, so I can smell it \nSo don't be mad and don't be sad \n'Cause the lyrics belong to ICE, You can call me Dad \nYou're pitchin' a fit, so step back and endure \nLet the witch doctor, Ice, do the dance to cure \nSo come up close and don't be square \nYou wanna battle me -- Anytime, anywhere \n\nYou thought that I was weak, Boy, you're dead wrong \nSo come on, everybody and sing this song \n\nSay -- Play that funky music Say, go white boy, go white boy go \nplay that funky music Go white boy, go white boy, go \nLay down and boogie and play that funky music till you die. \n\nPlay that funky music Come on, Come on, let me hear \nPlay that funky music white boy you say it, say it \nPlay that funky music A little louder now \nPlay that funky music, white boy Come on, Come on, Come on \nPlay that funky music \n");
    }
}
