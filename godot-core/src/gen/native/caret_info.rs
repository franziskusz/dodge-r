use godot_ffi as sys;
use crate::builtin::*;
use crate::builtin::meta::{
    ClassName, PtrcallReturnUnit, PtrcallReturnT, PtrcallReturnOptionGdT, PtrcallSignatureTuple, VarcallSignatureTuple
};
use crate::engine::native::*;
use crate::engine::Object;
use crate::obj::Gd;
use crate::sys::GodotFfi as _;
use crate::builtin::meta::{
    GodotConvert, FromGodot, ToGodot
};
#[doc = r" Native structure; can be passed via pointer in APIs that are not exposed to GDScript."]
#[doc = r""]
#[doc = "[`ToGodot`] and [`FromGodot`] are implemented for `*mut CaretInfo` and `*const CaretInfo`."]
#[repr(C)]
pub struct CaretInfo {
    pub leading_caret: Rect2, pub trailing_caret: Rect2, pub leading_direction: crate::engine::text_server::Direction, pub trailing_direction: crate::engine::text_server::Direction,
}
impl GodotConvert for * mut CaretInfo {
    type Via = i64;
    
}
impl ToGodot for * mut CaretInfo {
    fn to_godot(&self) -> Self::Via {
        * self as i64
    }
}
impl FromGodot for * mut CaretInfo {
    fn try_from_godot(via: Self::Via) -> Result < Self, crate::builtin::meta::ConvertError > {
        Ok(via as Self)
    }
}
impl GodotConvert for * const CaretInfo {
    type Via = i64;
    
}
impl ToGodot for * const CaretInfo {
    fn to_godot(&self) -> Self::Via {
        * self as i64
    }
}
impl FromGodot for * const CaretInfo {
    fn try_from_godot(via: Self::Via) -> Result < Self, crate::builtin::meta::ConvertError > {
        Ok(via as Self)
    }
}