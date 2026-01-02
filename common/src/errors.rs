#[cfg(feature = "std")]
use thiserror::Error;

#[derive(Debug)]
#[cfg_attr(feature = "std", derive(Error))]
pub enum ProtocolError {
    #[cfg_attr(feature = "std", error("invalid message format"))]
    InvalidFormat,

    #[cfg_attr(feature = "std", error("state mismatch"))]
    StateMismatch,

    #[cfg_attr(feature = "std", error("invalid nonce"))]
    InvalidNonce,

    #[cfg_attr(feature = "std", error("proof verification failed"))]
    InvalidProof,

    #[cfg_attr(feature = "std", error("policy violation"))]
    PolicyViolation,

    #[cfg_attr(feature = "std", error("commitment mismatch"))]
    CommitmentMismatch,
}
