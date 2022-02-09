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
//! # Usage
//! To use elabs_k256, you need to import the `elabs_k256` crate and use the `k256` or `k256_hash` function.
//! ```toml
//! [dependencies]
//! elabs_k256 = "0.1"
//! ```
//! # Example
//! ```rust
//! use elabs_k256::k256;
//!
//! fn main() {
//!    let input = "Hello World";
//!    let hash = k256(input);
//!    println!("{:?}", hash);
//! }
//! ```
//! ```rust
//! use elabs_k256::k256_hash;
//!
//! fn main() {
//!   let input = "Hello World";
//!   let mut hash = [0u8; 32];
//!   k256_hash(input, &mut hash);
//!   println!("{:?}", hash);
//! }
//! ```

use tiny_keccak::{Hasher, Keccak};

/// Compute the keccak256 hash of the input data.
/// The output is 32 bytes long.
/// # Arguments
/// * `data` - The data to hash.
/// # Return
/// * The hash of the data.
/// # Example
/// ```
/// use elabs_k256::k256;
///
/// fn main() {
///   let data = "Hello World";
///   let hash = k256(data);
///   println!("{:?}", hash);
/// }
/// ```
pub fn k256(data: &str) -> [u8; 32] {
    let mut hash = [0u8; 32];
    k256_hash(data, &mut hash);
    hash
}

/// Compute the Keccak-256 hash of the given slice.
/// The result is written directly to the output slice.
/// # Arguments
/// * `data` - The data to hash.
/// * `output` - The output buffer.
/// # Example
/// ```
/// use elabs_k256::k256_hash;
///
/// fn main() {
///  let data = "Hello World";
///  let mut hash = [0u8; 32];
///  k256_hash(data, &mut hash);
///  println!("{:?}", hash);
/// }
/// ```
pub fn k256_hash(data: &str, output: &mut [u8; 32]) {
    let mut hasher = Keccak::v256();
    hasher.update(data.as_bytes());
    hasher.finalize(output);
}

#[cfg(test)]
mod test {
    use super::*;

    // Test k256 and k256_hash.
    #[test]
    fn test_hash() {
        let data = "Hello World";
        let hash = k256(data);
        let mut hash2 = [0u8; 32];
        k256_hash(data, &mut hash2);
        assert_eq!(hash, hash2);
    }
}
