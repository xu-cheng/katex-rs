//! Use to seal the traits.

/// Mark trait is sealed.
pub trait Sealed {}

#[cfg(feature = "quick-js")]
mod quick_js {
    use super::Sealed;

    impl Sealed for crate::js_engine::quick_js::Engine {}
    impl Sealed for crate::js_engine::quick_js::Value {}
}
