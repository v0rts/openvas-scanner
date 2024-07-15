use thiserror::Error;

use nasl_syntax::{LoadError, Statement};

use crate::verify;

use super::Replace;

#[derive(Debug, Clone, PartialEq, Eq, Error)]
/// Error during transpiling
pub enum TranspileError {
    /// Loader is unable to handle operation
    #[error("Load error: {0}")]
    LoadError(#[from] LoadError),
    /// Describes an error while verifying the file
    #[error("Verify error: {0}")]
    VerifyError(#[from] verify::Error),
    /// Describes an error while verifying the file
    #[error("Replace error: {0}")]
    ReplaceError(#[from] ReplaceError),
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
/// Error cases on a replace operation
pub enum ReplaceError {
    /// The replace operation is invalid on statement
    #[error("Operation {0} not allowed on {1}.")]
    Unsupported(Replace, Statement),
}
