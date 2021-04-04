/// Error type for this crate.
#[non_exhaustive]
#[derive(thiserror::Error, Clone, Debug)]
pub enum Error {
    /// Error on js context creation. See [`quick_js::ContextError`].
    #[error("failed to initialize js environment (detail: {0})")]
    JsInitError(String),
    /// Error on js execution. See [`quick_js::ExecutionError`].
    #[error("failed to execute js (detail: {0})")]
    JsExecError(String),
    /// Error on js value conversion. See [`quick_js::ValueError`].
    #[error("failed to convert js value (detail: {0})")]
    JsValueError(String),
}

impl From<quick_js::ContextError> for Error {
    fn from(e: quick_js::ContextError) -> Self {
        Self::JsInitError(format!("{}", e))
    }
}

impl From<quick_js::ExecutionError> for Error {
    fn from(e: quick_js::ExecutionError) -> Self {
        Self::JsExecError(format!("{}", e))
    }
}

impl From<quick_js::ValueError> for Error {
    fn from(e: quick_js::ValueError) -> Self {
        Self::JsValueError(format!("{}", e))
    }
}

/// Alias to `core::result::Result<T, katex::Error>`
pub type Result<T, E = Error> = core::result::Result<T, E>;
