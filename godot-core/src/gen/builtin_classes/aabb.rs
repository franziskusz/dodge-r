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
pub struct InnerAabb < 'a > {
    _outer_lifetime: std::marker::PhantomData < &'a() >, sys_ptr: sys::GDExtensionTypePtr,
}
impl < 'a > InnerAabb < 'a > {
    pub fn from_outer(outer: &Aabb) -> Self {
        Self {
            _outer_lifetime: std::marker::PhantomData, sys_ptr: outer.sys(),
        }
    }
    pub fn abs(&self,) -> Aabb {
        type RetMarshal = PtrcallReturnT < Aabb >;
        type CallSig = (Aabb,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(318usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "abs", self.sys_ptr, args)
        }
    }
    pub fn get_center(&self,) -> Vector3 {
        type RetMarshal = PtrcallReturnT < Vector3 >;
        type CallSig = (Vector3,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(319usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "get_center", self.sys_ptr, args)
        }
    }
    pub fn get_volume(&self,) -> f64 {
        type RetMarshal = PtrcallReturnT < f64 >;
        type CallSig = (f64,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(320usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "get_volume", self.sys_ptr, args)
        }
    }
    pub fn has_volume(&self,) -> bool {
        type RetMarshal = PtrcallReturnT < bool >;
        type CallSig = (bool,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(321usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "has_volume", self.sys_ptr, args)
        }
    }
    pub fn has_surface(&self,) -> bool {
        type RetMarshal = PtrcallReturnT < bool >;
        type CallSig = (bool,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(322usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "has_surface", self.sys_ptr, args)
        }
    }
    pub fn has_point(&self, point: Vector3,) -> bool {
        type RetMarshal = PtrcallReturnT < bool >;
        type CallSig = (bool, Vector3);
        let args = (point,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(323usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "has_point", self.sys_ptr, args)
        }
    }
    pub fn is_equal_approx(&self, aabb: Aabb,) -> bool {
        type RetMarshal = PtrcallReturnT < bool >;
        type CallSig = (bool, Aabb);
        let args = (aabb,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(324usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "is_equal_approx", self.sys_ptr, args)
        }
    }
    pub fn is_finite(&self,) -> bool {
        type RetMarshal = PtrcallReturnT < bool >;
        type CallSig = (bool,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(325usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "is_finite", self.sys_ptr, args)
        }
    }
    pub fn intersects(&self, with: Aabb,) -> bool {
        type RetMarshal = PtrcallReturnT < bool >;
        type CallSig = (bool, Aabb);
        let args = (with,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(326usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "intersects", self.sys_ptr, args)
        }
    }
    pub fn encloses(&self, with: Aabb,) -> bool {
        type RetMarshal = PtrcallReturnT < bool >;
        type CallSig = (bool, Aabb);
        let args = (with,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(327usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "encloses", self.sys_ptr, args)
        }
    }
    pub fn intersects_plane(&self, plane: Plane,) -> bool {
        type RetMarshal = PtrcallReturnT < bool >;
        type CallSig = (bool, Plane);
        let args = (plane,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(328usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "intersects_plane", self.sys_ptr, args)
        }
    }
    pub fn intersection(&self, with: Aabb,) -> Aabb {
        type RetMarshal = PtrcallReturnT < Aabb >;
        type CallSig = (Aabb, Aabb);
        let args = (with,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(329usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "intersection", self.sys_ptr, args)
        }
    }
    pub fn merge(&self, with: Aabb,) -> Aabb {
        type RetMarshal = PtrcallReturnT < Aabb >;
        type CallSig = (Aabb, Aabb);
        let args = (with,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(330usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "merge", self.sys_ptr, args)
        }
    }
    pub fn expand(&self, to_point: Vector3,) -> Aabb {
        type RetMarshal = PtrcallReturnT < Aabb >;
        type CallSig = (Aabb, Vector3);
        let args = (to_point,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(331usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "expand", self.sys_ptr, args)
        }
    }
    pub fn grow(&self, by: f64,) -> Aabb {
        type RetMarshal = PtrcallReturnT < Aabb >;
        type CallSig = (Aabb, f64);
        let args = (by,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(332usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "grow", self.sys_ptr, args)
        }
    }
    pub fn get_support(&self, dir: Vector3,) -> Vector3 {
        type RetMarshal = PtrcallReturnT < Vector3 >;
        type CallSig = (Vector3, Vector3);
        let args = (dir,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(333usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "get_support", self.sys_ptr, args)
        }
    }
    pub fn get_longest_axis(&self,) -> Vector3 {
        type RetMarshal = PtrcallReturnT < Vector3 >;
        type CallSig = (Vector3,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(334usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "get_longest_axis", self.sys_ptr, args)
        }
    }
    pub fn get_longest_axis_index(&self,) -> i64 {
        type RetMarshal = PtrcallReturnT < i64 >;
        type CallSig = (i64,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(335usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "get_longest_axis_index", self.sys_ptr, args)
        }
    }
    pub fn get_longest_axis_size(&self,) -> f64 {
        type RetMarshal = PtrcallReturnT < f64 >;
        type CallSig = (f64,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(336usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "get_longest_axis_size", self.sys_ptr, args)
        }
    }
    pub fn get_shortest_axis(&self,) -> Vector3 {
        type RetMarshal = PtrcallReturnT < Vector3 >;
        type CallSig = (Vector3,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(337usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "get_shortest_axis", self.sys_ptr, args)
        }
    }
    pub fn get_shortest_axis_index(&self,) -> i64 {
        type RetMarshal = PtrcallReturnT < i64 >;
        type CallSig = (i64,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(338usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "get_shortest_axis_index", self.sys_ptr, args)
        }
    }
    pub fn get_shortest_axis_size(&self,) -> f64 {
        type RetMarshal = PtrcallReturnT < f64 >;
        type CallSig = (f64,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(339usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "get_shortest_axis_size", self.sys_ptr, args)
        }
    }
    pub fn get_endpoint(&self, idx: i64,) -> Vector3 {
        type RetMarshal = PtrcallReturnT < Vector3 >;
        type CallSig = (Vector3, i64);
        let args = (idx,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(340usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "get_endpoint", self.sys_ptr, args)
        }
    }
    pub fn intersects_segment(&self, from: Vector3, to: Vector3,) -> Variant {
        type RetMarshal = PtrcallReturnT < Variant >;
        type CallSig = (Variant, Vector3, Vector3);
        let args = (from, to,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(341usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "intersects_segment", self.sys_ptr, args)
        }
    }
    pub fn intersects_ray(&self, from: Vector3, dir: Vector3,) -> Variant {
        type RetMarshal = PtrcallReturnT < Variant >;
        type CallSig = (Variant, Vector3, Vector3);
        let args = (from, dir,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(342usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "intersects_ray", self.sys_ptr, args)
        }
    }
}