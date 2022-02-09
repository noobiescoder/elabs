# Elabs-k256
Elabs-k256 is a wrapper around the tiny_keccak::Keccak::v256() Hasher.
It give a simple interface to use the Hasher.
# Usage
To use elabs_k256, you need to import the `elabs_k256` crate and use the `k256` or `k256_hash` function.
```toml
[dependencies]
elabs_k256 = "0.1"
```
# Example
```rust
use elabs_k256::k256;
//!
fn main() {
   let input = "Hello World";
   let hash = k256(input);
   println!("{:?}", hash);
}
```
```rust
use elabs_k256::k256_hash;
//!
fn main() {
  let input = "Hello World";
  let mut hash = [0u8; 32];
  k256_hash(input, &mut hash);
  println!("{:?}", hash);
}
```
