use anyhow::{anyhow, Result};

pub fn add(blocks: &mut [Vec<u8>], size: u8) -> Result<()> {
    todo!()
}

// Validates that all blocks except the last have valid size.
fn is_valid_nonpad(blocks: &[Vec<u8>], size: u8) -> bool {
    for block in blocks.iter().take(blocks.len() - 1) {
        if block.len() != size as usize {
            return false;
        }
    }
    true
}

pub fn add_pkcs8_padding(block: &[u8], len: usize) -> Result<Vec<u8>> {
    if block.len() >= len {
        return Err(anyhow!("Block length is greater or equal to {}", len));
    }
    let padbyte = len - block.len();
    Ok(block
        .iter()
        .copied()
        .chain(std::iter::repeat(padbyte as u8))
        .take(len)
        .collect())
}

#[cfg(test)]
mod padding_tests {
    use super::*;

    #[test]
    fn test_add_pkcs8_padding() {
        let block = b"YELLOW SUBMARINE";
        let want = b"YELLOW SUBMARINE\x04\x04\x04\x04";
        assert_eq!(add_pkcs8_padding(block, 20).unwrap(), want);
        assert!(add_pkcs8_padding(block, 1).is_err());
    }
}
