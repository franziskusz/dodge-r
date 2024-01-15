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
#[doc = "[`ToGodot`] and [`FromGodot`] are implemented for `*mut PhysicsServer3DExtensionRayResult` and `*const PhysicsServer3DExtensionRayResult`."]
#[repr(C)]
pub struct PhysicsServer3DExtensionRayResult {
    pub position: Vector3, pub normal: Vector3, pub rid: Rid, pub collider_id: ObjectId, pub collider: * mut Gd < crate::engine::Object >, pub shape: i32, pub face_index: i32,
}
impl GodotConvert for * mut PhysicsServer3DExtensionRayResult {
    type Via = i64;
    
}
impl ToGodot for * mut PhysicsServer3DExtensionRayResult {
    fn to_godot(&self) -> Self::Via {
        * self as i64
    }
}
impl FromGodot for * mut PhysicsServer3DExtensionRayResult {
    fn try_from_godot(via: Self::Via) -> Result < Self, crate::builtin::meta::ConvertError > {
        Ok(via as Self)
    }
}
impl GodotConvert for * const PhysicsServer3DExtensionRayResult {
    type Via = i64;
    
}
impl ToGodot for * const PhysicsServer3DExtensionRayResult {
    fn to_godot(&self) -> Self::Via {
        * self as i64
    }
}
impl FromGodot for * const PhysicsServer3DExtensionRayResult {
    fn try_from_godot(via: Self::Via) -> Result < Self, crate::builtin::meta::ConvertError > {
        Ok(via as Self)
    }
}