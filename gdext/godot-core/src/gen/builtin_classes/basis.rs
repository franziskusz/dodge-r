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
pub struct InnerBasis < 'a > {
    _outer_lifetime: std::marker::PhantomData < &'a() >, sys_ptr: sys::GDExtensionTypePtr,
}
impl < 'a > InnerBasis < 'a > {
    pub fn from_outer(outer: &Basis) -> Self {
        Self {
            _outer_lifetime: std::marker::PhantomData, sys_ptr: outer.sys(),
        }
    }
    pub fn inverse(&self,) -> Basis {
        type RetMarshal = PtrcallReturnT < Basis >;
        type CallSig = (Basis,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(343usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "inverse", self.sys_ptr, args)
        }
    }
    pub fn transposed(&self,) -> Basis {
        type RetMarshal = PtrcallReturnT < Basis >;
        type CallSig = (Basis,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(344usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "transposed", self.sys_ptr, args)
        }
    }
    pub fn orthonormalized(&self,) -> Basis {
        type RetMarshal = PtrcallReturnT < Basis >;
        type CallSig = (Basis,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(345usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "orthonormalized", self.sys_ptr, args)
        }
    }
    pub fn determinant(&self,) -> f64 {
        type RetMarshal = PtrcallReturnT < f64 >;
        type CallSig = (f64,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(346usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "determinant", self.sys_ptr, args)
        }
    }
    pub fn rotated(&self, axis: Vector3, angle: f64,) -> Basis {
        type RetMarshal = PtrcallReturnT < Basis >;
        type CallSig = (Basis, Vector3, f64);
        let args = (axis, angle,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(347usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "rotated", self.sys_ptr, args)
        }
    }
    pub fn scaled(&self, scale: Vector3,) -> Basis {
        type RetMarshal = PtrcallReturnT < Basis >;
        type CallSig = (Basis, Vector3);
        let args = (scale,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(348usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "scaled", self.sys_ptr, args)
        }
    }
    pub fn get_scale(&self,) -> Vector3 {
        type RetMarshal = PtrcallReturnT < Vector3 >;
        type CallSig = (Vector3,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(349usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "get_scale", self.sys_ptr, args)
        }
    }
    pub fn get_euler(&self, order: i64,) -> Vector3 {
        type RetMarshal = PtrcallReturnT < Vector3 >;
        type CallSig = (Vector3, i64);
        let args = (order,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(350usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "get_euler", self.sys_ptr, args)
        }
    }
    pub fn tdotx(&self, with: Vector3,) -> f64 {
        type RetMarshal = PtrcallReturnT < f64 >;
        type CallSig = (f64, Vector3);
        let args = (with,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(351usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "tdotx", self.sys_ptr, args)
        }
    }
    pub fn tdoty(&self, with: Vector3,) -> f64 {
        type RetMarshal = PtrcallReturnT < f64 >;
        type CallSig = (f64, Vector3);
        let args = (with,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(352usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "tdoty", self.sys_ptr, args)
        }
    }
    pub fn tdotz(&self, with: Vector3,) -> f64 {
        type RetMarshal = PtrcallReturnT < f64 >;
        type CallSig = (f64, Vector3);
        let args = (with,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(353usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "tdotz", self.sys_ptr, args)
        }
    }
    pub fn slerp(&self, to: Basis, weight: f64,) -> Basis {
        type RetMarshal = PtrcallReturnT < Basis >;
        type CallSig = (Basis, Basis, f64);
        let args = (to, weight,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(354usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "slerp", self.sys_ptr, args)
        }
    }
    pub fn is_conformal(&self,) -> bool {
        type RetMarshal = PtrcallReturnT < bool >;
        type CallSig = (bool,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(355usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "is_conformal", self.sys_ptr, args)
        }
    }
    pub fn is_equal_approx(&self, b: Basis,) -> bool {
        type RetMarshal = PtrcallReturnT < bool >;
        type CallSig = (bool, Basis);
        let args = (b,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(356usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "is_equal_approx", self.sys_ptr, args)
        }
    }
    pub fn is_finite(&self,) -> bool {
        type RetMarshal = PtrcallReturnT < bool >;
        type CallSig = (bool,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(357usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "is_finite", self.sys_ptr, args)
        }
    }
    pub fn get_rotation_quaternion(&self,) -> Quaternion {
        type RetMarshal = PtrcallReturnT < Quaternion >;
        type CallSig = (Quaternion,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(358usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "get_rotation_quaternion", self.sys_ptr, args)
        }
    }
    pub fn looking_at(target: Vector3, up: Vector3, use_model_front: bool,) -> Basis {
        type RetMarshal = PtrcallReturnT < Basis >;
        type CallSig = (Basis, Vector3, Vector3, bool);
        let args = (target, up, use_model_front,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(359usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "looking_at", std::ptr::null_mut(), args)
        }
    }
    pub fn from_scale(scale: Vector3,) -> Basis {
        type RetMarshal = PtrcallReturnT < Basis >;
        type CallSig = (Basis, Vector3);
        let args = (scale,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(360usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "from_scale", std::ptr::null_mut(), args)
        }
    }
    pub fn from_euler(euler: Vector3, order: i64,) -> Basis {
        type RetMarshal = PtrcallReturnT < Basis >;
        type CallSig = (Basis, Vector3, i64);
        let args = (euler, order,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(361usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "from_euler", std::ptr::null_mut(), args)
        }
    }
}