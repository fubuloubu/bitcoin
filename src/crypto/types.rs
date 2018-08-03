use std::{fmt, cmp, ops, str};
use super::hex::{ToHex, FromHex, FromHexError};

// TODO: Ensure this code is minimal

pub struct H256([u8; 32]);

impl Default for H256 {
    fn default() -> Self {
        H256([0u8; 32])
    }
}

impl AsRef<H256> for H256 {
	fn as_ref(&self) -> &H256 {
		self
	}
}

impl Clone for H256 {
	fn clone(&self) -> Self {
		let mut result = Self::default();
		result.copy_from_slice(&self.0);
		result
	}
}

impl From<[u8; 32]> for H256 {
	fn from(h: [u8; 32]) -> Self {
		H256(h)
	}
}

impl From<H256> for [u8; 32] {
	fn from(h: H256) -> Self {
		h.0
	}
}

impl<'a> From<&'a [u8]> for H256 {
	fn from(slc: &[u8]) -> Self {
		let mut inner = [0u8; 32];
		inner[..].clone_from_slice(&slc[0..32]);
		H256(inner)
	}
}

impl From<&'static str> for H256 {
	fn from(s: &'static str) -> Self {
		s.parse().unwrap()
	}
}

impl From<String> for H256 {
	fn from(s: String) -> Self {
		s.parse().unwrap()
	}
}

impl str::FromStr for H256 {
	type Err = FromHexError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let vec = try!(s.from_hex());
		match vec.len() {
			32 => {
				let mut result = [0u8; 32];
				result.copy_from_slice(&vec);
				Ok(H256(result))
			},
			_ => Err(FromHexError::InvalidHexLength)
		}
	}
}

impl fmt::Debug for H256 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.0.to_hex())
    }
}

impl fmt::Display for H256 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.0.to_hex())
    }
}

impl ops::Deref for H256 {
	type Target = [u8; 32];

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl ops::DerefMut for H256 {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.0
	}
}

impl cmp::PartialEq for H256 {
	fn eq(&self, other: &Self) -> bool {
		let self_ref: &[u8] = &self.0;
		let other_ref: &[u8] = &other.0;
		self_ref == other_ref
	}
}

impl cmp::PartialOrd for H256 {
	fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
		let self_ref: &[u8] = &self.0;
		let other_ref: &[u8] = &other.0;
		self_ref.partial_cmp(other_ref)
	}
}

impl H256 {
	pub fn take(self) -> [u8; 32] {
		self.0
	}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_h256_default() {
        let expected = H256([0u8; 32]);
        let result = H256::default();
        assert_eq!(expected, result);
    }

    #[test]
    fn test_h256_string() {
        let expected = H256([0u8; 32]);
        let result = "0000000000000000000000000000000000000000000000000000000000000000".into();
        assert_eq!(expected, result);
    }

    #[test]
    fn test_h256_equal() {
        let a: H256 = "1234567812345678123456781234567812345678123456781234567812345678".into();
        let b: H256 = "1234567812345678123456781234567812345678123456781234567812345678".into();
        assert_eq!(a, b);
    }

    #[test]
    fn test_h256_order() {
        let a: H256 = "0000000000000000000000000000000000000000000000000000000000000000".into();
        let b: H256 = "1234567812345678123456781234567812345678123456781234567812345678".into();
        assert!(a < b);
    }
}
