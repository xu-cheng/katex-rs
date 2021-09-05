//! JS Engine implemented by [QuickJs](https://crates.io/crates/quick-js).

use core::convert::TryInto;

use crate::{
    error::{Error, Result},
    js_engine::{JsEngine, JsScope, JsValue},
};

/// QuickJS Engine.
pub struct Engine(quick_js::Context);

impl JsEngine for Engine {
    fn new() -> Result<Self> {
        Ok(Self(quick_js::Context::new()?))
    }
}

/// QuickJS Scope.
pub struct Scope<'a>(&'a quick_js::Context);

impl<'a> JsScope<'a> for Scope<'a> {
    type JsEngine = Engine;
    type JsValue = Value;

    fn global_scope(engine: &'a mut Self::JsEngine) -> Self {
        Self(&engine.0)
    }

    fn eval(&'a self, code: &str) -> Result<Self::JsValue> {
        Ok(Value(self.0.eval(code)?))
    }

    fn call_function(
        &'a self,
        func_name: &str,
        args: impl Iterator<Item = Self::JsValue>,
    ) -> Result<Self::JsValue> {
        Ok(Value(self.0.call_function(func_name, args.map(|v| v.0))?))
    }

    fn create_bool_value(&'a self, input: bool) -> Result<Self::JsValue> {
        Ok(Value(quick_js::JsValue::Bool(input)))
    }

    fn create_int_value(&'a self, input: i32) -> Result<Self::JsValue> {
        Ok(Value(quick_js::JsValue::Int(input)))
    }

    fn create_float_value(&'a self, input: f64) -> Result<Self::JsValue> {
        Ok(Value(quick_js::JsValue::Float(input)))
    }

    fn create_string_value(&'a self, input: String) -> Result<Self::JsValue> {
        Ok(Value(quick_js::JsValue::String(input)))
    }

    fn create_object_value(
        &'a self,
        input: impl Iterator<Item = (String, Self::JsValue)>,
    ) -> Result<Self::JsValue> {
        let obj = input.into_iter().map(|(k, v)| (k, v.0)).collect();
        Ok(Value(quick_js::JsValue::Object(obj)))
    }
}

/// QuickJS Value.
#[derive(Debug)]
pub struct Value(quick_js::JsValue);

impl JsValue for Value {
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
