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
#[doc = "[`ToGodot`] and [`FromGodot`] are implemented for `*mut AudioFrame` and `*const AudioFrame`."]
#[repr(C)]
pub struct AudioFrame {
    pub left: f32, pub right: f32,
}
impl GodotConvert for * mut AudioFrame {
    type Via = i64;
    
}
impl ToGodot for * mut AudioFrame {
    fn to_godot(&self) -> Self::Via {
        * self as i64
    }
}
impl FromGodot for * mut AudioFrame {
    fn try_from_godot(via: Self::Via) -> Result < Self, crate::builtin::meta::ConvertError > {
        Ok(via as Self)
    }
}
impl GodotConvert for * const AudioFrame {
    type Via = i64;
    
}
impl ToGodot for * const AudioFrame {
    fn to_godot(&self) -> Self::Via {
        * self as i64
    }
}
impl FromGodot for * const AudioFrame {
    fn try_from_godot(via: Self::Via) -> Result < Self, crate::builtin::meta::ConvertError > {
        Ok(via as Self)
    }
}