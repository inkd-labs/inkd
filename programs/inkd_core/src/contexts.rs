use anchor_lang::prelude::*;

use crate::state::*;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + Config::LEN,
        seeds = [Config::SEED],
        bump,
    )]
    pub config: Account<'info, Config>,
    /// CHECK: treasury is a plain account used to receive protocol fees.
    pub treasury: UncheckedAccount<'info>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(params: crate::instructions::register_issuer::RegisterIssuerParams)]
pub struct RegisterIssuer<'info> {
    #[account(
        mut,
        seeds = [Config::SEED],
        bump = config.bump,
    )]
    pub config: Account<'info, Config>,
    #[account(
        init,
        payer = authority,
        space = 8 + Issuer::LEN,
        seeds = [Issuer::SEED, params.slug.as_ref()],
        bump,
    )]
    pub issuer: Account<'info, Issuer>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(params: crate::instructions::mint_attestation::MintAttestationParams)]
pub struct MintAttestation<'info> {
    #[account(
        mut,
        seeds = [Config::SEED],
        bump = config.bump,
    )]
    pub config: Account<'info, Config>,
    #[account(
        mut,
        seeds = [Issuer::SEED, issuer.slug.as_ref()],
        bump = issuer.bump,
    )]
    pub issuer: Account<'info, Issuer>,
    #[account(
        init,
        payer = payer,
        space = 8 + Attestation::LEN,
        seeds = [
            Attestation::SEED,
            issuer.key().as_ref(),
            params.recipient.as_ref(),
            params.credential.as_ref(),
        ],
        bump,
    )]
    pub attestation: Account<'info, Attestation>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub issuer_authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Verify<'info> {
    #[account(
        seeds = [Config::SEED],
        bump = config.bump,
    )]
    pub config: Account<'info, Config>,
    #[account(
        seeds = [Issuer::SEED, issuer.slug.as_ref()],
        bump = issuer.bump,
    )]
    pub issuer: Account<'info, Issuer>,
    pub attestation: Account<'info, Attestation>,
    pub verifier: Signer<'info>,
}

#[derive(Accounts)]
pub struct Revoke<'info> {
    #[account(
        mut,
        seeds = [Issuer::SEED, issuer.slug.as_ref()],
        bump = issuer.bump,
    )]
    pub issuer: Account<'info, Issuer>,
    #[account(mut)]
    pub attestation: Account<'info, Attestation>,
    pub issuer_authority: Signer<'info>,
}
