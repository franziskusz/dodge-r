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
#[doc = "[`ToGodot`] and [`FromGodot`] are implemented for `*mut ScriptLanguageExtensionProfilingInfo` and `*const ScriptLanguageExtensionProfilingInfo`."]
#[repr(C)]
pub struct ScriptLanguageExtensionProfilingInfo {
    pub signature: StringName, pub call_count: u64, pub total_time: u64, pub self_time: u64,
}
impl GodotConvert for * mut ScriptLanguageExtensionProfilingInfo {
    type Via = i64;
    
}
impl ToGodot for * mut ScriptLanguageExtensionProfilingInfo {
    fn to_godot(&self) -> Self::Via {
        * self as i64
    }
}
impl FromGodot for * mut ScriptLanguageExtensionProfilingInfo {
    fn try_from_godot(via: Self::Via) -> Result < Self, crate::builtin::meta::ConvertError > {
        Ok(via as Self)
    }
}
impl GodotConvert for * const ScriptLanguageExtensionProfilingInfo {
    type Via = i64;
    
}
impl ToGodot for * const ScriptLanguageExtensionProfilingInfo {
    fn to_godot(&self) -> Self::Via {
        * self as i64
    }
}
impl FromGodot for * const ScriptLanguageExtensionProfilingInfo {
    fn try_from_godot(via: Self::Via) -> Result < Self, crate::builtin::meta::ConvertError > {
        Ok(via as Self)
    }
}