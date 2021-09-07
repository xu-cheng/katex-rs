//! JS Engine implemented by [Duktape](https://crates.io/crates/ducc).

use crate::{
    error::{Error, Result},
    js_engine::{JsEngine, JsScope, JsValue},
};
use core::fmt;
use ducc::{FromValue, ToValue};

/// Duktape Engine.
pub struct Engine(ducc::Ducc);

impl JsEngine for Engine {
    fn new() -> Result<Self> {
        Ok(Self(ducc::Ducc::new()))
    }
}

/// Duktape Scope.
pub struct Scope<'a>(&'a mut ducc::Ducc);

impl<'a> JsScope<'a> for Scope<'a> {
    type JsEngine = Engine;
    type JsValue = Value<'a>;

    fn global_scope(engine: &'a mut Self::JsEngine) -> Self {
        Self(&mut engine.0)
    }

    fn eval(&'a self, code: &str) -> Result<Self::JsValue> {
        let result = self
            .0
            .exec(code, Some("katex"), ducc::ExecSettings::default())?;
        Ok(Value {
            value: result,
            scope: self,
        })
    }

    fn call_function(
        &'a self,
        func_name: &str,
        args: impl Iterator<Item = Self::JsValue>,
    ) -> Result<Self::JsValue> {
        let function = self
            .0
            .globals()
            .get::<String, ducc::Function>(func_name.to_owned())?;
        let args: ducc::Values = args.map(|v| v.value).collect();
        let result = function.call(args)?;
        Ok(Value {
            value: result,
            scope: self,
        })
    }

    fn create_bool_value(&'a self, input: bool) -> Result<Self::JsValue> {
        Ok(Value {
            value: input.to_value(self.0)?,
            scope: self,
        })
    }

    fn create_int_value(&'a self, input: i32) -> Result<Self::JsValue> {
        Ok(Value {
            value: input.to_value(self.0)?,
            scope: self,
        })
    }

    fn create_float_value(&'a self, input: f64) -> Result<Self::JsValue> {
        Ok(Value {
            value: input.to_value(self.0)?,
            scope: self,
        })
    }

    fn create_string_value(&'a self, input: String) -> Result<Self::JsValue> {
        Ok(Value {
            value: input.to_value(self.0)?,
            scope: self,
        })
    }

    fn create_object_value(
        &'a self,
        input: impl Iterator<Item = (String, Self::JsValue)>,
    ) -> Result<Self::JsValue> {
        let obj = self.0.create_object();
        for (k, v) in input {
            obj.set(k, v.value)?;
        }
        Ok(Value {
            value: ducc::Value::Object(obj),
            scope: self,
        })
    }
}

/// Duktape Value.
pub struct Value<'a> {
    value: ducc::Value<'a>,
    scope: &'a Scope<'a>,
}

impl<'a> JsValue for Value<'a> {
    fn into_string(self) -> Result<String> {
        Ok(String::from_value(self.value, self.scope.0)?)
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
                Self::JsValueError(format!("{}", e))
            }
            _ => Self::JsExecError(format!("{}", e)),
        }
    }
}
