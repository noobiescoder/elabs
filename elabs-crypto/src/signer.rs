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

use secp256k1::{
    ecdsa::{self, RecoverableSignature, RecoveryId},
    Error, Message, Secp256k1,
};

use crate::*;

/// Return the public key that created the given signature.
/// # Arguments
/// * `hash` - The hash of the message.
/// * `signature` - The signature.
/// * `recovery_id` - The recovery id.
/// # Returns
/// * `Ok(PublicKey)` - The public key that created the signature.
/// * `Err(Error)` - The error that occurred.
pub fn ecrecover(hash: &[u8], signature: &[u8], recovery_id: u8) -> Result<PublicKey, Error> {
    let secp = Secp256k1::new();
    let id = RecoveryId::from_i32(recovery_id as i32)?;
    let sig = RecoverableSignature::from_compact(&signature, id)?;
    let msgb = Message::from_slice(&hash)?;
    let pk = secp.recover_ecdsa(&msgb, &sig)?;
    Ok(PublicKey::from_secp256k1(pk))
}

/// Sign a message with the given private key.
/// # Arguments
/// * `msg` - The message.
/// * `private_key` - The private key.
/// # Returns
/// * `Ok(Signature)` - The signature.
/// * `Err(Error)` - The error that occurred.
pub fn sign(msg: &[u8], private_key: PrivateKey) -> Result<RecoverableSignature, Error> {
    let secp = Secp256k1::new();
    let hash = keccak256(msg);
    let msgb = Message::from_slice(&hash)?;
    Ok(secp.sign_ecdsa_recoverable(&msgb, &private_key.to_secp256k1().unwrap()))
}

/// Verify a signature with the given public key.
/// # Arguments
/// * `msg` - The message.
/// * `signature` - The signature.
/// * `public_key` - The public key.
/// # Returns
/// * `Ok(bool)` - Whether the signature is valid.
/// * `Err(Error)` - The error that occurred.
pub fn verify(msg: &[u8], signature: &[u8], public_key: PublicKey) -> Result<bool, Error> {
    let secp = Secp256k1::new();
    let hash = keccak256(msg);
    let msgb = Message::from_slice(&hash)?;
    let sig = ecdsa::Signature::from_compact(&signature)?;
    let verify = secp.verify_ecdsa(&msgb, &sig, &public_key.to_secp256k1().unwrap());
    Ok(verify.is_ok())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_ecrecover() {
        let msg = b"hello world";
        let hash = keccak256(msg);
        let sk = PrivateKey::random();
        let pk = sk.to_public().unwrap();
        let sig = sign(msg, sk).unwrap();
        let (recid, bsig) = sig.serialize_compact();
        let pk2 = ecrecover(&hash, &bsig, recid.to_i32() as u8).unwrap();
        assert_eq!(pk, pk2);
    }

    #[test]
    fn test_sign_verify() {
        let msg = b"hello world";
        let hash = keccak256(msg);
        let sk = PrivateKey::random();
        let pk = sk.to_public().unwrap();
        let sig = sign(&hash, sk).unwrap();
        assert!(verify(&hash, &sig.serialize_compact().1, pk).unwrap());
    }
}
