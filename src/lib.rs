//! This crate offers Rust bindings to [KaTeX](https://katex.org).
//! This allows you to render LaTeX equations to HTML.
//!
//! # Usage
//!
//! Add this to your `Cargo.toml`:
//! ```toml
//! [dependencies]
//! katex = "0.4.0-alpha.2"
//! ```
//!
//! This crate offers the following features:
//!
//! * `quick-js`: Enable by default. Use [quick-js](https://crates.io/crates/quick-js) as the JS backend.
//! * `duktape`: Use [duktape](https://crates.io/crates/ducc) as the JS backend. You need to disable the default features to enable this backend.
//!
//! # Examples
//!
//! ```
//! let html = katex::render("E = mc^2").unwrap();
//!
//! let opts = katex::Opts::builder().display_mode(true).build().unwrap();
//! let html_in_display_mode = katex::render_with_opts("E = mc^2", &opts).unwrap();
//! ```

#![forbid(unsafe_code)]
#![deny(missing_docs)]

use core::cell::RefCell;

pub mod error;
pub use error::{Error, Result};

pub mod opts;
pub use opts::{Opts, OptsBuilder, OutputType};

mod js_engine;
use js_engine::{Engine, JsEngine, JsScope, JsValue, Scope};

/// JS source code.
const JS_SRC: &str = concat!(
    // KaTeX JS source code
    include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/vendor/katex.min.js")),
    // mhchem JS source code
    include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/vendor/contrib/mhchem.min.js"
    )),
    // entry function
    "function renderToString(input, opts) { return katex.renderToString(input, opts); }"
);

thread_local! {
    /// Per thread JS Engine used to render KaTeX.
    static KATEX: Result<RefCell<Engine>> = init_katex();
}

/// Initialize KaTeX js environment.
fn init_katex() -> Result<RefCell<Engine>> {
    let mut engine = Engine::new()?;
    let scope = Scope::global_scope(&mut engine);
    scope.eval(JS_SRC)?;
    Ok(RefCell::new(engine))
}

/// Render LaTeX equation to HTML using specified [engine](`JsEngine`) and [options](`Opts`).
#[inline]
fn render_inner(engine: &mut Engine, input: &str, opts: impl AsRef<Opts>) -> Result<String> {
    use core::iter;

    let scope = Scope::global_scope(engine);
    let input = scope.create_string_value(input.to_owned())?;
    let opts = opts.as_ref().to_js_value(&scope)?;
    let args = iter::once(input).chain(iter::once(opts));
    let result = scope.call_function("renderToString", args)?;
    result.into_string()
}

/// Render LaTeX equation to HTML with additional [options](`Opts`).
pub fn render_with_opts(input: &str, opts: impl AsRef<Opts>) -> Result<String> {
    KATEX.with(|engine| {
        engine
            .as_ref()
            .map_err(|e| e.clone())
            .and_then(|engine| render_inner(&mut *engine.borrow_mut(), input, opts))
    })
}

/// Render LaTeX equation to HTML.
#[inline]
pub fn render(input: &str) -> Result<String> {
    render_with_opts(input, Opts::default())
}

#[cfg(test)]
mod tests;
