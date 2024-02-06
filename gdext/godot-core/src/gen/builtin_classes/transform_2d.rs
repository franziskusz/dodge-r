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
pub struct InnerTransform2D < 'a > {
    _outer_lifetime: std::marker::PhantomData < &'a() >, sys_ptr: sys::GDExtensionTypePtr,
}
impl < 'a > InnerTransform2D < 'a > {
    pub fn from_outer(outer: &Transform2D) -> Self {
        Self {
            _outer_lifetime: std::marker::PhantomData, sys_ptr: outer.sys(),
        }
    }
    pub fn inverse(&self,) -> Transform2D {
        type RetMarshal = PtrcallReturnT < Transform2D >;
        type CallSig = (Transform2D,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(233usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "inverse", self.sys_ptr, args)
        }
    }
    pub fn affine_inverse(&self,) -> Transform2D {
        type RetMarshal = PtrcallReturnT < Transform2D >;
        type CallSig = (Transform2D,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(234usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "affine_inverse", self.sys_ptr, args)
        }
    }
    pub fn get_rotation(&self,) -> f64 {
        type RetMarshal = PtrcallReturnT < f64 >;
        type CallSig = (f64,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(235usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "get_rotation", self.sys_ptr, args)
        }
    }
    pub fn get_origin(&self,) -> Vector2 {
        type RetMarshal = PtrcallReturnT < Vector2 >;
        type CallSig = (Vector2,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(236usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "get_origin", self.sys_ptr, args)
        }
    }
    pub fn get_scale(&self,) -> Vector2 {
        type RetMarshal = PtrcallReturnT < Vector2 >;
        type CallSig = (Vector2,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(237usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "get_scale", self.sys_ptr, args)
        }
    }
    pub fn get_skew(&self,) -> f64 {
        type RetMarshal = PtrcallReturnT < f64 >;
        type CallSig = (f64,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(238usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "get_skew", self.sys_ptr, args)
        }
    }
    pub fn orthonormalized(&self,) -> Transform2D {
        type RetMarshal = PtrcallReturnT < Transform2D >;
        type CallSig = (Transform2D,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(239usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "orthonormalized", self.sys_ptr, args)
        }
    }
    pub fn rotated(&self, angle: f64,) -> Transform2D {
        type RetMarshal = PtrcallReturnT < Transform2D >;
        type CallSig = (Transform2D, f64);
        let args = (angle,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(240usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "rotated", self.sys_ptr, args)
        }
    }
    pub fn rotated_local(&self, angle: f64,) -> Transform2D {
        type RetMarshal = PtrcallReturnT < Transform2D >;
        type CallSig = (Transform2D, f64);
        let args = (angle,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(241usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "rotated_local", self.sys_ptr, args)
        }
    }
    pub fn scaled(&self, scale: Vector2,) -> Transform2D {
        type RetMarshal = PtrcallReturnT < Transform2D >;
        type CallSig = (Transform2D, Vector2);
        let args = (scale,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(242usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "scaled", self.sys_ptr, args)
        }
    }
    pub fn scaled_local(&self, scale: Vector2,) -> Transform2D {
        type RetMarshal = PtrcallReturnT < Transform2D >;
        type CallSig = (Transform2D, Vector2);
        let args = (scale,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(243usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "scaled_local", self.sys_ptr, args)
        }
    }
    pub fn translated(&self, offset: Vector2,) -> Transform2D {
        type RetMarshal = PtrcallReturnT < Transform2D >;
        type CallSig = (Transform2D, Vector2);
        let args = (offset,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(244usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "translated", self.sys_ptr, args)
        }
    }
    pub fn translated_local(&self, offset: Vector2,) -> Transform2D {
        type RetMarshal = PtrcallReturnT < Transform2D >;
        type CallSig = (Transform2D, Vector2);
        let args = (offset,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(245usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "translated_local", self.sys_ptr, args)
        }
    }
    pub fn determinant(&self,) -> f64 {
        type RetMarshal = PtrcallReturnT < f64 >;
        type CallSig = (f64,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(246usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "determinant", self.sys_ptr, args)
        }
    }
    pub fn basis_xform(&self, v: Vector2,) -> Vector2 {
        type RetMarshal = PtrcallReturnT < Vector2 >;
        type CallSig = (Vector2, Vector2);
        let args = (v,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(247usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "basis_xform", self.sys_ptr, args)
        }
    }
    pub fn basis_xform_inv(&self, v: Vector2,) -> Vector2 {
        type RetMarshal = PtrcallReturnT < Vector2 >;
        type CallSig = (Vector2, Vector2);
        let args = (v,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(248usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "basis_xform_inv", self.sys_ptr, args)
        }
    }
    pub fn interpolate_with(&self, xform: Transform2D, weight: f64,) -> Transform2D {
        type RetMarshal = PtrcallReturnT < Transform2D >;
        type CallSig = (Transform2D, Transform2D, f64);
        let args = (xform, weight,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(249usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "interpolate_with", self.sys_ptr, args)
        }
    }
    pub fn is_conformal(&self,) -> bool {
        type RetMarshal = PtrcallReturnT < bool >;
        type CallSig = (bool,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(250usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "is_conformal", self.sys_ptr, args)
        }
    }
    pub fn is_equal_approx(&self, xform: Transform2D,) -> bool {
        type RetMarshal = PtrcallReturnT < bool >;
        type CallSig = (bool, Transform2D);
        let args = (xform,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(251usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "is_equal_approx", self.sys_ptr, args)
        }
    }
    pub fn is_finite(&self,) -> bool {
        type RetMarshal = PtrcallReturnT < bool >;
        type CallSig = (bool,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(252usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "is_finite", self.sys_ptr, args)
        }
    }
    pub fn looking_at(&self, target: Vector2,) -> Transform2D {
        type RetMarshal = PtrcallReturnT < Transform2D >;
        type CallSig = (Transform2D, Vector2);
        let args = (target,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(253usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "looking_at", self.sys_ptr, args)
        }
    }
}