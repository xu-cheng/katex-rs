//! This crate offers Rust bindings to [KaTeX](https://katex.org).
//! This allows you to render LaTeX equations to HTML.
//!
//! # Usage
//!
//! Add this to your `Cargo.toml`:
//! ```toml
//! [dependencies]
//! katex = "0.2"
//! ```
//!
//! # Examples
//!
//! ```
//! let html = katex::render("E = mc^2").unwrap();
//!
//! let opts = katex::Opts::builder().display_mode(true).build().unwrap();
//! let html_in_display_mode = katex::render_with_opts("E = mc^2", opts).unwrap();
//! ```

#[macro_use]
extern crate derive_builder;

use core::convert::TryFrom;
use core::fmt;
use quick_js::{self, Context as JsContext, JsValue};
use std::collections::HashMap;
use std::panic::RefUnwindSafe;
use std::sync::Arc;

const KATEX_SRC: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/vendor/katex.min.js"));

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
    let _ = ctx.eval(
        r#"
    function renderToString(input, opts) {
        if (opts.trust === "USE_TRUST_CALLBACK") {
            opts.trust = trustCallback;
        }
        return katex.renderToString(input, opts);
    }
    "#,
    )?;
    Ok(ctx)
}

/// The input used by the [`TrustCallback`].
/// See [`OptsBuilder::trust_callback`].
#[derive(Debug)]
pub struct TrustContext<'a> {
    pub command: &'a str,
    pub url: &'a str,
    pub protocol: &'a str,
}

impl<'a> TryFrom<&'a JsValue> for TrustContext<'a> {
    type Error = quick_js::ValueError;

    fn try_from(input: &'a JsValue) -> core::result::Result<Self, Self::Error> {
        match input {
            JsValue::Object(obj) => {
                let command = obj
                    .get("command")
                    .ok_or_else(|| quick_js::ValueError::UnexpectedType)?
                    .as_str()
                    .ok_or_else(|| quick_js::ValueError::UnexpectedType)?;
                let url = obj
                    .get("url")
                    .ok_or_else(|| quick_js::ValueError::UnexpectedType)?
                    .as_str()
                    .ok_or_else(|| quick_js::ValueError::UnexpectedType)?;
                let protocol = obj
                    .get("protocol")
                    .ok_or_else(|| quick_js::ValueError::UnexpectedType)?
                    .as_str()
                    .ok_or_else(|| quick_js::ValueError::UnexpectedType)?;
                Ok(Self {
                    command,
                    url,
                    protocol,
                })
            }
            _ => Err(quick_js::ValueError::UnexpectedType),
        }
    }
}

/// A callback function to determine whether to trust users' input.
/// It accepts [`TrustContext`] and returns a [`bool`].
/// See [`OptsBuilder::trust_callback`].
#[derive(Clone)]
pub struct TrustCallback(Arc<dyn Fn(TrustContext) -> bool + RefUnwindSafe>);

impl fmt::Debug for TrustCallback {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Fn(TrustContext) -> bool")
    }
}

impl<F: Fn(TrustContext) -> bool + RefUnwindSafe + 'static> From<F> for TrustCallback {
    fn from(f: F) -> Self {
        Self(Arc::from(f))
    }
}

impl quick_js::Callback<TrustCallback> for TrustCallback {
    fn argument_count(&self) -> usize {
        1
    }

    fn call(
        &self,
        args: Vec<JsValue>,
    ) -> core::result::Result<core::result::Result<JsValue, String>, quick_js::ValueError> {
        let arg = args
            .get(0)
            .ok_or_else(|| quick_js::ValueError::UnexpectedType)?;
        let ctx = TrustContext::try_from(arg)?;
        let result = self.0(ctx);
        Ok(Ok(JsValue::from(result)))
    }
}

