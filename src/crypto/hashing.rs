extern crate crypto_hash;

use super::types::H256;

use self::crypto_hash::{Algorithm, hex_digest};

pub fn sha256(_input: &[u8]) -> H256 {
    H256::from(hex_digest(Algorithm::SHA256, _input))
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sha256() {
        let expected: H256 = "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824".into();
        let result = sha256(b"hello");
        assert_eq!(result, expected);
    }
}
