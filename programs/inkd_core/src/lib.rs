use anchor_lang::prelude::*;

pub mod contexts;
pub mod errors;
pub mod events;
pub mod instructions;
pub mod state;

pub use contexts::*;
pub use errors::*;
pub use events::*;
pub use instructions::*;
pub use state::*;

declare_id!("6VKQVC6RQj5wDnXQ1pVfuTGga2iUUjFPMxAHSN6fjPck");

/// The core on-chain program for Inkd.
///
/// The protocol exposes a small set of instructions that together implement a
/// soulbound attestation primitive on top of compressed NFTs. Transfers are
/// disabled at the account level; only the issuing authority can revoke.
#[program]
pub mod inkd_core {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, params: InitializeParams) -> Result<()> {
        instructions::initialize::handler(ctx, params)
    }

    pub fn register_issuer(
        ctx: Context<RegisterIssuer>,
        params: RegisterIssuerParams,
    ) -> Result<()> {
        instructions::register_issuer::handler(ctx, params)
    }

    pub fn mint_attestation(
        ctx: Context<MintAttestation>,
        params: MintAttestationParams,
    ) -> Result<()> {
        instructions::mint_attestation::handler(ctx, params)
    }

    pub fn verify(ctx: Context<Verify>, params: VerifyParams) -> Result<()> {
        instructions::verify::handler(ctx, params)
    }

    pub fn revoke(ctx: Context<Revoke>, params: RevokeParams) -> Result<()> {
        instructions::revoke::handler(ctx, params)
    }
}
