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
#[doc = "[`ToGodot`] and [`FromGodot`] are implemented for `*mut PhysicsServer2DExtensionRayResult` and `*const PhysicsServer2DExtensionRayResult`."]
#[repr(C)]
pub struct PhysicsServer2DExtensionRayResult {
    pub position: Vector2, pub normal: Vector2, pub rid: Rid, pub collider_id: ObjectId, pub collider: * mut Gd < crate::engine::Object >, pub shape: i32,
}
impl GodotConvert for * mut PhysicsServer2DExtensionRayResult {
    type Via = i64;
    
}
impl ToGodot for * mut PhysicsServer2DExtensionRayResult {
    fn to_godot(&self) -> Self::Via {
        * self as i64
    }
}
impl FromGodot for * mut PhysicsServer2DExtensionRayResult {
    fn try_from_godot(via: Self::Via) -> Result < Self, crate::builtin::meta::ConvertError > {
        Ok(via as Self)
    }
}
impl GodotConvert for * const PhysicsServer2DExtensionRayResult {
    type Via = i64;
    
}
impl ToGodot for * const PhysicsServer2DExtensionRayResult {
    fn to_godot(&self) -> Self::Via {
        * self as i64
    }
}
impl FromGodot for * const PhysicsServer2DExtensionRayResult {
    fn try_from_godot(via: Self::Via) -> Result < Self, crate::builtin::meta::ConvertError > {
        Ok(via as Self)
    }
}