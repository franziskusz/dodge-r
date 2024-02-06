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
pub struct InnerPackedFloat32Array < 'a > {
    _outer_lifetime: std::marker::PhantomData < &'a() >, sys_ptr: sys::GDExtensionTypePtr,
}
impl < 'a > InnerPackedFloat32Array < 'a > {
    pub fn from_outer(outer: &PackedFloat32Array) -> Self {
        Self {
            _outer_lifetime: std::marker::PhantomData, sys_ptr: outer.sys(),
        }
    }
    pub fn size(&self,) -> i64 {
        type RetMarshal = PtrcallReturnT < i64 >;
        type CallSig = (i64,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(719usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "size", self.sys_ptr, args)
        }
    }
    pub fn is_empty(&self,) -> bool {
        type RetMarshal = PtrcallReturnT < bool >;
        type CallSig = (bool,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(720usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "is_empty", self.sys_ptr, args)
        }
    }
    pub fn set(&mut self, index: i64, value: f64,) {
        type RetMarshal = PtrcallReturnUnit;
        type CallSig = ((), i64, f64);
        let args = (index, value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(721usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "set", self.sys_ptr, args)
        }
    }
    pub fn push_back(&mut self, value: f64,) -> bool {
        type RetMarshal = PtrcallReturnT < bool >;
        type CallSig = (bool, f64);
        let args = (value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(722usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "push_back", self.sys_ptr, args)
        }
    }
    pub fn append(&mut self, value: f64,) -> bool {
        type RetMarshal = PtrcallReturnT < bool >;
        type CallSig = (bool, f64);
        let args = (value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(723usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "append", self.sys_ptr, args)
        }
    }
    pub fn append_array(&mut self, array: PackedFloat32Array,) {
        type RetMarshal = PtrcallReturnUnit;
        type CallSig = ((), PackedFloat32Array);
        let args = (array,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(724usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "append_array", self.sys_ptr, args)
        }
    }
    pub fn remove_at(&mut self, index: i64,) {
        type RetMarshal = PtrcallReturnUnit;
        type CallSig = ((), i64);
        let args = (index,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(725usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "remove_at", self.sys_ptr, args)
        }
    }
    pub fn insert(&mut self, at_index: i64, value: f64,) -> i64 {
        type RetMarshal = PtrcallReturnT < i64 >;
        type CallSig = (i64, i64, f64);
        let args = (at_index, value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(726usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "insert", self.sys_ptr, args)
        }
    }
    pub fn fill(&mut self, value: f64,) {
        type RetMarshal = PtrcallReturnUnit;
        type CallSig = ((), f64);
        let args = (value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(727usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "fill", self.sys_ptr, args)
        }
    }
    pub fn resize(&mut self, new_size: i64,) -> i64 {
        type RetMarshal = PtrcallReturnT < i64 >;
        type CallSig = (i64, i64);
        let args = (new_size,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(728usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "resize", self.sys_ptr, args)
        }
    }
    pub fn clear(&mut self,) {
        type RetMarshal = PtrcallReturnUnit;
        type CallSig = ((),);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(729usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "clear", self.sys_ptr, args)
        }
    }
    pub fn has(&self, value: f64,) -> bool {
        type RetMarshal = PtrcallReturnT < bool >;
        type CallSig = (bool, f64);
        let args = (value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(730usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "has", self.sys_ptr, args)
        }
    }
    pub fn reverse(&mut self,) {
        type RetMarshal = PtrcallReturnUnit;
        type CallSig = ((),);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(731usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "reverse", self.sys_ptr, args)
        }
    }
    pub fn slice(&self, begin: i64, end: i64,) -> PackedFloat32Array {
        type RetMarshal = PtrcallReturnT < PackedFloat32Array >;
        type CallSig = (PackedFloat32Array, i64, i64);
        let args = (begin, end,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(732usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "slice", self.sys_ptr, args)
        }
    }
    pub fn to_byte_array(&self,) -> PackedByteArray {
        type RetMarshal = PtrcallReturnT < PackedByteArray >;
        type CallSig = (PackedByteArray,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(733usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "to_byte_array", self.sys_ptr, args)
        }
    }
    pub fn sort(&mut self,) {
        type RetMarshal = PtrcallReturnUnit;
        type CallSig = ((),);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(734usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "sort", self.sys_ptr, args)
        }
    }
    pub fn bsearch(&mut self, value: f64, before: bool,) -> i64 {
        type RetMarshal = PtrcallReturnT < i64 >;
        type CallSig = (i64, f64, bool);
        let args = (value, before,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(735usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "bsearch", self.sys_ptr, args)
        }
    }
    pub fn duplicate(&mut self,) -> PackedFloat32Array {
        type RetMarshal = PtrcallReturnT < PackedFloat32Array >;
        type CallSig = (PackedFloat32Array,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(736usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "duplicate", self.sys_ptr, args)
        }
    }
    pub fn find(&self, value: f64, from: i64,) -> i64 {
        type RetMarshal = PtrcallReturnT < i64 >;
        type CallSig = (i64, f64, i64);
        let args = (value, from,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(737usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "find", self.sys_ptr, args)
        }
    }
    pub fn rfind(&self, value: f64, from: i64,) -> i64 {
        type RetMarshal = PtrcallReturnT < i64 >;
        type CallSig = (i64, f64, i64);
        let args = (value, from,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(738usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "rfind", self.sys_ptr, args)
        }
    }
    pub fn count(&self, value: f64,) -> i64 {
        type RetMarshal = PtrcallReturnT < i64 >;
        type CallSig = (i64, f64);
        let args = (value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(739usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "count", self.sys_ptr, args)
        }
    }
}