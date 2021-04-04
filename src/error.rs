//! Error handling for this crate.

/// Error type for this crate.
#[non_exhaustive]
#[derive(thiserror::Error, Clone, Debug)]
pub enum Error {
    /// Error on JS engine creation.
    #[error("failed to initialize js environment (detail: {0})")]
    JsInitError(String),
    /// Error on JS execution.
    #[error("failed to execute js (detail: {0})")]
    JsExecError(String),
    /// Error on JS value conversion.
    #[error("failed to convert js value (detail: {0})")]
    JsValueError(String),
}

/// Alias to `core::result::Result<T, katex::Error>`
pub type Result<T, E = Error> = core::result::Result<T, E>;
