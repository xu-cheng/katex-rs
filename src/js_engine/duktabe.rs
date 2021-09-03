//! JS Engine implemented by [ducc](https://crates.io/crates/ducc).
use std::{collections::HashMap, sync::Arc};

use ducc::ToValue;
use ouroboros::self_referencing;

use crate::{
    error::{Error, Result},
    js_engine::{JsEngine, JsValue},
};

/// Ducc Engine.
pub struct Engine(Arc<ducc::Ducc>);

impl JsEngine for Engine {
    type JsValue = Value;

    fn new() -> Result<Self> {
        Ok(Self(Arc::new(ducc::Ducc::new())))
    }

    fn eval(&mut self, code: &str) -> Result<Self::JsValue> {
        let value: ValueDucc = ValueDuccTryBuilder {
            engine: Arc::clone(&self.0),
            value_builder: |engine| {
                let settings = ducc::ExecSettings::default();
                engine.exec::<ducc::Value>(code, None, settings)
            },
        }
        .try_build()
        .map_err(|e| Error::JsExecError(e.to_string()))?;

        Ok(Value(ValueInner::Ducc(Arc::new(value))))
    }

    fn call_function(
        &mut self,
        func_name: &str,
        args: impl Iterator<Item = Self::JsValue>,
    ) -> Result<Self::JsValue> {
        let value: ValueDucc = ValueDuccTryBuilder {
            engine: Arc::clone(&self.0),
            value_builder: |engine| {
                let function = engine
                    .globals()
                    .get::<String, ducc::Function>(func_name.to_string())?;

                let mut args_vec = vec![];
                for arg in args {
                    args_vec.push(arg.to_value(engine)?);
                }
                let args = ducc::Values::from_vec(args_vec);

                function.call::<ducc::Values, ducc::Value>(args)
            },
        }
        .try_build()
        .map_err(|e| Error::JsExecError(e.to_string()))?;

        Ok(Value(ValueInner::Ducc(Arc::new(value))))
    }
}

#[self_referencing]
struct ValueDucc {
    engine: Arc<ducc::Ducc>,

    #[borrows(engine)]
    #[covariant]
    pub value: ducc::Value<'this>,
}

impl ValueDucc {
    fn value(&self) -> &ducc::Value<'_> {
        self.borrow_value()
    }

    fn to_value<'me, 'other>(
        &'me self,
        ducc: &'other ducc::Ducc,
    ) -> ducc::Result<ducc::Value<'other>> {
        let my_duc: &ducc::Ducc = self.borrow_engine().as_ref();
        if std::ptr::addr_of!(ducc) == std::ptr::addr_of!(my_duc) {
            // same engine => could re-use variable
            //
            // Something like this would work, but is actually not required for KaTex:
            //
            // ```
            // let value: ducc::Value<'me> = self.value().clone();
            // let value: ducc::Value<'other> = unsafe {
            //     std::mem::transmute<ducc::Value<'me>, ducc::Value<'other>>(value)
            // };
            // ```
            Err(ducc::ErrorKind::RuntimeError {
                code: ducc::RuntimeErrorCode::ReferenceError,
                name: "same engine, but still somewhat unsafe".to_string(),
            }
            .into())
        } else {
            // different engine => fail
            Err(ducc::ErrorKind::RuntimeError {
                code: ducc::RuntimeErrorCode::ReferenceError,
                name: "different engines".to_string(),
            }
            .into())
        }
    }
}

#[derive(Clone)]
enum ValueRust {
    Null,
    Bool(bool),
    Int(i32),
    Float(f64),
    String(String),
    Array(Vec<Value>),
    Object(HashMap<String, Value>),
}

#[derive(Clone)]
enum ValueInner {
    Ducc(Arc<ValueDucc>),
    Rust(ValueRust),
}

#[derive(Clone)]
pub struct Value(ValueInner);

impl JsValue for Value {
    fn null() -> Self {
        Self(ValueInner::Rust(ValueRust::Null))
    }

    fn from_bool(input: bool) -> Self {
        Self(ValueInner::Rust(ValueRust::Bool(input)))
    }

    fn from_int(input: i32) -> Self {
        Self(ValueInner::Rust(ValueRust::Int(input)))
    }

    fn from_float(input: f64) -> Self {
        Self(ValueInner::Rust(ValueRust::Float(input)))
    }

    fn from_string(input: String) -> Self {
        Self(ValueInner::Rust(ValueRust::String(input)))
    }

    fn from_array(input: impl Iterator<Item = Self>) -> Self {
        Self(ValueInner::Rust(ValueRust::Array(input.collect())))
    }

    fn from_object(input: impl Iterator<Item = (String, Self)>) -> Self {
        Self(ValueInner::Rust(ValueRust::Object(input.collect())))
    }

