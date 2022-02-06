use std::fmt::Display;

use rand::RngCore;

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
        let mut ret = [0u8; 32];
        hex::decode_to_slice(hex, &mut ret).map_err(|_| "Invalid hex string")?;
        PrivateKey::from_slice(&ret)
    }

    /// Get the private key as bytes.
    pub fn to_bytes(&self) -> [u8; 32] {
        self.0
    }

    /// PublicKey from private key.
    pub fn public_key(&self) -> PublicKey {
        PublicKey::from_private(self)
    }
}

impl AsRef<[u8]> for PrivateKey {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl Display for PrivateKey {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", hex::encode(&self.0))
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

    /// Get the public key as bytes.
    pub fn to_bytes(&self) -> [u8; 65] {
        self.0
    }

    /// Get the public key as a hexadecimal string.
    pub fn to_hex(&self) -> String {
        hex::encode(&self.0)
    }
}

impl Display for PublicKey {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for i in 0..65 {
            write!(f, "{:02x}", self.0[i])?;
        }
        Ok(())
    }
}
