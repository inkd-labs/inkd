use sha2::{Digest, Sha256};

/// Compute the deterministic Merkle leaf for an attestation.
///
/// The domain tag prevents second-preimage attacks across different protocol
/// versions. All inputs are concatenated in a fixed order and hashed with
/// `sha256` (the same hash used by Bubblegum compressed NFTs).
pub fn compute_leaf(
    issuer: &[u8; 32],
    recipient: &[u8; 32],
    credential: &[u8; 32],
    payload_hash: &[u8; 32],
    index: u64,
) -> [u8; 32] {
    let domain = b"inkd::v1::leaf";
    let idx = index.to_le_bytes();
    let mut hasher = Sha256::new();
    hasher.update(domain);
    hasher.update(issuer);
    hasher.update(recipient);
    hasher.update(credential);
    hasher.update(payload_hash);
    hasher.update(idx);
    let out = hasher.finalize();
    let mut result = [0u8; 32];
    result.copy_from_slice(&out);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deterministic_for_same_inputs() {
        let z = [0u8; 32];
        let a = compute_leaf(&z, &z, &z, &z, 0);
        let b = compute_leaf(&z, &z, &z, &z, 0);
        assert_eq!(a, b);
    }

    #[test]
    fn differs_by_index() {
        let z = [0u8; 32];
        let a = compute_leaf(&z, &z, &z, &z, 0);
        let b = compute_leaf(&z, &z, &z, &z, 1);
        assert_ne!(a, b);
    }
}
