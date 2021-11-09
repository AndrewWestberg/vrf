//! Crate specific errors
use thiserror::Error;

/// VRF related errors
#[derive(Debug, Error, PartialEq, Eq, Clone)]
pub enum VrfError {
    /// This error occurs when the VRF verification failed
    #[error("VRF verification failed")]
    VerificationFailed,
    /// This error occurs when an `EdwardsPoint` decompression fails
    #[error("Decompression failed")]
    DecompressionFailed,
    /// This error occurs when a public key has small order
    #[error("PK has small order")]
    PkSmallOrder,
}
