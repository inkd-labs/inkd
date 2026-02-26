use anchor_lang::prelude::*;

use crate::contexts::Revoke;
use crate::errors::InkdError;
use crate::events::AttestationRevoked;
use crate::state::AttestationStatus;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct RevokeParams {
    pub reason_code: u16,
}

pub fn handler(ctx: Context<Revoke>, params: RevokeParams) -> Result<()> {
    let issuer = &mut ctx.accounts.issuer;
    require_keys_eq!(
        issuer.authority,
        ctx.accounts.issuer_authority.key(),
        InkdError::UnauthorizedSigner
    );

    let attestation = &mut ctx.accounts.attestation;
    require!(
        attestation.status != AttestationStatus::Revoked as u8,
        InkdError::AlreadyRevoked
    );

    let now = Clock::get()?.unix_timestamp;
    attestation.set_revoked(now);

    issuer.revoked_count = issuer
        .revoked_count
        .checked_add(1)
        .ok_or(InkdError::MathOverflow)?;

    emit!(AttestationRevoked {
        attestation: attestation.key(),
        issuer: issuer.key(),
        reason_code: params.reason_code,
        timestamp: now,
    });

    Ok(())
}
