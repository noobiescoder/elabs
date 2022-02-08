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

use elabs_k256::k256_hash;

use crate::{
    key::{PrivateKey, PublicKey},
    Key,
};

/// The 20 byte Ethereum address.
pub struct Address([u8; 20]);

impl Address {
    /// Creates a new address from PublicKey.
    pub fn from_public_key(public_key: &PublicKey) -> Address {
        let mut buf = [0u8; 32];
        k256_hash(&public_key.to_bytes()[1..], &mut buf);
        let mut address = [0u8; 20];
        address.copy_from_slice(&buf[12..]);
        Address(address)
    }

    /// Create a new address from PrivateKey.
    pub fn from_private_key(private_key: &PrivateKey) -> Address {
        let public_key = private_key.public_key();
        Address::from_public_key(&public_key)
    }

    /// Returns the address as a bytes.
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    /// Returns the address as a hex string.
    /// The address should be prefixed with 0x and checksummed to eip-55.
    pub fn to_hex(&self) -> String {
        let address = self
            .as_bytes()
            .iter()
            .map(|b| format!("{:02x}", b))
            .collect::<Vec<String>>()
            .join("");

        let mut buf = [0u8; 32];
        k256_hash(&address.as_bytes(), &mut buf);

        let hash = buf
            .iter()
            .map(|b| format!("{:02x}", b))
            .collect::<Vec<String>>()
            .join("");

        address
            .char_indices()
            .fold(String::from("0x"), |mut acc, (i, c)| {
                let n = u16::from_str_radix(&hash[i..i + 1], 16).unwrap();

                if n > 7 {
                    acc.push_str(&c.to_uppercase().to_string());
                } else {
                    acc.push(c);
                }

                acc
            })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_address_from_public_key() {
        let private_key = PrivateKey::from_hex(
            "0x4c0883a69102937d6231471b5dbb6204fe5129617082792ae468d01a3f362318",
        )
        .unwrap();
        let public_key = private_key.public_key();
        let address = Address::from_public_key(&public_key);

        assert_eq!(
            address.to_hex().to_lowercase(),
            "0x2c7536e3605d9c16a7a3d7b1898e529396a65c23"
        );
    }
}
