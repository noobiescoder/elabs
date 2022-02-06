use tiny_keccak::{Hasher, Keccak};

/// Compute in place the Keccak-256 hash of the given slice.
pub fn k256(data: &mut [u8]) {
    let mut hasher = Keccak::v256();
    hasher.update(data);
    hasher.finalize(data);
}

/// Compute the Keccak-256 hash of the given slice.
/// The result is returned to the given vector.
pub fn k256_hash(data: &[u8], out: &mut [u8]) {
    let mut hasher = Keccak::v256();
    hasher.update(data);
    hasher.finalize(out);
}

#[cfg(test)]
mod test {
    use super::*;

    // Test k256 and k256_hash.
    #[test]
    fn test_k256() {
        let mut data = [0u8; 32];
        let mut out = [0u8; 32];

        // Test with empty data.
        k256(&mut data);
        k256_hash(&[0u8; 32], &mut out);
        assert_eq!(data, out);
    }
}
