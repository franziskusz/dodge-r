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
#[doc = "[`ToGodot`] and [`FromGodot`] are implemented for `*mut PhysicsServer3DExtensionShapeRestInfo` and `*const PhysicsServer3DExtensionShapeRestInfo`."]
#[repr(C)]
pub struct PhysicsServer3DExtensionShapeRestInfo {
    pub point: Vector3, pub normal: Vector3, pub rid: Rid, pub collider_id: ObjectId, pub shape: i32, pub linear_velocity: Vector3,
}
impl GodotConvert for * mut PhysicsServer3DExtensionShapeRestInfo {
    type Via = i64;
    
}
impl ToGodot for * mut PhysicsServer3DExtensionShapeRestInfo {
    fn to_godot(&self) -> Self::Via {
        * self as i64
    }
}
impl FromGodot for * mut PhysicsServer3DExtensionShapeRestInfo {
    fn try_from_godot(via: Self::Via) -> Result < Self, crate::builtin::meta::ConvertError > {
        Ok(via as Self)
    }
}
impl GodotConvert for * const PhysicsServer3DExtensionShapeRestInfo {
    type Via = i64;
    
}
impl ToGodot for * const PhysicsServer3DExtensionShapeRestInfo {
    fn to_godot(&self) -> Self::Via {
        * self as i64
    }
}
impl FromGodot for * const PhysicsServer3DExtensionShapeRestInfo {
    fn try_from_godot(via: Self::Via) -> Result < Self, crate::builtin::meta::ConvertError > {
        Ok(via as Self)
    }
}