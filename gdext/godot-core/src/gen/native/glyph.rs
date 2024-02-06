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
#[doc = "[`ToGodot`] and [`FromGodot`] are implemented for `*mut Glyph` and `*const Glyph`."]
#[repr(C)]
pub struct Glyph {
    pub start: i32, pub end: i32, pub count: u8, pub repeat: u8, pub flags: u16, pub x_off: f32, pub y_off: f32, pub advance: f32, pub font_rid: Rid, pub font_size: i32, pub index: i32,
}
impl GodotConvert for * mut Glyph {
    type Via = i64;
    
}
impl ToGodot for * mut Glyph {
    fn to_godot(&self) -> Self::Via {
        * self as i64
    }
}
impl FromGodot for * mut Glyph {
    fn try_from_godot(via: Self::Via) -> Result < Self, crate::builtin::meta::ConvertError > {
        Ok(via as Self)
    }
}
impl GodotConvert for * const Glyph {
    type Via = i64;
    
}
impl ToGodot for * const Glyph {
    fn to_godot(&self) -> Self::Via {
        * self as i64
    }
}
impl FromGodot for * const Glyph {
    fn try_from_godot(via: Self::Via) -> Result < Self, crate::builtin::meta::ConvertError > {
        Ok(via as Self)
    }
}