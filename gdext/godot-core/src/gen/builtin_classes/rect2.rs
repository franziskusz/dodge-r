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
pub struct InnerRect2 < 'a > {
    _outer_lifetime: std::marker::PhantomData < &'a() >, sys_ptr: sys::GDExtensionTypePtr,
}
impl < 'a > InnerRect2 < 'a > {
    pub fn from_outer(outer: &Rect2) -> Self {
        Self {
            _outer_lifetime: std::marker::PhantomData, sys_ptr: outer.sys(),
        }
    }
    pub fn get_center(&self,) -> Vector2 {
        type RetMarshal = PtrcallReturnT < Vector2 >;
        type CallSig = (Vector2,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(155usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "get_center", self.sys_ptr, args)
        }
    }
    pub fn get_area(&self,) -> f64 {
        type RetMarshal = PtrcallReturnT < f64 >;
        type CallSig = (f64,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(156usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "get_area", self.sys_ptr, args)
        }
    }
    pub fn has_area(&self,) -> bool {
        type RetMarshal = PtrcallReturnT < bool >;
        type CallSig = (bool,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(157usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "has_area", self.sys_ptr, args)
        }
    }
    pub fn has_point(&self, point: Vector2,) -> bool {
        type RetMarshal = PtrcallReturnT < bool >;
        type CallSig = (bool, Vector2);
        let args = (point,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(158usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "has_point", self.sys_ptr, args)
        }
    }
    pub fn is_equal_approx(&self, rect: Rect2,) -> bool {
        type RetMarshal = PtrcallReturnT < bool >;
        type CallSig = (bool, Rect2);
        let args = (rect,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(159usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "is_equal_approx", self.sys_ptr, args)
        }
    }
    pub fn is_finite(&self,) -> bool {
        type RetMarshal = PtrcallReturnT < bool >;
        type CallSig = (bool,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(160usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "is_finite", self.sys_ptr, args)
        }
    }
    pub fn intersects(&self, b: Rect2, include_borders: bool,) -> bool {
        type RetMarshal = PtrcallReturnT < bool >;
        type CallSig = (bool, Rect2, bool);
        let args = (b, include_borders,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(161usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "intersects", self.sys_ptr, args)
        }
    }
    pub fn encloses(&self, b: Rect2,) -> bool {
        type RetMarshal = PtrcallReturnT < bool >;
        type CallSig = (bool, Rect2);
        let args = (b,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(162usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "encloses", self.sys_ptr, args)
        }
    }
    pub fn intersection(&self, b: Rect2,) -> Rect2 {
        type RetMarshal = PtrcallReturnT < Rect2 >;
        type CallSig = (Rect2, Rect2);
        let args = (b,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(163usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "intersection", self.sys_ptr, args)
        }
    }
    pub fn merge(&self, b: Rect2,) -> Rect2 {
        type RetMarshal = PtrcallReturnT < Rect2 >;
        type CallSig = (Rect2, Rect2);
        let args = (b,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(164usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "merge", self.sys_ptr, args)
        }
    }
    pub fn expand(&self, to: Vector2,) -> Rect2 {
        type RetMarshal = PtrcallReturnT < Rect2 >;
        type CallSig = (Rect2, Vector2);
        let args = (to,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(165usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "expand", self.sys_ptr, args)
        }
    }
    pub fn grow(&self, amount: f64,) -> Rect2 {
        type RetMarshal = PtrcallReturnT < Rect2 >;
        type CallSig = (Rect2, f64);
        let args = (amount,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(166usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "grow", self.sys_ptr, args)
        }
    }
    pub fn grow_side(&self, side: i64, amount: f64,) -> Rect2 {
        type RetMarshal = PtrcallReturnT < Rect2 >;
        type CallSig = (Rect2, i64, f64);
        let args = (side, amount,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(167usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "grow_side", self.sys_ptr, args)
        }
    }
    pub fn grow_individual(&self, left: f64, top: f64, right: f64, bottom: f64,) -> Rect2 {
        type RetMarshal = PtrcallReturnT < Rect2 >;
        type CallSig = (Rect2, f64, f64, f64, f64);
        let args = (left, top, right, bottom,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(168usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "grow_individual", self.sys_ptr, args)
        }
    }
    pub fn abs(&self,) -> Rect2 {
        type RetMarshal = PtrcallReturnT < Rect2 >;
        type CallSig = (Rect2,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(169usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "abs", self.sys_ptr, args)
        }
    }
}