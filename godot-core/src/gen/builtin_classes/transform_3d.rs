use godot_ffi as sys;
use crate::builtin::*;
use crate::builtin::meta::{
    ClassName, PtrcallReturnUnit, PtrcallReturnT, PtrcallReturnOptionGdT, PtrcallSignatureTuple, VarcallSignatureTuple
};
use crate::engine::native::*;
use crate::engine::Object;
use crate::obj::Gd;
use crate::sys::GodotFfi as _;
#[repr(transparent)]
pub struct InnerTransform3D < 'a > {
    _outer_lifetime: std::marker::PhantomData < &'a() >, sys_ptr: sys::GDExtensionTypePtr,
}
impl < 'a > InnerTransform3D < 'a > {
    pub fn from_outer(outer: &Transform3D) -> Self {
        Self {
            _outer_lifetime: std::marker::PhantomData, sys_ptr: outer.sys(),
        }
    }
    pub fn inverse(&self,) -> Transform3D {
        type RetMarshal = PtrcallReturnT < Transform3D >;
        type CallSig = (Transform3D,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(362usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "inverse", self.sys_ptr, args)
        }
    }
    pub fn affine_inverse(&self,) -> Transform3D {
        type RetMarshal = PtrcallReturnT < Transform3D >;
        type CallSig = (Transform3D,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(363usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "affine_inverse", self.sys_ptr, args)
        }
    }
    pub fn orthonormalized(&self,) -> Transform3D {
        type RetMarshal = PtrcallReturnT < Transform3D >;
        type CallSig = (Transform3D,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(364usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "orthonormalized", self.sys_ptr, args)
        }
    }
    pub fn rotated(&self, axis: Vector3, angle: f64,) -> Transform3D {
        type RetMarshal = PtrcallReturnT < Transform3D >;
        type CallSig = (Transform3D, Vector3, f64);
        let args = (axis, angle,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(365usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "rotated", self.sys_ptr, args)
        }
    }
    pub fn rotated_local(&self, axis: Vector3, angle: f64,) -> Transform3D {
        type RetMarshal = PtrcallReturnT < Transform3D >;
        type CallSig = (Transform3D, Vector3, f64);
        let args = (axis, angle,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(366usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "rotated_local", self.sys_ptr, args)
        }
    }
    pub fn scaled(&self, scale: Vector3,) -> Transform3D {
        type RetMarshal = PtrcallReturnT < Transform3D >;
        type CallSig = (Transform3D, Vector3);
        let args = (scale,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(367usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "scaled", self.sys_ptr, args)
        }
    }
    pub fn scaled_local(&self, scale: Vector3,) -> Transform3D {
        type RetMarshal = PtrcallReturnT < Transform3D >;
        type CallSig = (Transform3D, Vector3);
        let args = (scale,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(368usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "scaled_local", self.sys_ptr, args)
        }
    }
    pub fn translated(&self, offset: Vector3,) -> Transform3D {
        type RetMarshal = PtrcallReturnT < Transform3D >;
        type CallSig = (Transform3D, Vector3);
        let args = (offset,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(369usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "translated", self.sys_ptr, args)
        }
    }
    pub fn translated_local(&self, offset: Vector3,) -> Transform3D {
        type RetMarshal = PtrcallReturnT < Transform3D >;
        type CallSig = (Transform3D, Vector3);
        let args = (offset,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(370usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "translated_local", self.sys_ptr, args)
        }
    }
    pub fn looking_at(&self, target: Vector3, up: Vector3, use_model_front: bool,) -> Transform3D {
        type RetMarshal = PtrcallReturnT < Transform3D >;
        type CallSig = (Transform3D, Vector3, Vector3, bool);
        let args = (target, up, use_model_front,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(371usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "looking_at", self.sys_ptr, args)
        }
    }
    pub fn interpolate_with(&self, xform: Transform3D, weight: f64,) -> Transform3D {
        type RetMarshal = PtrcallReturnT < Transform3D >;
        type CallSig = (Transform3D, Transform3D, f64);
        let args = (xform, weight,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(372usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "interpolate_with", self.sys_ptr, args)
        }
    }
    pub fn is_equal_approx(&self, xform: Transform3D,) -> bool {
        type RetMarshal = PtrcallReturnT < bool >;
        type CallSig = (bool, Transform3D);
        let args = (xform,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(373usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "is_equal_approx", self.sys_ptr, args)
        }
    }
    pub fn is_finite(&self,) -> bool {
        type RetMarshal = PtrcallReturnT < bool >;
        type CallSig = (bool,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(374usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "is_finite", self.sys_ptr, args)
        }
    }
}