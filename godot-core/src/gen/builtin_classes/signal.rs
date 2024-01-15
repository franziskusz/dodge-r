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
pub struct InnerSignal < 'a > {
    _outer_lifetime: std::marker::PhantomData < &'a() >, sys_ptr: sys::GDExtensionTypePtr,
}
impl < 'a > InnerSignal < 'a > {
    pub fn from_outer(outer: &Signal) -> Self {
        Self {
            _outer_lifetime: std::marker::PhantomData, sys_ptr: outer.sys(),
        }
    }
    pub fn is_null(&self,) -> bool {
        type RetMarshal = PtrcallReturnT < bool >;
        type CallSig = (bool,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(549usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "is_null", self.sys_ptr, args)
        }
    }
    pub fn get_object(&self,) -> Option < Gd < crate::engine::Object > > {
        type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::Object > >;
        type CallSig = (Option < Gd < crate::engine::Object > >,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(550usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "get_object", self.sys_ptr, args)
        }
    }
    pub fn get_object_id(&self,) -> i64 {
        type RetMarshal = PtrcallReturnT < i64 >;
        type CallSig = (i64,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(551usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "get_object_id", self.sys_ptr, args)
        }
    }
    pub fn get_name(&self,) -> StringName {
        type RetMarshal = PtrcallReturnT < StringName >;
        type CallSig = (StringName,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(552usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "get_name", self.sys_ptr, args)
        }
    }
    pub fn connect(&mut self, callable: Callable, flags: i64,) -> i64 {
        type RetMarshal = PtrcallReturnT < i64 >;
        type CallSig = (i64, Callable, i64);
        let args = (callable, flags,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(553usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "connect", self.sys_ptr, args)
        }
    }
    pub fn disconnect(&mut self, callable: Callable,) {
        type RetMarshal = PtrcallReturnUnit;
        type CallSig = ((), Callable);
        let args = (callable,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(554usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "disconnect", self.sys_ptr, args)
        }
    }
    pub fn is_connected(&self, callable: Callable,) -> bool {
        type RetMarshal = PtrcallReturnT < bool >;
        type CallSig = (bool, Callable);
        let args = (callable,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(555usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "is_connected", self.sys_ptr, args)
        }
    }
    pub fn get_connections(&self,) -> VariantArray {
        type RetMarshal = PtrcallReturnT < VariantArray >;
        type CallSig = (VariantArray,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(556usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "get_connections", self.sys_ptr, args)
        }
    }
}