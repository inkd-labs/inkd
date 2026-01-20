use anchor_lang::prelude::*;

#[error_code]
pub enum InkdError {
    #[msg("The provided authority does not match the expected signer.")]
    UnauthorizedSigner,
    #[msg("The issuer account is not active.")]
    IssuerInactive,
    #[msg("The attestation has already been revoked.")]
    AlreadyRevoked,
    #[msg("The attestation has expired and can no longer be verified.")]
    Expired,
    #[msg("Merkle leaf mismatch between provided proof and stored record.")]
    LeafMismatch,
    #[msg("Credential slug is longer than the maximum of 32 bytes.")]
    CredentialTooLong,
    #[msg("Issuer slug is longer than the maximum of 32 bytes.")]
    IssuerSlugTooLong,
    #[msg("Protocol is currently paused by authority.")]
    ProtocolPaused,
    #[msg("Provided fee is below the configured minimum.")]
    FeeTooLow,
    #[msg("Numerical overflow when updating counter.")]
    MathOverflow,
    #[msg("Attestation index is out of bounds for the configured tree depth.")]
    IndexOutOfRange,
}
