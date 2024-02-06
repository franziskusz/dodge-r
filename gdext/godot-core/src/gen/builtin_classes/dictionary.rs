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
pub struct InnerDictionary < 'a > {
    _outer_lifetime: std::marker::PhantomData < &'a() >, sys_ptr: sys::GDExtensionTypePtr,
}
impl < 'a > InnerDictionary < 'a > {
    pub fn from_outer(outer: &Dictionary) -> Self {
        Self {
            _outer_lifetime: std::marker::PhantomData, sys_ptr: outer.sys(),
        }
    }
    pub fn size(&self,) -> i64 {
        type RetMarshal = PtrcallReturnT < i64 >;
        type CallSig = (i64,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(557usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "size", self.sys_ptr, args)
        }
    }
    pub fn is_empty(&self,) -> bool {
        type RetMarshal = PtrcallReturnT < bool >;
        type CallSig = (bool,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(558usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "is_empty", self.sys_ptr, args)
        }
    }
    pub fn clear(&mut self,) {
        type RetMarshal = PtrcallReturnUnit;
        type CallSig = ((),);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(559usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "clear", self.sys_ptr, args)
        }
    }
    pub fn merge(&mut self, dictionary: Dictionary, overwrite: bool,) {
        type RetMarshal = PtrcallReturnUnit;
        type CallSig = ((), Dictionary, bool);
        let args = (dictionary, overwrite,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(560usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "merge", self.sys_ptr, args)
        }
    }
    pub fn has(&self, key: Variant,) -> bool {
        type RetMarshal = PtrcallReturnT < bool >;
        type CallSig = (bool, Variant);
        let args = (key,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(561usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "has", self.sys_ptr, args)
        }
    }
    pub fn has_all(&self, keys: VariantArray,) -> bool {
        type RetMarshal = PtrcallReturnT < bool >;
        type CallSig = (bool, VariantArray);
        let args = (keys,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(562usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "has_all", self.sys_ptr, args)
        }
    }
    pub fn find_key(&self, value: Variant,) -> Variant {
        type RetMarshal = PtrcallReturnT < Variant >;
        type CallSig = (Variant, Variant);
        let args = (value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(563usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "find_key", self.sys_ptr, args)
        }
    }
    pub fn erase(&mut self, key: Variant,) -> bool {
        type RetMarshal = PtrcallReturnT < bool >;
        type CallSig = (bool, Variant);
        let args = (key,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(564usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "erase", self.sys_ptr, args)
        }
    }
    pub fn hash(&self,) -> i64 {
        type RetMarshal = PtrcallReturnT < i64 >;
        type CallSig = (i64,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(565usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "hash", self.sys_ptr, args)
        }
    }
    pub fn keys(&self,) -> VariantArray {
        type RetMarshal = PtrcallReturnT < VariantArray >;
        type CallSig = (VariantArray,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(566usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "keys", self.sys_ptr, args)
        }
    }
    pub fn values(&self,) -> VariantArray {
        type RetMarshal = PtrcallReturnT < VariantArray >;
        type CallSig = (VariantArray,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(567usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "values", self.sys_ptr, args)
        }
    }
    pub fn duplicate(&self, deep: bool,) -> Dictionary {
        type RetMarshal = PtrcallReturnT < Dictionary >;
        type CallSig = (Dictionary, bool);
        let args = (deep,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(568usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "duplicate", self.sys_ptr, args)
        }
    }
    pub fn get(&self, key: Variant, default: Variant,) -> Variant {
        type RetMarshal = PtrcallReturnT < Variant >;
        type CallSig = (Variant, Variant, Variant);
        let args = (key, default,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(569usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "get", self.sys_ptr, args)
        }
    }
    pub fn make_read_only(&mut self,) {
        type RetMarshal = PtrcallReturnUnit;
        type CallSig = ((),);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(570usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "make_read_only", self.sys_ptr, args)
        }
    }
    pub fn is_read_only(&self,) -> bool {
        type RetMarshal = PtrcallReturnT < bool >;
        type CallSig = (bool,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(571usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "is_read_only", self.sys_ptr, args)
        }
    }
}