# Elabs Crypto.
This crate provides a set of cryptographic helper functions.
It provide `secp256k1` PublicKey, PrivateKey, signer, and `hash` functions.
This crate is based on [`secp256k1`] and [`tiny-keccak`].

## Usage
```toml
[dependencies]
elabs-crypto = "0.1"
```
## Example
```rust
use elabs_crypto::*;

fn main() {
let msg = b"hello world";
	let hash = keccak256(msg);
	let sk = PrivateKey::random();
	let pk = sk.to_public().unwrap();
	let sig = sign(msg, sk).unwrap();
	let (recid, bsig) = sig.serialize_compact();
	let pk2 = ecrecover(&hash, &bsig, recid.to_i32() as u8).unwrap();
	assert_eq!(pk, pk2);
}
```

