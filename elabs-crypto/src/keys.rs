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

/// ecdsa(secp256k1) SecretKey wrapper.
/// This is a wrapper for secp256k1 SecretKey.
/// The default byte size is 32 bytes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PrivateKey(pub [u8; 32]);

/// ecdsa(secp256k1) PublicKey wrapper.
/// This is a wrapper for secp256k1 PublicKey.
/// The default byte size is 65 bytes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PublicKey(pub [u8; 65]);

// TODO: implement Eq for PrivateKey and PublicKey.
// TODO: implement PartialEq for PrivateKey and PublicKey.

use std::fmt::Display;

use rand::RngCore;

impl PrivateKey {
    /// Generate a random PrivateKey.
    /// # Returns
    /// Random PrivateKey.
    pub fn random() -> Self {
        let mut buf = [0u8; 32];
        rand::thread_rng().fill_bytes(&mut buf);
        let sk = secp256k1::SecretKey::from_slice(&buf).unwrap();
        PrivateKey(sk.serialize_secret())
    }

    /// Create a PrivateKey from a secp256k1 SecretKey.
    /// # Arguments
    /// * `sk` - secp256k1 SecretKey.
    /// # Returns
    /// PrivateKey.
    pub fn from_secp256k1(sk: secp256k1::SecretKey) -> Self {
        PrivateKey(sk.serialize_secret())
    }

    /// Create a PrivateKey from a slice.
    /// # Arguments
    /// * `buf` - slice.
    /// # Returns
    /// * `Ok(PrivateKey)` - if the slice is 32 bytes and is a valid secp256k1 SecretKey.
    /// * `Err(KeyError)` - if the slice is not 32 bytes or is not a valid secp256k1 SecretKey.
    pub fn from_slice(buf: &[u8]) -> Result<Self, KeyError> {
        if buf.len() != 32 {
            return Err(KeyError::InvalidLength);
        }
        let sk = secp256k1::SecretKey::from_slice(buf).map_err(|_| KeyError::InvalidSecp256k1)?;
        Ok(PrivateKey(sk.serialize_secret()))
    }

    /// Create a PrivateKey from a hex string.
    /// If the string contains prefix `0x`, it will be removed.
    /// # Arguments
    /// * `hex` - hex string.
    /// # Returns
    /// * `Ok(PrivateKey)` - if the string is 32 bytes and is a valid secp256k1 SecretKey.
    /// * `Err(KeyError)` - if the string is not 32 bytes or is not a valid secp256k1 SecretKey.
    pub fn from_hex(hex: &str) -> Result<Self, KeyError> {
        let hex_str = if hex.starts_with("0x") {
            &hex[2..]
        } else {
            hex
        };

        if hex_str.len() != 64 {
            return Err(KeyError::InvalidLength);
        }

        let mut buf = [0u8; 32];
        hex::decode_to_slice(hex_str, &mut buf).map_err(|_| KeyError::InvalidHex)?;

        let sk = secp256k1::SecretKey::from_slice(&buf).map_err(|_| KeyError::InvalidSecp256k1)?;
        Ok(PrivateKey(sk.serialize_secret()))
    }

    /// Return the secp256k1 SecretKey.
    /// # Returns
    /// * `Ok(secp256k1::SecretKey)` - if the PrivateKey is a valid secp256k1 SecretKey.
    /// * `Err(KeyError)` - if the PrivateKey is not a valid secp256k1 SecretKey.
    pub fn to_secp256k1(&self) -> Result<secp256k1::SecretKey, KeyError> {
        secp256k1::SecretKey::from_slice(&self.0).map_err(|_| KeyError::InvalidSecp256k1)
    }

    /// Return bytes of the PrivateKey.
    /// # Returns
    /// * `[u8; 32]` - if the PrivateKey is a valid secp256k1 SecretKey.
    /// * `Err(KeyError)` - if the PrivateKey is not a valid secp256k1 SecretKey.
    pub fn to_bytes(&self) -> Result<[u8; 32], KeyError> {
        let sk = self.to_secp256k1()?;
        Ok(sk.serialize_secret())
    }

    /// Return hex string of the PrivateKey.
    /// # Returns
    /// * `String` - if the PrivateKey is a valid secp256k1 SecretKey.
    /// * `Err(KeyError)` - if the PrivateKey is not a valid secp256k1 SecretKey.
    pub fn to_hex(&self) -> Result<String, KeyError> {
        let sk = self.to_secp256k1()?;
        Ok(hex::encode(sk.serialize_secret()))
    }

