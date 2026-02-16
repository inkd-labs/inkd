use anchor_lang::prelude::*;

use crate::contexts::MintAttestation;
use crate::errors::InkdError;
use crate::events::AttestationMinted;
use crate::state::{Attestation, AttestationStatus};
use inkd_math::leaf::compute_leaf;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct MintAttestationParams {
    pub recipient: Pubkey,
    pub credential: [u8; 32],
    pub expires_at: i64,
    pub payload_hash: [u8; 32],
}

pub fn handler(ctx: Context<MintAttestation>, params: MintAttestationParams) -> Result<()> {
    require!(!ctx.accounts.config.paused, InkdError::ProtocolPaused);

    let issuer = &mut ctx.accounts.issuer;
    require!(issuer.active, InkdError::IssuerInactive);
    require_keys_eq!(
        issuer.authority,
        ctx.accounts.issuer_authority.key(),
        InkdError::UnauthorizedSigner
    );

    let now = Clock::get()?.unix_timestamp;
    let index = issuer.issued_count;
    let leaf = compute_leaf(
        &issuer.key().to_bytes(),
        &params.recipient.to_bytes(),
        &params.credential,
        &params.payload_hash,
        index,
    );

    let attestation = &mut ctx.accounts.attestation;
    attestation.issuer = issuer.key();
    attestation.recipient = params.recipient;
    attestation.leaf = leaf;
    attestation.credential = params.credential;
    attestation.issued_at = now;
    attestation.expires_at = params.expires_at;
    attestation.revoked_at = 0;
    attestation.index = index;
    attestation.bump = ctx.bumps.attestation;
    attestation.status = AttestationStatus::Active as u8;

    issuer.issued_count = issuer
        .issued_count
        .checked_add(1)
        .ok_or(InkdError::MathOverflow)?;

    let config = &mut ctx.accounts.config;
    config.attestation_count = config
        .attestation_count
        .checked_add(1)
        .ok_or(InkdError::MathOverflow)?;

    emit!(AttestationMinted {
        attestation: attestation.key(),
        issuer: issuer.key(),
        recipient: params.recipient,
        leaf,
        index,
        timestamp: now,
    });

    Ok(())
}

impl Attestation {
    pub fn set_revoked(&mut self, now: i64) {
        self.status = AttestationStatus::Revoked as u8;
        self.revoked_at = now;
    }
}
