use crate::byte_utils;

use parity_codec_derive::{Encode, Decode};

/// Typeless generic key into contract storage.
///
/// # Note
///
/// This is the most low-level method to access contract storage.
///
/// # Unsafe
///
/// - Does not restrict ownership.
/// - Can read and write to any storage location.
/// - Does not synchronize between main memory and contract storage.
/// - Violates Rust's mutability and immutability guarantees.
///
/// Prefer using types found in `collections` or `Synced` type.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Encode, Decode)]
pub struct Key(pub [u8; 32]);

impl core::ops::Add<u32> for Key {
	type Output = Key;

	fn add(self, rhs: u32) -> Self::Output {
		let mut result = self;
		let ovfl = byte_utils::bytes_add_bytes(
			result.as_bytes_mut(),
			&byte_utils::u32_to_bytes4(rhs)
		);
		assert!(!ovfl, "overflows should not occure for 256-bit keys");
		result
	}
}

impl core::ops::AddAssign<u32> for Key {
	fn add_assign(&mut self, rhs: u32) {
		let ovfl = byte_utils::bytes_add_bytes(
			self.as_bytes_mut(),
			&byte_utils::u32_to_bytes4(rhs)
		);
		assert!(!ovfl, "overflows should not occure for 256-bit keys");
	}
}

impl core::ops::Add<u64> for Key {
	type Output = Key;

	fn add(self, rhs: u64) -> Self::Output {
		let mut result = self;
		let ovfl = byte_utils::bytes_add_bytes(
			result.as_bytes_mut(),
			&byte_utils::u64_to_bytes8(rhs)
		);
		debug_assert!(!ovfl, "overflows should not occure for 256-bit keys");
		result
	}
}

impl core::ops::AddAssign<u64> for Key {
	fn add_assign(&mut self, rhs: u64) {
		let ovfl = byte_utils::bytes_add_bytes(
			self.as_bytes_mut(),
			&byte_utils::u64_to_bytes8(rhs)
		);
		debug_assert!(!ovfl, "overflows should not occure for 256-bit keys");
	}
}

impl Key {
	/// Returns the byte slice of this key.
	pub fn as_bytes(&self) -> &[u8] {
		&self.0
	}

	/// Returns the mutable byte slice of this key.
	pub fn as_bytes_mut(&mut self) -> &mut [u8] {
		&mut self.0
	}
}

#[cfg(all(test, feature = "test-env"))]
mod tests {
	use super::*;

	use crate::env::{Env, ContractEnv};

	#[test]
	fn store_load_clear() {
		let key = Key([0x42; 32]);
		assert_eq!(unsafe { ContractEnv::load(key) }, None);
		unsafe { ContractEnv::store(key, &[0x5]); }
		assert_eq!(unsafe { ContractEnv::load(key) }, Some(vec![0x5]));
		unsafe { ContractEnv::clear(key); }
		assert_eq!(unsafe { ContractEnv::load(key) }, None);
	}

	#[test]
	fn key_with_offset() {
		let key00 = Key([0x0; 32]);
		let key05 = key00 + 5_u32;  // -> 5
		let key10 = key00 + 10_u32; // -> 10         | same as key55
		let key55 = key05 + 5_u32;  // -> 5 + 5 = 10 | same as key10
		unsafe { ContractEnv::store(key55, &[42]); }
		assert_eq!(unsafe { ContractEnv::load(key10) }, Some(vec![42]));
		unsafe { ContractEnv::store(key10, &[13, 37]); }
		assert_eq!(unsafe { ContractEnv::load(key55) }, Some(vec![13, 37]));
	}

	#[test]
	fn as_bytes() {
		let mut key = Key([0x42; 32]);
		assert_eq!(key.as_bytes(), &[0x42; 32]);
		assert_eq!(key.as_bytes_mut(), &mut [0x42; 32]);
	}
}