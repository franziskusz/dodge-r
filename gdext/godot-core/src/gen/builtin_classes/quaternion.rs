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
pub struct InnerQuaternion < 'a > {
    _outer_lifetime: std::marker::PhantomData < &'a() >, sys_ptr: sys::GDExtensionTypePtr,
}
impl < 'a > InnerQuaternion < 'a > {
    pub fn from_outer(outer: &Quaternion) -> Self {
        Self {
            _outer_lifetime: std::marker::PhantomData, sys_ptr: outer.sys(),
        }
    }
    pub fn length(&self,) -> f64 {
        type RetMarshal = PtrcallReturnT < f64 >;
        type CallSig = (f64,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(299usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "length", self.sys_ptr, args)
        }
    }
    pub fn length_squared(&self,) -> f64 {
        type RetMarshal = PtrcallReturnT < f64 >;
        type CallSig = (f64,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(300usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "length_squared", self.sys_ptr, args)
        }
    }
    pub fn normalized(&self,) -> Quaternion {
        type RetMarshal = PtrcallReturnT < Quaternion >;
        type CallSig = (Quaternion,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(301usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "normalized", self.sys_ptr, args)
        }
    }
    pub fn is_normalized(&self,) -> bool {
        type RetMarshal = PtrcallReturnT < bool >;
        type CallSig = (bool,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(302usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "is_normalized", self.sys_ptr, args)
        }
    }
    pub fn is_equal_approx(&self, to: Quaternion,) -> bool {
        type RetMarshal = PtrcallReturnT < bool >;
        type CallSig = (bool, Quaternion);
        let args = (to,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(303usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "is_equal_approx", self.sys_ptr, args)
        }
    }
    pub fn is_finite(&self,) -> bool {
        type RetMarshal = PtrcallReturnT < bool >;
        type CallSig = (bool,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(304usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "is_finite", self.sys_ptr, args)
        }
    }
    pub fn inverse(&self,) -> Quaternion {
        type RetMarshal = PtrcallReturnT < Quaternion >;
        type CallSig = (Quaternion,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(305usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "inverse", self.sys_ptr, args)
        }
    }
    pub fn log(&self,) -> Quaternion {
        type RetMarshal = PtrcallReturnT < Quaternion >;
        type CallSig = (Quaternion,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(306usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "log", self.sys_ptr, args)
        }
    }
    pub fn exp(&self,) -> Quaternion {
        type RetMarshal = PtrcallReturnT < Quaternion >;
        type CallSig = (Quaternion,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(307usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "exp", self.sys_ptr, args)
        }
    }
    pub fn angle_to(&self, to: Quaternion,) -> f64 {
        type RetMarshal = PtrcallReturnT < f64 >;
        type CallSig = (f64, Quaternion);
        let args = (to,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(308usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "angle_to", self.sys_ptr, args)
        }
    }
    pub fn dot(&self, with: Quaternion,) -> f64 {
        type RetMarshal = PtrcallReturnT < f64 >;
        type CallSig = (f64, Quaternion);
        let args = (with,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(309usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "dot", self.sys_ptr, args)
        }
    }
    pub fn slerp(&self, to: Quaternion, weight: f64,) -> Quaternion {
        type RetMarshal = PtrcallReturnT < Quaternion >;
        type CallSig = (Quaternion, Quaternion, f64);
        let args = (to, weight,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(310usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "slerp", self.sys_ptr, args)
        }
    }
    pub fn slerpni(&self, to: Quaternion, weight: f64,) -> Quaternion {
        type RetMarshal = PtrcallReturnT < Quaternion >;
        type CallSig = (Quaternion, Quaternion, f64);
        let args = (to, weight,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(311usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "slerpni", self.sys_ptr, args)
        }
    }
    pub fn spherical_cubic_interpolate(&self, b: Quaternion, pre_a: Quaternion, post_b: Quaternion, weight: f64,) -> Quaternion {
        type RetMarshal = PtrcallReturnT < Quaternion >;
        type CallSig = (Quaternion, Quaternion, Quaternion, Quaternion, f64);
        let args = (b, pre_a, post_b, weight,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(312usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "spherical_cubic_interpolate", self.sys_ptr, args)
        }
    }
    pub fn spherical_cubic_interpolate_in_time(&self, b: Quaternion, pre_a: Quaternion, post_b: Quaternion, weight: f64, b_t: f64, pre_a_t: f64, post_b_t: f64,) -> Quaternion {
        type RetMarshal = PtrcallReturnT < Quaternion >;
        type CallSig = (Quaternion, Quaternion, Quaternion, Quaternion, f64, f64, f64, f64);
        let args = (b, pre_a, post_b, weight, b_t, pre_a_t, post_b_t,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(313usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "spherical_cubic_interpolate_in_time", self.sys_ptr, args)
        }
    }
    pub fn get_euler(&self, order: i64,) -> Vector3 {
        type RetMarshal = PtrcallReturnT < Vector3 >;
        type CallSig = (Vector3, i64);
        let args = (order,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(314usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "get_euler", self.sys_ptr, args)
        }
    }
    pub fn from_euler(euler: Vector3,) -> Quaternion {
        type RetMarshal = PtrcallReturnT < Quaternion >;
        type CallSig = (Quaternion, Vector3);
        let args = (euler,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(315usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "from_euler", std::ptr::null_mut(), args)
        }
    }
    pub fn get_axis(&self,) -> Vector3 {
        type RetMarshal = PtrcallReturnT < Vector3 >;
        type CallSig = (Vector3,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(316usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "get_axis", self.sys_ptr, args)
        }
    }
    pub fn get_angle(&self,) -> f64 {
        type RetMarshal = PtrcallReturnT < f64 >;
        type CallSig = (f64,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(317usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "get_angle", self.sys_ptr, args)
        }
    }
}