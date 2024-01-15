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
pub struct InnerRect2i < 'a > {
    _outer_lifetime: std::marker::PhantomData < &'a() >, sys_ptr: sys::GDExtensionTypePtr,
}
impl < 'a > InnerRect2i < 'a > {
    pub fn from_outer(outer: &Rect2i) -> Self {
        Self {
            _outer_lifetime: std::marker::PhantomData, sys_ptr: outer.sys(),
        }
    }
    pub fn get_center(&self,) -> Vector2i {
        type RetMarshal = PtrcallReturnT < Vector2i >;
        type CallSig = (Vector2i,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(170usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "get_center", self.sys_ptr, args)
        }
    }
    pub fn get_area(&self,) -> i64 {
        type RetMarshal = PtrcallReturnT < i64 >;
        type CallSig = (i64,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(171usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "get_area", self.sys_ptr, args)
        }
    }
    pub fn has_area(&self,) -> bool {
        type RetMarshal = PtrcallReturnT < bool >;
        type CallSig = (bool,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(172usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "has_area", self.sys_ptr, args)
        }
    }
    pub fn has_point(&self, point: Vector2i,) -> bool {
        type RetMarshal = PtrcallReturnT < bool >;
        type CallSig = (bool, Vector2i);
        let args = (point,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(173usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "has_point", self.sys_ptr, args)
        }
    }
    pub fn intersects(&self, b: Rect2i,) -> bool {
        type RetMarshal = PtrcallReturnT < bool >;
        type CallSig = (bool, Rect2i);
        let args = (b,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(174usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "intersects", self.sys_ptr, args)
        }
    }
    pub fn encloses(&self, b: Rect2i,) -> bool {
        type RetMarshal = PtrcallReturnT < bool >;
        type CallSig = (bool, Rect2i);
        let args = (b,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(175usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "encloses", self.sys_ptr, args)
        }
    }
    pub fn intersection(&self, b: Rect2i,) -> Rect2i {
        type RetMarshal = PtrcallReturnT < Rect2i >;
        type CallSig = (Rect2i, Rect2i);
        let args = (b,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(176usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "intersection", self.sys_ptr, args)
        }
    }
    pub fn merge(&self, b: Rect2i,) -> Rect2i {
        type RetMarshal = PtrcallReturnT < Rect2i >;
        type CallSig = (Rect2i, Rect2i);
        let args = (b,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(177usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "merge", self.sys_ptr, args)
        }
    }
    pub fn expand(&self, to: Vector2i,) -> Rect2i {
        type RetMarshal = PtrcallReturnT < Rect2i >;
        type CallSig = (Rect2i, Vector2i);
        let args = (to,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(178usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "expand", self.sys_ptr, args)
        }
    }
    pub fn grow(&self, amount: i64,) -> Rect2i {
        type RetMarshal = PtrcallReturnT < Rect2i >;
        type CallSig = (Rect2i, i64);
        let args = (amount,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(179usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "grow", self.sys_ptr, args)
        }
    }
    pub fn grow_side(&self, side: i64, amount: i64,) -> Rect2i {
        type RetMarshal = PtrcallReturnT < Rect2i >;
        type CallSig = (Rect2i, i64, i64);
        let args = (side, amount,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(180usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "grow_side", self.sys_ptr, args)
        }
    }
    pub fn grow_individual(&self, left: i64, top: i64, right: i64, bottom: i64,) -> Rect2i {
        type RetMarshal = PtrcallReturnT < Rect2i >;
        type CallSig = (Rect2i, i64, i64, i64, i64);
        let args = (left, top, right, bottom,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(181usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "grow_individual", self.sys_ptr, args)
        }
    }
    pub fn abs(&self,) -> Rect2i {
        type RetMarshal = PtrcallReturnT < Rect2i >;
        type CallSig = (Rect2i,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(182usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "abs", self.sys_ptr, args)
        }
    }
}