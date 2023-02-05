//! This crate offers Rust bindings to [KaTeX](https://katex.org).
//! This allows you to render LaTeX equations to HTML.
//!
//! # Usage
//!
//! Add this to your `Cargo.toml`:
//! ```toml
//! [dependencies]
//! katex = "0.4"
//! ```
//!
//! This crate offers the following features:
//!
//! * `quick-js`: Enable by default. Use [quick-js](https://crates.io/crates/quick-js)
//!    as the JS backend.
//! * `duktape`: Use [duktape](https://crates.io/crates/ducc) as the JS backend.
//!    You need to disable the default features to enable this backend.
//! * `wasm-js`: Use [wasm-bindgen](https://crates.io/crates/wasm-bindgen) and
//!    [js-sys](https://crates.io/crates/js-sys) as the JS backend.
//!    You need to disable the default features to enable this backend.
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

pub mod error;
pub use error::{Error, Result};

pub mod opts;
pub use opts::{Opts, OptsBuilder, OutputType};

mod js_engine;
use js_engine::{Engine, JsEngine, JsValue};

/// KaTeX version.
pub const KATEX_VERSION: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/KATEX-VERSION"));

/// JS source code.
const JS_SRC: &str = concat!(
    // HACK to load KaTeX code in Node.js
    // By setting `module` and `exports` as undefined, we prevent KaTeX to
    // be loaded like normal Node.js module.
    include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/js/node-hack.js")),
    // KaTeX JS source code
    include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/vendor/katex.min.js")),
    // mhchem JS source code
    include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/vendor/contrib/mhchem.min.js"
    )),
    // restore HACK done in node-hack.js
    include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/js/post-node-hack.js")),
    // entry function
    include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/js/entry.js")),
);

thread_local! {
    /// Per thread JS Engine used to render KaTeX.
    static KATEX: Result<Engine> = init_katex();
}

/// Initialize KaTeX js environment.
fn init_katex<E>() -> Result<E>
where
    E: JsEngine,
{
    let engine = E::new()?;
    engine.eval(JS_SRC)?;
    Ok(engine)
}

/// Render LaTeX equation to HTML using specified [engine](`JsEngine`) and [options](`Opts`).
#[inline]
fn render_inner<E>(engine: &E, input: &str, opts: impl AsRef<Opts>) -> Result<String>
where
    E: JsEngine,
{
    use core::iter;

    let input = engine.create_string_value(input.to_owned())?;
    let opts = opts.as_ref().to_js_value(engine)?;
    let args = iter::once(input).chain(iter::once(opts));
    let result = engine.call_function("katexRenderToString", args)?;
    result.into_string()
}

/// Render LaTeX equation to HTML with additional [options](`Opts`).
pub fn render_with_opts(input: &str, opts: impl AsRef<Opts>) -> Result<String> {
    KATEX.with(|engine| {
        engine
            .as_ref()
            .map_err(|e| e.clone())
            .and_then(|engine| render_inner(engine, input, opts))
    })
}

/// Render LaTeX equation to HTML.
#[inline]
pub fn render(input: &str) -> Result<String> {
    render_with_opts(input, Opts::default())
}

#[cfg(test)]
mod tests;
