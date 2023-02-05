//! JS Engine implemented by [wasm-bindgen](https://crates.io/crates/wasm-bindgen)
//! and [js-sys](https://crates.io/crates/js-sys).

use crate::{
    error::{Error, Result},
    js_engine::{JsEngine, JsValue},
};

/// Wasm JS Engine.
pub struct Engine;

impl JsEngine for Engine {
    type JsValue<'a> = Value;

    fn new() -> Result<Self> {
        Ok(Self)
    }

    fn eval<'a>(&'a self, code: &str) -> Result<Self::JsValue<'a>> {
        js_sys::eval(code)
            .map(Value)
            .map_err(|e| Error::JsExecError(format!("{e:?}")))
    }

    fn call_function<'a>(
        &'a self,
        func_name: &str,
        args: impl Iterator<Item = Self::JsValue<'a>>,
    ) -> Result<Self::JsValue<'a>> {
        let function: js_sys::Function = js_sys::Reflect::get(&js_sys::global(), &func_name.into())
            .map_err(|e| Error::JsExecError(format!("{e:?}")))?
            .into();

        let args: js_sys::Array = args.map(|v| v.0).collect();
        let result = function
            .apply(&wasm_bindgen::JsValue::NULL, &args)
            .map_err(|e| Error::JsExecError(format!("{e:?}")))?;
        Ok(Value(result))
    }

    fn create_bool_value(&self, input: bool) -> Result<Self::JsValue<'_>> {
        Ok(Value(input.into()))
    }

    fn create_int_value(&self, input: i32) -> Result<Self::JsValue<'_>> {
        Ok(Value(input.into()))
    }

    fn create_float_value(&self, input: f64) -> Result<Self::JsValue<'_>> {
        Ok(Value(input.into()))
    }

    fn create_string_value(&self, input: String) -> Result<Self::JsValue<'_>> {
        Ok(Value(input.into()))
    }

    fn create_object_value<'a>(
        &'a self,
        input: impl Iterator<Item = (String, Self::JsValue<'a>)>,
    ) -> Result<Self::JsValue<'a>> {
        let obj = js_sys::Object::new();
        for (k, v) in input {
            js_sys::Reflect::set(&obj, &k.into(), &v.0)
                .map_err(|e| Error::JsValueError(format!("{e:?}")))?;
        }
        Ok(Value(obj.into()))
    }
}

/// Wasm JS Value.
#[derive(Debug)]
pub struct Value(wasm_bindgen::JsValue);

impl<'a> JsValue<'a> for Value {
    fn into_string(self) -> Result<String> {
        self.0
            .as_string()
            .ok_or_else(|| Error::JsValueError("cannot convert value to string".to_owned()))
    }
}
