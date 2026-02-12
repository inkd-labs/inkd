use anchor_lang::prelude::*;

use crate::contexts::Initialize;
use crate::errors::InkdError;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct InitializeParams {
    pub mint_fee_lamports: u64,
}

pub fn handler(ctx: Context<Initialize>, params: InitializeParams) -> Result<()> {
    let config = &mut ctx.accounts.config;
    require!(params.mint_fee_lamports >= 1, InkdError::FeeTooLow);

    config.authority = ctx.accounts.authority.key();
    config.treasury = ctx.accounts.treasury.key();
    config.issuer_count = 0;
    config.attestation_count = 0;
    config.mint_fee_lamports = params.mint_fee_lamports;
    config.bump = ctx.bumps.config;
    config.paused = false;

    Ok(())
}