/// Options to be passed to KaTeX.
///
/// Read <https://katex.org/docs/options.html> for more information.
#[non_exhaustive]
#[derive(Clone, Builder, Debug, Default)]
#[builder(default)]
#[builder(setter(into, strip_option))]
#[builder(build_fn(validate = "Self::validate"))]
pub struct Opts {
    /// Whether to render the math in the display mode.
    display_mode: Option<bool>,
    /// KaTeX output type.
    output_type: Option<OutputType>,
    /// Whether to have `\tags` rendered on the left instead of the right.
    leqno: Option<bool>,
    /// Whether to make display math flush left.
    fleqn: Option<bool>,
    /// Whether to let KaTeX throw a ParseError for invalid LaTeX.
    throw_on_error: Option<bool>,
    /// Color used for invalid LaTeX.
    error_color: Option<String>,
    /// Collection of custom macros.
    /// Read <https://katex.org/docs/options.html> for more information.
    macros: HashMap<String, String>,
    /// Specifies a minimum thickness, in ems.
    /// Read <https://katex.org/docs/options.html> for more information.
    min_rule_thickness: Option<f64>,
    /// Max size for user-specified sizes.
    /// If set to `None`, users can make elements and spaces arbitrarily large.
    /// Read <https://katex.org/docs/options.html> for more information.
    #[allow(clippy::option_option)]
    max_size: Option<Option<f64>>,
    /// Limit the number of macro expansions to the specified number.
    /// If set to `None`, the macro expander will try to fully expand as in LaTeX.
    /// Read <https://katex.org/docs/options.html> for more information.
    #[allow(clippy::option_option)]
    max_expand: Option<Option<i32>>,
    /// Whether to trust users' input.
    /// Cannot be assigned at the same time with [`OptsBuilder::trust_callback`].
    /// Read <https://katex.org/docs/options.html> for more information.
    trust: Option<bool>,
    /// A callback function to determine whether to trust users' input.
    /// Cannot be assigned at the same time with [`OptsBuilder::trust`].
    /// Read <https://katex.org/docs/options.html> for more information.
    ///
    /// # Examples
    ///
    /// ```
    /// let opts = katex::Opts::builder()
    ///     .trust_callback(|ctx: katex::TrustContext| -> bool {
    ///         ctx.command == r#"\url"#
    ///     })
    ///     .build()
    ///     .unwrap();
    /// ```
    trust_callback: Option<TrustCallback>,
}

impl Opts {
    /// Return [`OptsBuilder`].
    pub fn builder() -> OptsBuilder {
        OptsBuilder::default()
    }

    /// Set whether to render the math in the display mode.
    pub fn set_display_mode(&mut self, flag: bool) {
        self.display_mode = Some(flag);
    }

    /// Set KaTeX output type.
    pub fn set_output_type(&mut self, output_type: OutputType) {
        self.output_type = Some(output_type);
    }

    /// Set whether to have `\tags` rendered on the left instead of the right.
    pub fn set_leqno(&mut self, flag: bool) {
        self.leqno = Some(flag);
    }

    /// Set whether to make display math flush left.
    pub fn set_fleqn(&mut self, flag: bool) {
        self.fleqn = Some(flag);
    }

    /// Set whether to let KaTeX throw a ParseError for invalid LaTeX.
    pub fn set_throw_on_error(&mut self, flag: bool) {
        self.throw_on_error = Some(flag);
    }

    /// Set the color used for invalid LaTeX.
    pub fn set_error_color(&mut self, color: String) {
        self.error_color = Some(color);
    }

    /// Add a custom macro.
    /// Read <https://katex.org/docs/options.html> for more information.
    pub fn add_macro(&mut self, entry_name: String, entry_data: String) {
        self.macros.insert(entry_name, entry_data);
    }

    /// Set the minimum thickness, in ems.
    /// Read <https://katex.org/docs/options.html> for more information.
    pub fn set_min_rule_thickness(&mut self, value: f64) {
        self.min_rule_thickness = Some(value);
    }

    /// Set the max size for user-specified sizes.
    /// If set to `None`, users can make elements and spaces arbitrarily large.
    /// Read <https://katex.org/docs/options.html> for more information.
    pub fn set_max_size(&mut self, value: Option<f64>) {
        self.max_size = Some(value);
    }

    /// Set the limit for the number of macro expansions.
    /// If set to `None`, the macro expander will try to fully expand as in LaTeX.
    /// Read <https://katex.org/docs/options.html> for more information.
    pub fn set_max_expand(&mut self, value: Option<i32>) {
        self.max_expand = Some(value);
    }

    /// Set whether to trust users' input.
    /// Cannot be used at the same time with [`set_trust_callback`].
    /// Read <https://katex.org/docs/options.html> for more information.
    ///
    /// # Panic
    ///
    /// Panic if `trust_callback` is also set.
    pub fn set_trust(&mut self, flag: bool) {
        if self.trust_callback.is_some() {
            panic!("Cannot set `trust` and `trust_callback` at the same time");
        }
        self.trust = Some(flag);
    }

    /// Set the callback function to determine whether to trust users' input.
    /// Cannot be used at the same time with [`set_trust`].
    /// Read <https://katex.org/docs/options.html> for more information.
    ///
    /// # Panic
    ///
    /// Panic if `trust` is also set.
    pub fn set_trust_callback(&mut self, callback: TrustCallback) {
        if self.trust.is_some() {
            panic!("Cannot set `trust` and `trust_callback` at the same time");
        }
        self.trust_callback = Some(callback);
    }
}

