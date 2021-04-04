//! This crate offers Rust bindings to [KaTeX](https://katex.org).
//! This allows you to render LaTeX equations to HTML.
//!
//! # Usage
//!
//! Add this to your `Cargo.toml`:
//! ```toml
//! [dependencies]
//! katex = "0.3"
//! ```
//!
//! # Examples
//!
//! ```
//! let html = katex::render("E = mc^2").unwrap();
//!
//! let opts = katex::Opts::builder().display_mode(true).build().unwrap();
//! let html_in_display_mode = katex::render_with_opts("E = mc^2", &opts).unwrap();
//! ```

pub mod opts;
pub use opts::{Opts, OptsBuilder, OutputType};

use quick_js::{self, Context as JsContext, JsValue};

const KATEX_SRC: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/vendor/katex.min.js"));
const MHCHEM_SRC: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/vendor/contrib/mhchem.min.js"
));

thread_local! {
    static KATEX: Result<JsContext> = init_katex();
}

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
pub type Result<T> = core::result::Result<T, Error>;

/// Initialize KaTeX js environment.
fn init_katex() -> Result<JsContext> {
    let ctx = JsContext::new()?;
    let _ = ctx.eval(KATEX_SRC)?;
    let _ = ctx.eval(MHCHEM_SRC)?;
    let _ = ctx.eval(
        r#"
    function renderToString(input, opts) {
        return katex.renderToString(input, opts);
    }
    "#,
    )?;
    Ok(ctx)
}

/// Render LaTeX equation to HTML with additional [options](`Opts`).
pub fn render_with_opts(input: &str, opts: impl AsRef<Opts>) -> Result<String> {
    KATEX.with(|ctx| {
        let ctx = match ctx.as_ref() {
            Ok(ctx) => ctx,
            Err(e) => return Err(e.clone()),
        };
        let opts = opts.as_ref();
        let args: Vec<JsValue> = vec![input.into(), opts.to_js_value()];
        let result = ctx
            .call_function("renderToString", args)?
            .into_string()
            .ok_or(quick_js::ValueError::UnexpectedType)?;
        Ok(result)
    })
}

/// Render LaTeX equation to HTML.
#[inline]
pub fn render(input: &str) -> Result<String> {
    render_with_opts(input, Opts::default())
}

#[cfg(test)]
mod tests;