    /// Return the secp256k1 PublicKey.
    /// # Returns
    /// * `Ok(secp256k1::PublicKey)` - the PublicKey.
    /// * `Err(KeyError)` - if the PrivateKey is not a valid secp256k1 SecretKey.
    pub fn to_public(&self) -> Result<PublicKey, KeyError> {
        let sk = self.to_secp256k1()?;
        Ok(PublicKey(
            secp256k1::PublicKey::from_secret_key(&secp256k1::Secp256k1::new(), &sk)
                .serialize_uncompressed(),
        ))
    }
}

impl PublicKey {
    /// Create a PublicKey from a PrivateKey.
    /// # Arguments
    /// * `sk` - SecretKey.
    /// # Returns
    /// * `Ok(PublicKey)` - if the SecretKey is a valid secp256k1 SecretKey.
    /// * `Err(KeyError)` - if the SecretKey is not a valid secp256k1 SecretKey.
    pub fn from_private(sk: &PrivateKey) -> Result<Self, KeyError> {
        let sk = sk.to_secp256k1()?;
        Ok(PublicKey(
            secp256k1::PublicKey::from_secret_key(&secp256k1::Secp256k1::new(), &sk)
                .serialize_uncompressed(),
        ))
    }

    /// Create a PublicKey from a secp256k1 PublicKey.
    /// # Arguments
    /// * `pk` - secp256k1 PublicKey.
    /// # Returns
    /// PublicKey.
    pub fn from_secp256k1(pk: secp256k1::PublicKey) -> Self {
        PublicKey(pk.serialize_uncompressed())
    }

    /// Create a PublicKey from a slice.
    /// # Arguments
    /// * `buf` - slice.
    /// # Returns
    /// * `Ok(PublicKey)` - if the slice is 65 bytes and is a valid secp256k1 PublicKey.
    /// * `Err(KeyError)` - if the slice is not 65 bytes or is not a valid secp256k1 PublicKey.
    pub fn from_slice(buf: &[u8]) -> Result<Self, KeyError> {
        if buf.len() != 65 {
            return Err(KeyError::InvalidLength);
        }
        let pk = secp256k1::PublicKey::from_slice(buf).map_err(|_| KeyError::InvalidSecp256k1)?;
        Ok(PublicKey(pk.serialize_uncompressed()))
    }

    /// Create a PublicKey from a hex string.
    /// If the string contains prefix `0x`, it will be removed.
    /// # Arguments
    /// * `hex` - hex string.
    /// # Returns
    /// * `Ok(PublicKey)` - if the string is 65 bytes and is a valid secp256k1 PublicKey.
    /// * `Err(KeyError)` - if the string is not 65 bytes or is not a valid secp256k1 PublicKey.
    pub fn from_hex(hex: &str) -> Result<Self, KeyError> {
        let hex_str = if hex.starts_with("0x") {
            &hex[2..]
        } else {
            hex
        };

        let mut buf = [0u8; 65];
        hex::decode_to_slice(hex_str, &mut buf).map_err(|_| KeyError::InvalidHex)?;

        let pk = secp256k1::PublicKey::from_slice(&buf).map_err(|_| KeyError::InvalidSecp256k1)?;
        Ok(PublicKey(pk.serialize_uncompressed()))
    }

    /// Return the secp256k1 PublicKey.
    /// # Returns
    /// * `Ok(secp256k1::PublicKey)` - if the PublicKey is a valid secp256k1 PublicKey.
    /// * `Err(KeyError)` - if the PublicKey is not a valid secp256k1 PublicKey.
    pub fn to_secp256k1(&self) -> Result<secp256k1::PublicKey, KeyError> {
        secp256k1::PublicKey::from_slice(&self.0).map_err(|_| KeyError::InvalidSecp256k1)
    }

    /// Return bytes of the PublicKey.
    /// # Returns
    /// * `[u8; 65]` - if the PublicKey is a valid secp256k1 PublicKey.
    /// * `Err(KeyError)` - if the PublicKey is not a valid secp256k1 PublicKey.
    pub fn to_bytes(&self) -> Result<[u8; 65], KeyError> {
        let pk = self.to_secp256k1()?;
        Ok(pk.serialize_uncompressed())
    }