impl Into<JsValue> for Opts {
    fn into(self) -> JsValue {
        let mut opt: HashMap<String, JsValue> = HashMap::new();
        if let Some(display_mode) = self.display_mode {
            opt.insert("displayMode".to_owned(), display_mode.into());
        }
        if let Some(output_type) = self.output_type {
            opt.insert(
                "output".to_owned(),
                match output_type {
                    OutputType::Html => "html",
                    OutputType::Mathml => "mathml",
                    OutputType::HtmlAndMathml => "htmlAndMathml",
                }
                .into(),
            );
        }
        if let Some(leqno) = self.leqno {
            opt.insert("leqno".to_owned(), leqno.into());
        }
        if let Some(fleqn) = self.fleqn {
            opt.insert("fleqn".to_owned(), fleqn.into());
        }
        if let Some(throw_on_error) = self.throw_on_error {
            opt.insert("throwOnError".to_owned(), throw_on_error.into());
        }
        if let Some(error_color) = self.error_color {
            opt.insert("errorColor".to_owned(), error_color.into());
        }
        opt.insert("macros".to_owned(), self.macros.into());
        if let Some(min_rule_thickness) = self.min_rule_thickness {
            opt.insert("minRuleThickness".to_owned(), min_rule_thickness.into());
        }
        if let Some(max_size) = self.max_size {
            if let Some(max_size) = max_size {
                opt.insert("maxSize".to_owned(), max_size.into());
            }
        }
        if let Some(max_expand) = self.max_expand {
            match max_expand {
                Some(max_expand) => {
                    opt.insert("maxExpand".to_owned(), max_expand.into());
                }
                None => {
                    opt.insert("maxExpand".to_owned(), i32::max_value().into());
                }
            }
        }
        if let Some(trust) = self.trust {
            opt.insert("trust".to_owned(), trust.into());
        }
        if self.trust_callback.is_some() {
            opt.insert("trust".to_owned(), "USE_TRUST_CALLBACK".into());
        }
        JsValue::Object(opt)
    }
}

impl OptsBuilder {
    /// Add an entry to [`macros`](OptsBuilder::macros).
    ///
    /// # Examples
    ///
    /// ```
    /// let opts = katex::Opts::builder()
    ///     .add_macro(r#"\RR"#.to_owned(), r#"\mathbb{R}"#.to_owned())
    ///     .build()
    ///     .unwrap();
    /// let html = katex::render_with_opts(r#"\RR"#, opts).unwrap();
    /// ```
    pub fn add_macro(mut self, entry_name: String, entry_data: String) -> Self {
        match self.macros.as_mut() {
            Some(macros) => {
                macros.insert(entry_name, entry_data);
            }
            None => {
                let mut macros = HashMap::new();
                macros.insert(entry_name, entry_data);
                self.macros = Some(macros);
            }
        }
        self
    }

    /// Check that `Opts` is valid.
    fn validate(&self) -> core::result::Result<(), String> {
        if self.trust.is_some() && self.trust_callback.is_some() {
            return Err("cannot set `trust` and `trust_callback` at the same time".to_owned());
        }

        Ok(())
    }
}

/// Output type from KaTeX.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum OutputType {
    /// Outputs KaTeX in HTML only.
    Html,
    /// Outputs KaTeX in MathML only.
    Mathml,
    /// Outputs HTML for visual rendering and includes MathML for accessibility.
    HtmlAndMathml,
}

