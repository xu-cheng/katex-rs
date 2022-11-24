//! Abstraction of the JS Engine.

use crate::error::Result;
use cfg_if::cfg_if;

/// A trait to represent a JS engine.
pub(crate) trait JsEngine: Sized {
    /// Create a JS engine.
    fn new() -> Result<Self>;
}

/// A trait to represent a JS scope.
pub(crate) trait JsScope<'a>: Sized {
    /// The type of the JS engine.
    type JsEngine: JsEngine;

    /// The type of the JS value.
    type JsValue: JsValue;

    /// Get the global scope from the JS engine.
    fn global_scope(engine: &'a mut Self::JsEngine) -> Self;

    /// Evaluate arbitrary code in the JS engine.
    fn eval(&'a self, code: &str) -> Result<Self::JsValue>;

    /// Call a JS function in the JS engine.
    fn call_function(
        &'a self,
        func_name: &str,
        args: impl Iterator<Item = Self::JsValue>,
    ) -> Result<Self::JsValue>;

    /// Create a JS value from [`bool`].
    fn create_bool_value(&'a self, input: bool) -> Result<Self::JsValue>;

    /// Create a JS value from [`i32`].
    fn create_int_value(&'a self, input: i32) -> Result<Self::JsValue>;

    /// Create a JS value from [`f64`].
    fn create_float_value(&'a self, input: f64) -> Result<Self::JsValue>;

    /// Create a JS value from [`String`].
    fn create_string_value(&'a self, input: String) -> Result<Self::JsValue>;

    /// Create a JS object value from an iterator for `(String, Self::JsValue)`.
    fn create_object_value(
        &'a self,
        input: impl Iterator<Item = (String, Self::JsValue)>,
    ) -> Result<Self::JsValue>;
}

/// A trait to represent a JS value.
pub(crate) trait JsValue: Sized {
    /// Convert the JS Value to a [`String`].
    fn into_string(self) -> Result<String>;
}

cfg_if! {
    if #[cfg(feature = "quick-js")] {
        cfg_if! {
            if #[cfg(any(unix, all(windows, target_env = "gnu")))] {
                mod quick_js;

                pub(crate) type Engine = self::quick_js::Engine;
                pub(crate) type Scope<'a> = self::quick_js::Scope<'a>;
            } else {
                compile_error!("quick-js backend is not support in the current build target.");
            }
        }
    } else if #[cfg(feature = "duktape")] {
        cfg_if! {
            if #[cfg(any(unix, windows))] {
                mod duktape;

                pub(crate) type Engine = self::duktape::Engine;
                pub(crate) type Scope<'a> = self::duktape::Scope<'a>;
            } else {
                compile_error!("duktape backend is not support in the current build target.");
            }
        }
    } else if #[cfg(feature = "wasm-js")] {
        cfg_if! {
            if #[cfg(all(target_arch = "wasm32", target_os = "unknown"))] {
                mod wasm_js;

                pub(crate) type Engine = self::wasm_js::Engine;
                pub(crate) type Scope<'a> = self::wasm_js::Scope<'a>;
            } else {
                compile_error!("wasm-js backend is not support in the current build target.");
            }
        }
    } else {
        compile_error!("Must enable one of the JS engines.");
    }
}
