/// Used to seal the traits.
pub trait Sealed {}

impl Sealed for crate::js_engine::quickjs::Engine {}
impl Sealed for crate::js_engine::quickjs::Value {}
