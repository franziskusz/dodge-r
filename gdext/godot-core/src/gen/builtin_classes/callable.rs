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
pub struct InnerCallable < 'a > {
    _outer_lifetime: std::marker::PhantomData < &'a() >, sys_ptr: sys::GDExtensionTypePtr,
}
impl < 'a > InnerCallable < 'a > {
    pub fn from_outer(outer: &Callable) -> Self {
        Self {
            _outer_lifetime: std::marker::PhantomData, sys_ptr: outer.sys(),
        }
    }
    pub fn callv(&self, arguments: VariantArray,) -> Variant {
        type RetMarshal = PtrcallReturnT < Variant >;
        type CallSig = (Variant, VariantArray);
        let args = (arguments,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(536usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "callv", self.sys_ptr, args)
        }
    }
    pub fn is_null(&self,) -> bool {
        type RetMarshal = PtrcallReturnT < bool >;
        type CallSig = (bool,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(537usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "is_null", self.sys_ptr, args)
        }
    }
    pub fn is_custom(&self,) -> bool {
        type RetMarshal = PtrcallReturnT < bool >;
        type CallSig = (bool,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(538usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "is_custom", self.sys_ptr, args)
        }
    }
    pub fn is_standard(&self,) -> bool {
        type RetMarshal = PtrcallReturnT < bool >;
        type CallSig = (bool,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(539usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "is_standard", self.sys_ptr, args)
        }
    }
    pub fn is_valid(&self,) -> bool {
        type RetMarshal = PtrcallReturnT < bool >;
        type CallSig = (bool,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(540usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "is_valid", self.sys_ptr, args)
        }
    }
    pub fn get_object(&self,) -> Option < Gd < crate::engine::Object > > {
        type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::Object > >;
        type CallSig = (Option < Gd < crate::engine::Object > >,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(541usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "get_object", self.sys_ptr, args)
        }
    }
    pub fn get_object_id(&self,) -> i64 {
        type RetMarshal = PtrcallReturnT < i64 >;
        type CallSig = (i64,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(542usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "get_object_id", self.sys_ptr, args)
        }
    }
    pub fn get_method(&self,) -> StringName {
        type RetMarshal = PtrcallReturnT < StringName >;
        type CallSig = (StringName,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(543usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "get_method", self.sys_ptr, args)
        }
    }
    pub fn get_bound_arguments_count(&self,) -> i64 {
        type RetMarshal = PtrcallReturnT < i64 >;
        type CallSig = (i64,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(544usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "get_bound_arguments_count", self.sys_ptr, args)
        }
    }
    pub fn get_bound_arguments(&self,) -> VariantArray {
        type RetMarshal = PtrcallReturnT < VariantArray >;
        type CallSig = (VariantArray,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(545usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "get_bound_arguments", self.sys_ptr, args)
        }
    }
    pub fn hash(&self,) -> i64 {
        type RetMarshal = PtrcallReturnT < i64 >;
        type CallSig = (i64,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(546usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "hash", self.sys_ptr, args)
        }
    }
    pub fn bindv(&mut self, arguments: VariantArray,) -> Callable {
        type RetMarshal = PtrcallReturnT < Callable >;
        type CallSig = (Callable, VariantArray);
        let args = (arguments,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(547usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "bindv", self.sys_ptr, args)
        }
    }
    pub fn unbind(&self, argcount: i64,) -> Callable {
        type RetMarshal = PtrcallReturnT < Callable >;
        type CallSig = (Callable, i64);
        let args = (argcount,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(548usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "unbind", self.sys_ptr, args)
        }
    }
}