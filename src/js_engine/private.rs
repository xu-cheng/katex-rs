//! Use to seal the traits.

/// Mark trait is sealed.
pub trait Sealed {}

impl Sealed for crate::js_engine::quickjs::Engine {}
impl Sealed for crate::js_engine::quickjs::Value {}
