use base64::{engine::general_purpose, DecodeError, Engine as _};

pub fn decode(s: String) -> Result<Vec<u8>, DecodeError> {
    general_purpose::STANDARD.decode(s)
}

pub fn encode(b: &[u8]) -> String {
    general_purpose::STANDARD.encode(b)
}
