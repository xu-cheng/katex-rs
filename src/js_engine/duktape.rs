//! JS Engine implemented by [Duktape](https://crates.io/crates/ducc).

use crate::{
    error::{Error, Result},
    js_engine::{JsEngine, JsValue},
};
use core::fmt;
use ducc::{FromValue, ToValue};

/// Duktape Engine.
pub struct Engine(ducc::Ducc);

impl JsEngine for Engine {
    type JsValue<'a> = Value<'a>;

    fn new() -> Result<Self> {
        Ok(Self(ducc::Ducc::new()))
    }

    fn eval<'a>(&'a self, code: &str) -> Result<Self::JsValue<'a>> {
        let result = self
            .0
            .exec(code, Some("katex"), ducc::ExecSettings::default())?;
        Ok(Value {
            value: result,
            engine: &self.0,
        })
    }

    fn call_function<'a>(
        &'a self,
        func_name: &str,
        args: impl Iterator<Item = Self::JsValue<'a>>,
    ) -> Result<Self::JsValue<'a>> {
        let function = self
            .0
            .globals()
            .get::<String, ducc::Function>(func_name.to_owned())?;
        let args: ducc::Values = args.map(|v| v.value).collect();
        let result = function.call(args)?;
        Ok(Value {
            value: result,
            engine: &self.0,
        })
    }

    fn create_bool_value(&self, input: bool) -> Result<Self::JsValue<'_>> {
        Ok(Value {
            value: input.to_value(&self.0)?,
            engine: &self.0,
        })
    }

    fn create_int_value(&self, input: i32) -> Result<Self::JsValue<'_>> {
        Ok(Value {
            value: input.to_value(&self.0)?,
            engine: &self.0,
        })
    }

    fn create_float_value(&self, input: f64) -> Result<Self::JsValue<'_>> {
        Ok(Value {
            value: input.to_value(&self.0)?,
            engine: &self.0,
        })
    }

    fn create_string_value(&self, input: String) -> Result<Self::JsValue<'_>> {
        Ok(Value {
            value: input.to_value(&self.0)?,
            engine: &self.0,
        })
    }

    fn create_object_value<'a>(
        &'a self,
        input: impl Iterator<Item = (String, Self::JsValue<'a>)>,
    ) -> Result<Self::JsValue<'a>> {
        let obj = self.0.create_object();
        for (k, v) in input {
            obj.set(k, v.value)?;
        }
        Ok(Value {
            value: ducc::Value::Object(obj),
            engine: &self.0,
        })
    }
}

/// Duktape Value.
pub struct Value<'a> {
    value: ducc::Value<'a>,
    engine: &'a ducc::Ducc,
}

impl<'a> JsValue<'a> for Value<'a> {
    fn into_string(self) -> Result<String> {
        Ok(String::from_value(self.value, self.engine)?)
    }
}

impl<'a> fmt::Debug for Value<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Value").field("value", &self.value).finish()
    }
}

impl From<ducc::Error> for Error {
    fn from(e: ducc::Error) -> Self {
        use ducc::ErrorKind;

        match e.kind {
            ErrorKind::ToJsConversionError { .. } | ErrorKind::FromJsConversionError { .. } => {
                Self::JsValueError(format!("{e}"))
            }
            _ => Self::JsExecError(format!("{e}")),
        }
    }
}
