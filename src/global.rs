//! Shared crate-wide types.

/// Standard result type returned by the SDK.
pub type Result<T> = std::result::Result<T, crate::errors::FrontalError>;