/// Render LaTeX equation to HTML with additional [options](`Opts`).
pub fn render_with_opts(input: &str, opts: Opts) -> Result<String> {
    KATEX.with(|ctx| {
        let ctx = match ctx.as_ref() {
            Ok(ctx) => ctx,
            Err(e) => return Err(e.clone()),
        };
        if let Some(trust_callback) = opts.trust_callback.clone() {
            ctx.add_callback("trustCallback", trust_callback)?;
        }
        let args: Vec<JsValue> = vec![input.into(), opts.into()];
        let result = ctx
            .call_function("renderToString", args)?
            .into_string()
            .ok_or_else(|| quick_js::ValueError::UnexpectedType)?;
        Ok(result)
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
    fn test_display_mode() {
        let opts = Opts::builder().display_mode(true).build().unwrap();
        let html = render_with_opts("a = b + c", opts).unwrap();
        assert!(html.contains(r#"span class="katex-display""#));
    }

    #[test]
    fn test_output_html_only() {
        let opts = Opts::builder()
            .output_type(OutputType::Html)
            .build()
            .unwrap();
        let html = render_with_opts("a = b + c", opts).unwrap();
        assert!(!html.contains(r#"span class="katex-mathml""#));
        assert!(html.contains(r#"span class="katex-html""#));
    }

    #[test]
    fn test_output_mathml_only() {
        let opts = Opts::builder()
            .output_type(OutputType::Mathml)
            .build()
            .unwrap();
        let html = render_with_opts("a = b + c", opts).unwrap();
        assert!(html.contains(r#"MathML"#));
        assert!(!html.contains(r#"span class="katex-html""#));
    }

    #[test]
    fn test_leqno() {
        let opts = Opts::builder()
            .display_mode(true)
            .leqno(true)
            .build()
            .unwrap();
        let html = render_with_opts("a = b + c", opts).unwrap();
        assert!(html.contains(r#"span class="katex-display leqno""#));
    }

    #[test]
    fn test_fleqn() {
        let opts = Opts::builder()
            .display_mode(true)
            .fleqn(true)
            .build()
            .unwrap();
        let html = render_with_opts("a = b + c", opts).unwrap();
        assert!(html.contains(r#"span class="katex-display fleqn""#));
    }

    #[test]
    fn test_throw_on_error() {
        let err_msg = match render(r#"\"#) {
            Ok(_) => unreachable!(),
            Err(e) => match e {
                Error::JsExecError(msg) => msg,
                _ => unreachable!(),
            },
        };
        assert!(err_msg.contains("ParseError"));
    }

    #[test]
    fn test_error_color() {
        let opts = Opts::builder()
            .throw_on_error(false)
            .error_color("#ff0000")
            .build()
            .unwrap();
        let html = render_with_opts(r#"\"#, opts).unwrap();
        assert!(html.contains(r#"span class="katex-error""#));
        assert!(html.contains("color:#ff0000"));
    }

    #[test]
    fn test_macros() {
        let opts = Opts::builder()
            .add_macro(r#"\RR"#.to_owned(), r#"\mathbb{R}"#.to_owned())
            .build()
            .unwrap();
        let html = render_with_opts(r#"\RR"#, opts).unwrap();
        assert!(html.contains("mathbb"));
    }

    #[test]
    fn test_trust() {
        let opts = Opts::builder().error_color("#ff0000").build().unwrap();
        let html = render_with_opts(r#"\url{https://www.google.com}"#, opts).unwrap();
        assert!(html.contains(r#"color:#ff0000"#));
        assert!(!html.contains(r#"a href="https://www.google.com""#));

        let opts = Opts::builder()
            .error_color("#ff0000")
            .trust(true)
            .build()
            .unwrap();
        let html = render_with_opts(r#"\url{https://www.google.com}"#, opts).unwrap();
        assert!(!html.contains(r#"color:#ff0000"#));
        assert!(html.contains(r#"a href="https://www.google.com""#));
    }

    #[test]
    fn test_set_both_trust_and_trust_callback() {
        let opts = Opts::builder()
            .trust(true)
            .trust_callback(|_ctx: TrustContext| -> bool { true })
            .build();
        assert!(opts.is_err());
        assert_eq!(
            opts.unwrap_err(),
            "cannot set `trust` and `trust_callback` at the same time"
        );
    }

    #[test]
    fn test_trust_callback_using_closure() {
        let opts = Opts::builder()
            .error_color("#ff0000")
            .trust_callback(|ctx: TrustContext| -> bool {
                ctx.command == r#"\url"#
                    && ctx.protocol == "https"
                    && ctx.url == "https://www.google.com"
            })
            .build()
            .unwrap();
        let html = render_with_opts(r#"\url{https://www.google.com}"#, opts).unwrap();
        assert!(!html.contains(r#"color:#ff0000"#));
        assert!(html.contains(r#"a href="https://www.google.com""#));
    }

    #[test]
    fn test_trust_callback_using_fn() {
        fn callback(ctx: TrustContext) -> bool {
            ctx.command == r#"\url"#
                && ctx.protocol == "https"
                && ctx.url == "https://www.google.com"
        }
        let opts = Opts::builder()
            .error_color("#ff0000")
            .trust_callback(callback)
            .build()
            .unwrap();
        let html = render_with_opts(r#"\url{https://www.google.com}"#, opts).unwrap();
        assert!(!html.contains(r#"color:#ff0000"#));
        assert!(html.contains(r#"a href="https://www.google.com""#));
    }

    #[test]
    fn test_stack_overflow() {
        #[inline(never)]
        fn simulate_deep_stack(i: i32) {
            if i > 0 {
                simulate_deep_stack(i - 1);
            } else {
                let html = render("a = b + c").unwrap();
                assert!(html.contains(r#"span class="katex""#));
            }
        }
        simulate_deep_stack(100);
        simulate_deep_stack(0);
    }
}
