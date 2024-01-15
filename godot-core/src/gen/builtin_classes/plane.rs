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
pub struct InnerPlane < 'a > {
    _outer_lifetime: std::marker::PhantomData < &'a() >, sys_ptr: sys::GDExtensionTypePtr,
}
impl < 'a > InnerPlane < 'a > {
    pub fn from_outer(outer: &Plane) -> Self {
        Self {
            _outer_lifetime: std::marker::PhantomData, sys_ptr: outer.sys(),
        }
    }
    pub fn normalized(&self,) -> Plane {
        type RetMarshal = PtrcallReturnT < Plane >;
        type CallSig = (Plane,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(288usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "normalized", self.sys_ptr, args)
        }
    }
    pub fn get_center(&self,) -> Vector3 {
        type RetMarshal = PtrcallReturnT < Vector3 >;
        type CallSig = (Vector3,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(289usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "get_center", self.sys_ptr, args)
        }
    }
    pub fn is_equal_approx(&self, to_plane: Plane,) -> bool {
        type RetMarshal = PtrcallReturnT < bool >;
        type CallSig = (bool, Plane);
        let args = (to_plane,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(290usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "is_equal_approx", self.sys_ptr, args)
        }
    }
    pub fn is_finite(&self,) -> bool {
        type RetMarshal = PtrcallReturnT < bool >;
        type CallSig = (bool,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(291usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "is_finite", self.sys_ptr, args)
        }
    }
    pub fn is_point_over(&self, point: Vector3,) -> bool {
        type RetMarshal = PtrcallReturnT < bool >;
        type CallSig = (bool, Vector3);
        let args = (point,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(292usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "is_point_over", self.sys_ptr, args)
        }
    }
    pub fn distance_to(&self, point: Vector3,) -> f64 {
        type RetMarshal = PtrcallReturnT < f64 >;
        type CallSig = (f64, Vector3);
        let args = (point,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(293usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "distance_to", self.sys_ptr, args)
        }
    }
    pub fn has_point(&self, point: Vector3, tolerance: f64,) -> bool {
        type RetMarshal = PtrcallReturnT < bool >;
        type CallSig = (bool, Vector3, f64);
        let args = (point, tolerance,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(294usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "has_point", self.sys_ptr, args)
        }
    }
    pub fn project(&self, point: Vector3,) -> Vector3 {
        type RetMarshal = PtrcallReturnT < Vector3 >;
        type CallSig = (Vector3, Vector3);
        let args = (point,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(295usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "project", self.sys_ptr, args)
        }
    }
    pub fn intersect_3(&self, b: Plane, c: Plane,) -> Variant {
        type RetMarshal = PtrcallReturnT < Variant >;
        type CallSig = (Variant, Plane, Plane);
        let args = (b, c,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(296usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "intersect_3", self.sys_ptr, args)
        }
    }
    pub fn intersects_ray(&self, from: Vector3, dir: Vector3,) -> Variant {
        type RetMarshal = PtrcallReturnT < Variant >;
        type CallSig = (Variant, Vector3, Vector3);
        let args = (from, dir,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(297usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "intersects_ray", self.sys_ptr, args)
        }
    }
    pub fn intersects_segment(&self, from: Vector3, to: Vector3,) -> Variant {
        type RetMarshal = PtrcallReturnT < Variant >;
        type CallSig = (Variant, Vector3, Vector3);
        let args = (from, to,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(298usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "intersects_segment", self.sys_ptr, args)
        }
    }
}