//! JS Engine implemented by [QuickJs](https://crates.io/crates/quick-js).

use core::convert::TryInto;

use crate::{
    error::{Error, Result},
    js_engine::{JsEngine, JsValue},
};

/// QuickJS Engine.
pub struct Engine(quick_js::Context);

impl JsEngine for Engine {
    type JsValue = Value;

    fn new() -> Result<Self> {
        Ok(Self(quick_js::Context::new()?))
    }

    fn eval(&mut self, code: &str) -> Result<Self::JsValue> {
        Ok(Value(self.0.eval(code)?))
    }

    fn call_function(
        &mut self,
        func_name: &str,
        args: impl Iterator<Item = Self::JsValue>,
    ) -> Result<Self::JsValue> {
        Ok(Value(self.0.call_function(func_name, args.map(|v| v.0))?))
    }
}

/// QuickJS Value.
#[derive(Debug, Clone)]
pub struct Value(quick_js::JsValue);

impl JsValue for Value {
    fn null() -> Self {
        Self(quick_js::JsValue::Null)
    }

    fn from_bool(input: bool) -> Self {
        Self(quick_js::JsValue::Bool(input))
    }

    fn from_int(input: i32) -> Self {
        Self(quick_js::JsValue::Int(input))
    }

    fn from_float(input: f64) -> Self {
        Self(quick_js::JsValue::Float(input))
    }

    fn from_string(input: String) -> Self {
        Self(quick_js::JsValue::String(input))
    }

    fn from_array(input: impl Iterator<Item = Self>) -> Self {
        let array = input.into_iter().map(|v| v.0).collect();
        Self(quick_js::JsValue::Array(array))
    }

    fn from_object(input: impl Iterator<Item = (String, Self)>) -> Self {
        let obj = input.into_iter().map(|(k, v)| (k, v.0)).collect();
        Self(quick_js::JsValue::Object(obj))
    }

    fn is_null(&self) -> bool {
        matches!(self.0, quick_js::JsValue::Null)
    }

    fn is_bool(&self) -> bool {
        matches!(self.0, quick_js::JsValue::Bool(_))
    }

    fn is_int(&self) -> bool {
        matches!(self.0, quick_js::JsValue::Int(_))
    }

    fn is_float(&self) -> bool {
        matches!(self.0, quick_js::JsValue::Float(_))
    }

    fn is_string(&self) -> bool {
        matches!(self.0, quick_js::JsValue::String(_))
    }

    fn into_bool(self) -> Result<bool> {
        Ok(self.0.try_into()?)
    }

    fn into_int(self) -> Result<i32> {
        Ok(self.0.try_into()?)
    }

    fn into_float(self) -> Result<f64> {
        Ok(self.0.try_into()?)
    }

    fn into_string(self) -> Result<String> {
        Ok(self.0.try_into()?)
    }
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
