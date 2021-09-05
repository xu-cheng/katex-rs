//! Abstraction of the JS Engine.

use crate::error::Result;

/// A trait to represent a JS engine.
pub(crate) trait JsEngine: Sized {
    /// The type of a JS value.
    type JsValue: JsValue;

    /// Create a JS engine.
    fn new() -> Result<Self>;

    /// Evaluate arbitrary code in the JS engine.
    fn eval(&mut self, code: &str) -> Result<Self::JsValue>;

    /// Call a JS function in the JS engine.
    fn call_function(
        &mut self,
        func_name: &str,
        args: impl Iterator<Item = Self::JsValue>,
    ) -> Result<Self::JsValue>;
}

/// A trait to represent a JS value.
pub(crate) trait JsValue: Sized + Clone {
    /// Create a JS value from [`bool`].
    fn from_bool(input: bool) -> Self;
    /// Create a JS value from [`i32`].
    fn from_int(input: i32) -> Self;
    /// Create a JS value from [`f64`].
    fn from_float(input: f64) -> Self;
    /// Create a JS value from [`String`].
    fn from_string(input: String) -> Self;
    /// Create a JS object value from an iterator for `(String, Self)`.
    fn from_object(input: impl Iterator<Item = (String, Self)>) -> Self;

    /// Convert the JS Value to a [`String`].
    fn into_string(self) -> Result<String>;
}

cfg_if::cfg_if! {
    if #[cfg(feature = "quick-js")] {
        mod quick_js;

        pub(crate) type Engine = quick_js::Engine;
    } else {
        compile_error!("Must enable one of the JS engines.");
    }
}
