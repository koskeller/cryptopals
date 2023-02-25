// source: https://norvig.com/mayzner.html
static LETTER_FREQ: [(&str, f32); 26] = [
    ("e", 0.1249),
    ("t", 0.0928),
    ("a", 0.0804),
    ("o", 0.0764),
    ("i", 0.0757),
    ("n", 0.0723),
    ("s", 0.0651),
    ("r", 0.0628),
    ("h", 0.0505),
    ("l", 0.0407),
    ("d", 0.0382),
    ("c", 0.0334),
    ("u", 0.0273),
    ("m", 0.0251),
    ("f", 0.0240),
    ("p", 0.0214),
    ("g", 0.0187),
    ("w", 0.0168),
    ("y", 0.0166),
    ("b", 0.0148),
    ("v", 0.0105),
    ("k", 0.0054),
    ("x", 0.0023),
    ("j", 0.0016),
    ("q", 0.0012),
    ("z", 0.0009),
];

// source: http://practicalcryptography.com/cryptanalysis/letter-frequencies-various-languages/english-letter-frequencies/
static TRIGRAPH_FREQ: [(&str, f32); 30] = [
    ("the", 1.81),
    ("and", 0.73),
    ("ing", 0.72),
    ("ent", 0.42),
    ("ion", 0.42),
    ("her", 0.36),
    ("for", 0.34),
    ("tha", 0.33),
    ("nth", 0.33),
    ("int", 0.32),
    ("ere", 0.31),
    ("tio", 0.31),
    ("ter", 0.30),
    ("est", 0.28),
    ("ers", 0.28),
    ("ati", 0.26),
    ("hat", 0.26),
    ("ate", 0.25),
    ("all", 0.25),
    ("eth", 0.24),
    ("hes", 0.24),
    ("ver", 0.24),
    ("his", 0.24),
    ("oft", 0.22),
    ("ith", 0.21),
    ("fth", 0.21),
    ("sth", 0.21),
    ("oth", 0.21),
    ("res", 0.21),
    ("ont", 0.20),
];

pub fn score_by_trigraph(input: &str) -> f32 {
    let mut score = 0.0;
    for (pattern, freq) in TRIGRAPH_FREQ.iter() {
        score += input.to_lowercase().matches(pattern).count() as f32 * freq;
    }
    score
}

pub fn score_by_freq(input: &str) -> f32 {
    let mut score = 0.;
    for byte in input.bytes() {
        match byte {
            b'A'..=b'Z' => score += 1.,
            b'a'..=b'z' => score += 1.,
            _ => (),
        }
        match byte {
            b'e' | b't' | b'a' | b'o' | b'i' | b'n' | b's' | b'h' | b'r' => score += 5.,
            _ => (),
        }
    }
    score
}