    /// Return hex string of the PublicKey.
    /// # Returns
    /// * `String` - if the PublicKey is a valid secp256k1 PublicKey.
    /// * `Err(KeyError)` - if the PublicKey is not a valid secp256k1 PublicKey.
    pub fn to_hex(&self) -> Result<String, KeyError> {
        let pk = self.to_secp256k1()?;
        Ok(hex::encode(pk.serialize_uncompressed()))
    }
}

/// Error type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyError {
    /// Invalid length.
    InvalidLength,
    /// Invalid secp256k1 SecretKey.
    InvalidSecp256k1,
    /// Invalid hex string.
    InvalidHex,
}

impl Display for KeyError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            KeyError::InvalidLength => write!(f, "Invalid length"),
            KeyError::InvalidSecp256k1 => write!(f, "Invalid secp256k1 SecretKey"),
            KeyError::InvalidHex => write!(f, "Invalid hex string"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_private_key_random() {
        let sk = PrivateKey::random();
        assert_eq!(sk.0.len(), 32);
    }

    #[test]
    fn test_private_key_from_secp256k1() {
        let mut buf = [0u8; 32];
        rand::thread_rng().fill_bytes(&mut buf);
        let sk = secp256k1::SecretKey::from_slice(&buf).unwrap();
        let sk = PrivateKey::from_secp256k1(sk);
        assert_eq!(sk.0.len(), 32);
    }

    #[test]
    fn test_private_key_from_slice() {
        let mut buf = [0u8; 32];
        rand::thread_rng().fill_bytes(&mut buf);
        let sk = PrivateKey::from_slice(&buf).unwrap();
        let sk1 = PrivateKey::from_slice(&sk.to_bytes().unwrap()).unwrap();
        assert_eq!(sk.0.len(), 32);
        assert_eq!(sk, sk1);
    }

    #[test]
    fn test_private_key_from_hex() {
        let sk = PrivateKey::from_hex(
            "0x0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
        )
        .unwrap();
        let sk1 = PrivateKey::from_hex(&sk.to_hex().unwrap()).unwrap();
        assert_eq!(sk.0.len(), 32);
        assert_eq!(sk, sk1);
    }

    #[test]
    fn test_private_key_to_secp256k1() {
        let sk = PrivateKey::random();
        let sk = sk.to_secp256k1().unwrap();
        assert_eq!(sk.serialize_secret().len(), 32);
    }

    #[test]
    fn test_private_key_to_bytes() {
        let sk = PrivateKey::random();
        let sk = sk.to_bytes().unwrap();
        assert_eq!(sk.len(), 32);
    }

    #[test]
    fn test_private_key_to_hex() {
        let sk = PrivateKey::random();
        let sk = sk.to_hex().unwrap();
        assert_eq!(sk.len(), 64);
    }

    #[test]
    fn test_private_key_to_public() {
        let sk = PrivateKey::random();
        let pk = sk.to_public().unwrap();
        assert_eq!(pk.0.len(), 65);
    }

    #[test]
    fn test_public_key_from_secp256k1() {
        let sk = PrivateKey::random();
        let pks = sk.to_public().unwrap().to_secp256k1().unwrap();
        let pk = PublicKey::from_secp256k1(pks);
        assert_eq!(pk.0.len(), 65);
    }

    #[test]
    fn test_public_key_from_slice() {
        let sk = PrivateKey::random();
        let pkb = sk.to_public().unwrap().to_bytes().unwrap();
        let pk = PublicKey::from_slice(&pkb).unwrap().to_bytes().unwrap();
        assert_eq!(pk.len(), 65);
        assert_eq!(pk, pkb);
    }

    #[test]
    fn test_public_key_from_hex() {
        let sk = PrivateKey::random();
        let pk = sk.to_public().unwrap();
        let pk1 = PublicKey::from_hex(&pk.to_hex().unwrap()).unwrap();
        assert_eq!(pk1.0.len(), 65);
        assert_eq!(pk, pk1);
    }

    #[test]
    fn test_public_key_to_secp256k1() {
        let sk = PrivateKey::random();
        let pk = sk.to_public().unwrap().to_secp256k1().unwrap();
        assert_eq!(pk.serialize_uncompressed().len(), 65);
    }

    #[test]
    fn test_public_key_to_bytes() {
        let sk = PrivateKey::random();
        let pk = sk.to_public().unwrap().to_bytes().unwrap();
        assert_eq!(pk.len(), 65);
    }

    #[test]
    fn test_public_key_to_hex() {
        let sk = PrivateKey::random();
        let pk = sk.to_public().unwrap().to_hex().unwrap();
        assert_eq!(pk.len(), 130);
    }
}
