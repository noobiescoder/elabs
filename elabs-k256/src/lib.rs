// Copyright (C) 2022 The Elabs Project Authors.
// This file is part of the Elabs library.
//
// The Elabs library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, version 3 of the License.
//
// The Elabs library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License along with The Elabs library.
// If not, see <https://www.gnu.org/licenses/>.

//! Elabs-k256 is a wrapper around the tiny_keccak::Keccak::v256() Hasher.
//! It give a simple interface to use the Hasher.
//! # Example
//! ```
//! use elabs_k256::k256_hash;
//!
//! fn main() {
//!    let data = "Hello World";
//!    let mut hash = [0u8; 32];
//!    k256_hash(data, &mut hash);
//!    println!("{:?}", hash);
//! }
//! ```

use tiny_keccak::{Hasher, Keccak};

/// Compute in place the Keccak-256 hash of the given slice.
pub fn k256(data: &mut [u8]) {
    let mut hasher = Keccak::v256();
    hasher.update(data);
    hasher.finalize(data);
}

/// Compute the Keccak-256 hash of the given slice.
/// The result is returned to the given vector.
pub fn k256_hash(data: &[u8], out: &mut [u8]) {
    let mut hasher = Keccak::v256();
    hasher.update(data);
    hasher.finalize(out);
}

#[cfg(test)]
mod test {
    use super::*;

    // Test k256 and k256_hash.
    #[test]
    fn test_k256() {
        let mut data = [0u8; 32];
        let mut out = [0u8; 32];

        // Test with empty data.
        k256(&mut data);
        k256_hash(&[0u8; 32], &mut out);
        assert_eq!(data, out);
    }
}
