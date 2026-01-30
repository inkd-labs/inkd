//! Math and layout helpers for the Inkd protocol.
//!
//! This crate is intentionally dependency-light so it can be imported from the
//! on-chain program, the CLI, and any off-chain tooling without pulling in
//! Anchor or client stacks.

pub mod index;
pub mod leaf;
pub mod pda;

pub use index::TreeGeometry;
pub use leaf::compute_leaf;
pub use pda::{attestation_address, config_address, issuer_address};
