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

use rand::RngCore;

use crate::address::Address;

/// Key traits.
pub trait Key {
    /// Return the key as a bytes.
    fn as_bytes(&self) -> &[u8];
    /// Return the key to a bytes.
    fn to_bytes(&self) -> Vec<u8>;
    /// Return the key to a string.
    fn to_string(&self) -> String;
    /// Return the key to a hex string.
    fn to_hex(&self) -> String;
}

/// secp256k1 PrivateKey wrapper.
/// This is a wrapper around a 32-byte private key.
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct PrivateKey(pub [u8; 32]);

/// PrivateKey implementation
impl PrivateKey {
    /// Generate a new random private key.
    pub fn new() -> PrivateKey {
        let mut ret = [0u8; 32];
        rand::thread_rng().fill_bytes(&mut ret);
        PrivateKey::from_slice(&ret).unwrap()
    }

    /// Create a new private key from a slice.
    pub fn from_slice(data: &[u8]) -> Result<PrivateKey, String> {
        if data.len() != 32 {
            return Err("Invalid private key length".to_string());
        }

        let mut ret = [0u8; 32];
        ret.copy_from_slice(data);
        let sk = secp256k1::SecretKey::from_slice(&ret).map_err(|e| e.to_string())?;
        Ok(PrivateKey(sk.serialize_secret()))
    }

    /// Create a new private key from a hexadecimal string.
    pub fn from_hex(hex: &str) -> Result<PrivateKey, String> {
        // if hex has 0x prefix, remove it
        let hex = if hex.starts_with("0x") {
            &hex[2..]
        } else {
            hex
        };

        let mut ret = [0u8; 32];
        hex::decode_to_slice(hex, &mut ret).map_err(|_| "Invalid hex string")?;
        PrivateKey::from_slice(&ret)
    }

    /// PublicKey from private key.
    pub fn public_key(&self) -> PublicKey {
        PublicKey::from_private(self)
    }

    /// Address from private key.
    pub fn address(&self) -> Address {
        Address::from_private_key(self)
    }
}

impl Key for PrivateKey {
    fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    fn to_bytes(&self) -> Vec<u8> {
        self.0.to_vec()
    }

    fn to_string(&self) -> String {
        hex::encode(self.0)
    }

    fn to_hex(&self) -> String {
        self.to_string()
    }
}

/// secp256k1 PublicKey wrapper.
/// This is a wrapper around a 65-byte public key.
/// The first byte is 0x04 (uncompressed) or 0x02 (compressed).
/// The remaining 64 bytes are the public key.
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct PublicKey(pub [u8; 65]);

/// PublicKey implementation
impl PublicKey {
    /// Create a new PublicKey from PrivateKey.
    pub fn from_private(sk: &PrivateKey) -> PublicKey {
        let secp = secp256k1::Secp256k1::new();
        let sk = secp256k1::SecretKey::from_slice(&sk.to_bytes()).unwrap();
        let pk = secp256k1::PublicKey::from_secret_key(&secp, &sk);
        PublicKey(pk.serialize_uncompressed())
    }

    /// Create a new PublicKey from a slice.
    pub fn from_slice(data: &[u8]) -> Result<PublicKey, String> {
        if data.len() != 65 {
            return Err("Invalid public key length".to_string());
        }

        let mut ret = [0u8; 65];
        ret.copy_from_slice(data);
        let pk = secp256k1::PublicKey::from_slice(&ret).map_err(|e| e.to_string())?;
        Ok(PublicKey(pk.serialize_uncompressed()))
    }

    /// Create a new PublicKey from a hexadecimal string.
    pub fn from_hex(hex: &str) -> Result<PublicKey, String> {
        let mut ret = [0u8; 65];
        hex::decode_to_slice(hex, &mut ret).map_err(|_| "Invalid hex string")?;
        PublicKey::from_slice(&ret)
    }

    /// Get Address from public key.
    pub fn address(&self) -> Address {
        Address::from_public_key(self)
    }
}

impl Key for PublicKey {
    fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    fn to_bytes(&self) -> Vec<u8> {
        self.0.to_vec()
    }

    fn to_string(&self) -> String {
        hex::encode(self.0)
    }

    fn to_hex(&self) -> String {
        self.to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_private_key() {
        let sk = PrivateKey::new();
        assert_eq!(sk.to_string().len(), 64);
        assert_eq!(sk.to_hex().len(), 64);
        assert_eq!(sk.to_bytes().len(), 32);
        assert_eq!(sk.public_key().to_string().len(), 130);
        assert_eq!(sk.public_key().to_hex().len(), 130);
        assert_eq!(sk.public_key().to_bytes().len(), 65);
        assert_eq!(sk.address().to_hex().len(), 42);
    }

    #[test]
    fn test_public_key() {
        let pk = PublicKey::from_private(&PrivateKey::new());
        assert_eq!(pk.to_string().len(), 130);
        assert_eq!(pk.to_hex().len(), 130);
        assert_eq!(pk.to_bytes().len(), 65);
        assert_eq!(pk.address().to_hex().len(), 42);
    }
}
