//! Abstraction of the JS Engine.

use crate::error::Result;

/// A trait to represent a JS engine.
pub trait JsEngine: Sized + private::Sealed {
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
pub trait JsValue: Sized + Clone + private::Sealed {
    /// Create a JS value `null`.
    fn null() -> Self;
    /// Create a JS value from [`bool`].
    fn from_bool(input: bool) -> Self;
    /// Create a JS value from [`i32`].
    fn from_int(input: i32) -> Self;
    /// Create a JS value from [`f64`].
    fn from_float(input: f64) -> Self;
    /// Create a JS value from [`String`].
    fn from_string(input: String) -> Self;
    /// Create a JS array value from an iterator for `Self`.
    fn from_array(input: impl Iterator<Item = Self>) -> Self;
    /// Create a JS object value from an iterator for `(String, Self)`.
    fn from_object(input: impl Iterator<Item = (String, Self)>) -> Self;

    /// Check whether the JS value is `null`.
    fn is_null(&self) -> bool;
    /// Check whether the JS value is a [`bool`].
    fn is_bool(&self) -> bool;
    /// Check whether the JS value is a [`i32`].
    fn is_int(&self) -> bool;
    /// Check whether the JS value is a [`f64`].
    fn is_float(&self) -> bool;
    /// Check whether the JS value is a [`String`].
    fn is_string(&self) -> bool;

    /// Convert the JS Value to a [`bool`].
    fn to_bool(self) -> Result<bool>;
    /// Convert the JS Value to a [`i32`].
    fn to_int(self) -> Result<i32>;
    /// Convert the JS Value to a [`f64`].
    fn to_float(self) -> Result<f64>;
    /// Convert the JS Value to a [`String`].
    fn to_string(self) -> Result<String>;
}

mod private;
pub(crate) mod quick_js;
