use anchor_lang::prelude::*;

/// Global protocol configuration PDA.
///
/// There is exactly one `Config` per deployment. It acts as the anchor for
/// every issuer and every attestation and tracks the authority that is allowed
/// to roll out protocol-level changes.
#[account]
#[derive(Default)]
pub struct Config {
    pub authority: Pubkey,
    pub treasury: Pubkey,
    pub issuer_count: u64,
    pub attestation_count: u64,
    pub mint_fee_lamports: u64,
    pub bump: u8,
    pub paused: bool,
    pub _padding: [u8; 6],
}

impl Config {
    pub const LEN: usize = 32 + 32 + 8 + 8 + 8 + 1 + 1 + 6;
    pub const SEED: &'static [u8] = b"config";
}

/// An approved issuing authority (a DAO, protocol, or individual).
#[account]
#[derive(Default)]
pub struct Issuer {
    pub authority: Pubkey,
    pub slug: [u8; 32],
    pub issued_count: u64,
    pub revoked_count: u64,
    pub created_at: i64,
    pub bump: u8,
    pub active: bool,
    pub _padding: [u8; 6],
}

impl Issuer {
    pub const LEN: usize = 32 + 32 + 8 + 8 + 8 + 1 + 1 + 6;
    pub const SEED: &'static [u8] = b"issuer";
}

/// A single soulbound attestation account.
///
/// The `leaf` field is the Bubblegum Merkle leaf for the compressed NFT and is
/// used by the verifier together with the issuer pubkey to reconstruct proofs.
#[account]
#[derive(Default)]
pub struct Attestation {
    pub issuer: Pubkey,
    pub recipient: Pubkey,
    pub leaf: [u8; 32],
    pub credential: [u8; 32],
    pub issued_at: i64,
    pub expires_at: i64,
    pub revoked_at: i64,
    pub index: u64,
    pub bump: u8,
    pub status: u8,
    pub _padding: [u8; 6],
}

impl Attestation {
    pub const LEN: usize = 32 + 32 + 32 + 32 + 8 + 8 + 8 + 8 + 1 + 1 + 6;
    pub const SEED: &'static [u8] = b"attestation";

    pub fn is_active(&self, now: i64) -> bool {
        if self.status != AttestationStatus::Active as u8 {
            return false;
        }
        if self.expires_at > 0 && self.expires_at <= now {
            return false;
        }
        true
    }
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AttestationStatus {
    Pending = 0,
    Active = 1,
    Revoked = 2,
    Expired = 3,
}