    fn is_null(&self) -> bool {
        match &self.0 {
            ValueInner::Ducc(value) => {
                matches!(value.value(), ducc::Value::Null)
            }
            ValueInner::Rust(ValueRust::Null) => true,
            _ => false,
        }
    }

    fn is_bool(&self) -> bool {
        match &self.0 {
            ValueInner::Ducc(value) => {
                matches!(value.value(), ducc::Value::Boolean(_))
            }
            ValueInner::Rust(ValueRust::Bool(_)) => true,
            _ => false,
        }
    }

    fn is_int(&self) -> bool {
        match &self.0 {
            ValueInner::Ducc(value) => {
                matches!(value.value(), ducc::Value::Number(_))
            }
            ValueInner::Rust(ValueRust::Int(_)) => true,
            _ => false,
        }
    }

    fn is_float(&self) -> bool {
        match &self.0 {
            ValueInner::Ducc(value) => {
                matches!(value.value(), ducc::Value::Number(_))
            }
            ValueInner::Rust(ValueRust::Float(_)) => true,
            _ => false,
        }
    }

    fn is_string(&self) -> bool {
        match &self.0 {
            ValueInner::Ducc(value) => {
                matches!(value.value(), ducc::Value::String(_))
            }
            ValueInner::Rust(ValueRust::String(_)) => true,
            _ => false,
        }
    }

    fn into_bool(self) -> Result<bool> {
        match self.0 {
            ValueInner::Ducc(value) => match value.value() {
                ducc::Value::Boolean(value) => Ok(*value),
                _ => Err(Error::JsValueError("not a bool".to_string())),
            },
            ValueInner::Rust(ValueRust::Bool(value)) => Ok(value),
            _ => Err(Error::JsValueError("not a bool".to_string())),
        }
    }

    fn into_int(self) -> Result<i32> {
        match self.0 {
            ValueInner::Ducc(value) => match value.value() {
                ducc::Value::Number(value) => Ok(*value as i32),
                _ => Err(Error::JsValueError("not an int".to_string())),
            },
            ValueInner::Rust(ValueRust::Int(value)) => Ok(value),
            _ => Err(Error::JsValueError("not an int".to_string())),
        }
    }

    fn into_float(self) -> Result<f64> {
        match self.0 {
            ValueInner::Ducc(value) => match value.value() {
                ducc::Value::Number(value) => Ok(*value),
                _ => Err(Error::JsValueError("not a float".to_string())),
            },
            ValueInner::Rust(ValueRust::Float(value)) => Ok(value),
            _ => Err(Error::JsValueError("not a float".to_string())),
        }
    }

    fn into_string(self) -> Result<String> {
        match self.0 {
            ValueInner::Ducc(value) => match value.value() {
                ducc::Value::String(value) => {
                    let value = String::from_utf8(value.as_bytes().to_vec()).map_err(|_| {
                        Error::JsValueError("cannot extract string from bytes".to_string())
                    })?;
                    Ok(value)
                }
                _ => Err(Error::JsValueError("not a string".to_string())),
            },
            ValueInner::Rust(ValueRust::String(value)) => Ok(value),
            _ => Err(Error::JsValueError("not a string".to_string())),
        }
    }
}

impl<'ducc> ducc::ToValue<'ducc> for ValueRust {
    fn to_value(self, ducc: &'ducc ducc::Ducc) -> ducc::Result<ducc::Value<'ducc>> {
        match self {
            ValueRust::Null => Ok(ducc::Value::Null),
            ValueRust::Bool(value) => Ok(ducc::Value::Boolean(value)),
            ValueRust::Int(value) => Ok(ducc::Value::Number(value as f64)),
            ValueRust::Float(value) => Ok(ducc::Value::Number(value)),
            ValueRust::String(value) => Ok(ducc::Value::String(ducc.create_string(&value)?)),
            ValueRust::Array(value) => {
                let array = ducc.create_array();

                for v in value {
                    let v: ducc::Value = v.to_value(ducc)?;
                    array.push(v)?;
                }

                Ok(ducc::Value::Array(array))
            }
            ValueRust::Object(value) => {
                let entries = value
                    .into_iter()
                    .map(|(k, v)| Ok((k, v.to_value(ducc)?)))
                    .collect::<Result<Vec<(String, ducc::Value<'ducc>)>, ducc::Error>>()?;

                Ok(ducc::Value::Object(
                    ducc.create_object_from(entries.into_iter())?,
                ))
            }
        }
    }
}

impl<'ducc> ducc::ToValue<'ducc> for Value {
    fn to_value(self, ducc: &'ducc ducc::Ducc) -> ducc::Result<ducc::Value<'ducc>> {
        match self.0 {
            ValueInner::Rust(value) => value.to_value(ducc),
            ValueInner::Ducc(value) => value.to_value(ducc),
        }
    }
}
