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

//! Elabs-Crypto library
//! This library contains all the necessary cryptographic functions
//! to help the Elabs project.
//! It contains the following modules:
//! * [`key`] - Contains the key generation functions.
//! * [`address`] - Contains the address generation functions.
//! To use the library, you need to import the `elabs_crypto` crate.
//! ```
//! use elabs_crypto::*;
//! ```
//! # Examples
//! ## Key Generation
//! ```
//! use elabs_crypto::*;
//! let sk = PrivateKey::new();
//! let pk = sk.public_key();
//! ```
//! ## Address Generation
//! ```
//! use elabs_crypto::*;
//! let sk = PrivateKey::new();
//! let pk = sk.public_key();
//! let addr = Address::from_public_key(&pk);
//! ```

pub mod address;
pub use address::*;

pub mod key;
pub use key::*;
