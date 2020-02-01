//! This crate offers Rust bindings to [KaTeX](https://katex.org).
//! This allows you to render LaTeX equations to HTML.
//!
//! # Usage
//!
//! Add this to your `Cargo.toml`:
//! ```toml
//! [dependencies]
//! howlong = "0.1"
//! ```
//!
//! # Examples
//!
//! ```
//! # use katex;
//! let html = katex::render("E = mc^2").unwrap();
//!
//! let opts = katex::Opts::builder().display_mode(true).build().unwrap();
//! let html_in_display_mode = katex::render_with_opts("E = mc^2", opts).unwrap();
//! ```

#[macro_use]
extern crate derive_builder;

use quick_js::{self, Context as JsContext, JsValue};
use std::collections::HashMap;

const KATEX_SRC: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/vendor/katex.min.js"));

thread_local! {
    static KATEX: Result<JsContext> = init_katex();
}

/// Error type for this crate.
#[non_exhaustive]
#[derive(thiserror::Error, Clone, Debug)]
pub enum Error {
    #[error("failed to initialize js environment (detail: {0})")]
    JsInitError(String),
    #[error("failed to execute js (detail: {0})")]
    JsExecError(String),
    #[error("js returns invalid result")]
    InvalidResult,
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

/// Alias to `core::result::Result<T, katex::Error>`
pub type Result<T> = core::result::Result<T, Error>;

/// Initialize KaTeX js environment.
fn init_katex() -> Result<JsContext> {
    let ctx = JsContext::new()?;
    let _ = ctx.eval(KATEX_SRC)?;
    let _ = ctx.eval("renderToString = katex.renderToString;")?;
    Ok(ctx)
}

/// Options to be passed to KaTeX.
///
/// Read <https://katex.org/docs/options.html> for more information.
#[derive(Clone, Builder, Debug)]
#[builder(setter(into))]
pub struct Opts {
    /// Whether to render the math in the display mode. Default is `false`.
    #[builder(default = "false")]
    pub display_mode: bool,
}

impl Opts {
    /// Return [`OptsBuilder`].
    pub fn builder() -> OptsBuilder {
        OptsBuilder::default()
    }
}

impl Default for Opts {
    fn default() -> Self {
        Self::builder().build().unwrap()
    }
}

impl Into<JsValue> for Opts {
    fn into(self) -> JsValue {
        let mut opt: HashMap<String, JsValue> = HashMap::new();
        opt.insert("displayMode".to_owned(), self.display_mode.into());
        JsValue::Object(opt)
    }
}

/// Render LaTeX equation to HTML with additional [options](`Opts`).
pub fn render_with_opts(input: &str, opts: Opts) -> Result<String> {
    KATEX.with(|ctx| {
        let ctx = match ctx.as_ref() {
            Ok(ctx) => ctx,
            Err(e) => return Err(e.clone()),
        };
        let args: Vec<JsValue> = vec![input.into(), opts.into()];
        let result = ctx.call_function("renderToString", args)?;
        result.into_string().ok_or_else(|| Error::InvalidResult)
    })
}

/// Render LaTeX equation to HTML.
#[inline]
pub fn render(input: &str) -> Result<String> {
    render_with_opts(input, Default::default())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render() {
        let html = render("a = b + c").unwrap();
        assert!(!html.contains(r#"span class="katex-display""#));
        assert!(html.contains(r#"span class="katex""#));
        assert!(html.contains(r#"span class="katex-mathml""#));
        assert!(html.contains(r#"span class="katex-html""#));
    }

    #[test]
    fn test_render_in_display_mode() {
        let opts = Opts::builder().display_mode(true).build().unwrap();
        let html = render_with_opts("a = b + c", opts).unwrap();
        assert!(html.contains(r#"span class="katex-display""#));
    }
}
