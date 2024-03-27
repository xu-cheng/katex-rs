//! JS Engine implemented by [quickjs_runtime](https://crates.io/crates/quickjs_runtime).

use std::collections::HashMap;

use quickjs_runtime::{
    builder::QuickJsRuntimeBuilder,
    facades::QuickJsRuntimeFacade,
    jsutils::Script,
    values::{JsValueConvertable, JsValueFacade},
};

use crate::{
    error::{Error, Result},
    js_engine::{JsEngine, JsValue},
};

/// quickjs_runtime Engine.
pub struct Engine(QuickJsRuntimeFacade);

impl JsEngine for Engine {
    type JsValue<'a> = Value;

    fn new() -> Result<Self> {
        Ok(Self(QuickJsRuntimeBuilder::new().build()))
    }

    fn eval<'a>(&'a self, code: &str) -> Result<Self::JsValue<'a>> {
        self.0
            .eval_sync(None, Script::new("katex", code))
            .map(Value)
            .map_err(|e| Error::JsExecError(format!("{e}")))
    }

    fn call_function<'a>(
        &'a self,
        func_name: &str,
        args: impl Iterator<Item = Self::JsValue<'a>>,
    ) -> Result<Self::JsValue<'a>> {
        self.0
            .invoke_function_sync(None, &[], func_name, args.map(|v| v.0).collect())
            .map(Value)
            .map_err(|e| Error::JsExecError(format!("{e}")))
    }

    fn create_bool_value(&self, input: bool) -> Result<Self::JsValue<'_>> {
        Ok(input.into())
    }

    fn create_int_value(&self, input: i32) -> Result<Self::JsValue<'_>> {
        Ok(input.into())
    }

    fn create_float_value(&self, input: f64) -> Result<Self::JsValue<'_>> {
        Ok(input.into())
    }

    fn create_string_value(&self, input: String) -> Result<Self::JsValue<'_>> {
        Ok(input.into())
    }

    fn create_object_value<'a>(
        &'a self,
        input: impl Iterator<Item = (String, Self::JsValue<'a>)>,
    ) -> Result<Self::JsValue<'a>> {
        Ok(input
            .map(|(k, v)| (k, v.0))
            .collect::<HashMap<_, _>>()
            .into())
    }
}

/// quickjs_runtime Value.
#[derive(Debug)]
pub struct Value(JsValueFacade);

impl<'a> JsValue<'a> for Value {
    fn into_string(self) -> Result<String> {
        Ok(self.0.get_str().to_string())
    }
}

impl<T> From<T> for Value
where
    T: JsValueConvertable,
{
    fn from(value: T) -> Self {
        Self(value.to_js_value_facade())
    }
}
