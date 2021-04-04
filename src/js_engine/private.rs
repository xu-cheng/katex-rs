//! Use to seal the traits.

/// Mark trait is sealed.
pub trait Sealed {}

impl Sealed for crate::js_engine::quick_js::Engine {}
impl Sealed for crate::js_engine::quick_js::Value {}
