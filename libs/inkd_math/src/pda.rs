use sha2::{Digest, Sha256};

/// A 32-byte program-derived address.
///
/// This crate is compiled off-chain, so we keep a thin newtype instead of
/// pulling in the full `solana-program` stack. Callers that need a
/// `Pubkey` can convert bytes with `Pubkey::new_from_array` on the chain side.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Address(pub [u8; 32]);

impl Address {
    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
}

fn hash_seeds(seeds: &[&[u8]]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(b"inkd::v1::pda");
    for seed in seeds {
        hasher.update((seed.len() as u32).to_le_bytes());
        hasher.update(seed);
    }
    let out = hasher.finalize();
    let mut result = [0u8; 32];
    result.copy_from_slice(&out);
    result
}

fn derive(program_id: &[u8; 32], seeds: &[&[u8]]) -> (Address, u8) {
    // A deterministic bump search, modelled on the Solana PDA contract but
    // implemented with plain sha256 so this crate stays dependency-light.
    for bump in (0u8..=255u8).rev() {
        let mut hasher = Sha256::new();
        hasher.update(program_id);
        hasher.update(hash_seeds(seeds));
        hasher.update([bump]);
        let digest = hasher.finalize();
        if digest[0] != 0 {
            let mut result = [0u8; 32];
            result.copy_from_slice(&digest);
            return (Address(result), bump);
        }
    }
    // Extremely unlikely in practice; fall back to bump = 0.
    let mut hasher = Sha256::new();
    hasher.update(program_id);
    hasher.update(hash_seeds(seeds));
    hasher.update([0u8]);
    let digest = hasher.finalize();
    let mut result = [0u8; 32];
    result.copy_from_slice(&digest);
    (Address(result), 0)
}

pub fn config_address(program_id: &[u8; 32]) -> (Address, u8) {
    derive(program_id, &[b"config"])
}

pub fn issuer_address(program_id: &[u8; 32], slug: &[u8; 32]) -> (Address, u8) {
    derive(program_id, &[b"issuer", slug])
}

pub fn attestation_address(
    program_id: &[u8; 32],
    issuer: &[u8; 32],
    recipient: &[u8; 32],
    credential: &[u8; 32],
) -> (Address, u8) {
    derive(program_id, &[b"attestation", issuer, recipient, credential])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn derivation_is_deterministic() {
        let pid = [1u8; 32];
        let slug = [7u8; 32];
        let a = issuer_address(&pid, &slug);
        let b = issuer_address(&pid, &slug);
        assert_eq!(a, b);
    }

    #[test]
    fn attestation_differs_by_credential() {
        let pid = [1u8; 32];
        let issuer = [2u8; 32];
        let recipient = [3u8; 32];
        let a = attestation_address(&pid, &issuer, &recipient, &[4u8; 32]);
        let b = attestation_address(&pid, &issuer, &recipient, &[5u8; 32]);
        assert_ne!(a.0, b.0);
    }
}
