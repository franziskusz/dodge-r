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
pub struct InnerNodePath < 'a > {
    _outer_lifetime: std::marker::PhantomData < &'a() >, sys_ptr: sys::GDExtensionTypePtr,
}
impl < 'a > InnerNodePath < 'a > {
    pub fn from_outer(outer: &NodePath) -> Self {
        Self {
            _outer_lifetime: std::marker::PhantomData, sys_ptr: outer.sys(),
        }
    }
    pub fn is_absolute(&self,) -> bool {
        type RetMarshal = PtrcallReturnT < bool >;
        type CallSig = (bool,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(524usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "is_absolute", self.sys_ptr, args)
        }
    }
    pub fn get_name_count(&self,) -> i64 {
        type RetMarshal = PtrcallReturnT < i64 >;
        type CallSig = (i64,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(525usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "get_name_count", self.sys_ptr, args)
        }
    }
    pub fn get_name(&self, idx: i64,) -> StringName {
        type RetMarshal = PtrcallReturnT < StringName >;
        type CallSig = (StringName, i64);
        let args = (idx,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(526usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "get_name", self.sys_ptr, args)
        }
    }
    pub fn get_subname_count(&self,) -> i64 {
        type RetMarshal = PtrcallReturnT < i64 >;
        type CallSig = (i64,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(527usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "get_subname_count", self.sys_ptr, args)
        }
    }
    pub fn hash(&self,) -> i64 {
        type RetMarshal = PtrcallReturnT < i64 >;
        type CallSig = (i64,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(528usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "hash", self.sys_ptr, args)
        }
    }
    pub fn get_subname(&self, idx: i64,) -> StringName {
        type RetMarshal = PtrcallReturnT < StringName >;
        type CallSig = (StringName, i64);
        let args = (idx,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(529usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "get_subname", self.sys_ptr, args)
        }
    }
    pub fn get_concatenated_names(&self,) -> StringName {
        type RetMarshal = PtrcallReturnT < StringName >;
        type CallSig = (StringName,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(530usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "get_concatenated_names", self.sys_ptr, args)
        }
    }
    pub fn get_concatenated_subnames(&self,) -> StringName {
        type RetMarshal = PtrcallReturnT < StringName >;
        type CallSig = (StringName,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(531usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "get_concatenated_subnames", self.sys_ptr, args)
        }
    }
    pub fn get_as_property_path(&self,) -> NodePath {
        type RetMarshal = PtrcallReturnT < NodePath >;
        type CallSig = (NodePath,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(532usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "get_as_property_path", self.sys_ptr, args)
        }
    }
    pub fn is_empty(&self,) -> bool {
        type RetMarshal = PtrcallReturnT < bool >;
        type CallSig = (bool,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(533usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "is_empty", self.sys_ptr, args)
        }
    }
}