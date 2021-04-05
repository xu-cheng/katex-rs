//! Use to seal the traits.

/// Mark trait is sealed.
pub trait Sealed {}

cfg_if::cfg_if! {
    if #[cfg(feature = "quick-js")] {
        impl Sealed for crate::js_engine::quick_js::Engine {}
        impl Sealed for crate::js_engine::quick_js::Value {}
    }
}
