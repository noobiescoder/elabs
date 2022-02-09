// Copyright (C) 2022 The Elabs Authors.
// This file is part of the Elabs.
//
// Elabs is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Elabs is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Elabs.  If not, see <https://www.gnu.org/licenses/>.

//! # Elabs Crypto.
//! This crate provides a set of cryptographic helper functions.
//! It provide `secp256k1` PublicKey, PrivateKey, signer, and `hash` functions.
//! This crate is based on [`secp256k1`] and [`tiny-keccak`].
//!
//! ## Usage
//! ```toml
//! [dependencies]
//! elabs-crypto = "0.1"
//! ```
//! ## Example
//! ```rust
//! use elabs_crypto::*;
//!
//! fn main() {
//!        let msg = b"hello world";
//!        let hash = keccak256(msg);
//!        let sk = PrivateKey::random();
//!        let pk = sk.to_public().unwrap();
//!        let sig = sign(msg, sk).unwrap();
//!        let (recid, bsig) = sig.serialize_compact();
//!        let pk2 = ecrecover(&hash, &bsig, recid.to_i32() as u8).unwrap();
//!        assert_eq!(pk, pk2);
//! }
//! ```

pub mod keys;
pub use keys::*;

pub mod signer;
pub use signer::*;

use tiny_keccak::{Hasher, Keccak};

/// calculate and return keccak256 hash of the input data.
/// # Arguments
/// * `data` - input data
/// # Returns
/// keccak256 hash of the input data.
pub fn keccak256(data: &[u8]) -> [u8; 32] {
    let mut buf = [0u8; 32];
    let mut hasher = Keccak::v256();
    hasher.update(data);
    hasher.finalize(&mut buf);
    buf
}

/// calculate and return keccak512 hash of the input data.
/// # Arguments
/// * `data` - input data
/// # Returns
/// keccak512 hash of the input data.
pub fn keccak512(data: &[u8]) -> [u8; 64] {
    let mut buf = [0u8; 64];
    let mut hasher = Keccak::v512();
    hasher.update(data);
    hasher.finalize(&mut buf);
    buf
}
