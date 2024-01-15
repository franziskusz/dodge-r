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
pub struct InnerArray < 'a > {
    _outer_lifetime: std::marker::PhantomData < &'a() >, sys_ptr: sys::GDExtensionTypePtr,
}
impl < 'a > InnerArray < 'a > {
    pub fn from_outer(outer: &VariantArray) -> Self {
        Self {
            _outer_lifetime: std::marker::PhantomData, sys_ptr: outer.sys(),
        }
    }
    pub fn from_outer_typed < T > (outer: &Array < T >) -> Self where T: crate::builtin::meta::GodotType {
        Self {
            _outer_lifetime: std::marker::PhantomData, sys_ptr: outer.sys(),
        }
    }
    pub fn size(&self,) -> i64 {
        type RetMarshal = PtrcallReturnT < i64 >;
        type CallSig = (i64,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(572usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "size", self.sys_ptr, args)
        }
    }
    pub fn is_empty(&self,) -> bool {
        type RetMarshal = PtrcallReturnT < bool >;
        type CallSig = (bool,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(573usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "is_empty", self.sys_ptr, args)
        }
    }
    pub fn clear(&mut self,) {
        type RetMarshal = PtrcallReturnUnit;
        type CallSig = ((),);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(574usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "clear", self.sys_ptr, args)
        }
    }
    pub fn hash(&self,) -> i64 {
        type RetMarshal = PtrcallReturnT < i64 >;
        type CallSig = (i64,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(575usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "hash", self.sys_ptr, args)
        }
    }
    pub fn assign(&mut self, array: VariantArray,) {
        type RetMarshal = PtrcallReturnUnit;
        type CallSig = ((), VariantArray);
        let args = (array,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(576usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "assign", self.sys_ptr, args)
        }
    }
    pub fn push_back(&mut self, value: Variant,) {
        type RetMarshal = PtrcallReturnUnit;
        type CallSig = ((), Variant);
        let args = (value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(577usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "push_back", self.sys_ptr, args)
        }
    }
    pub fn push_front(&mut self, value: Variant,) {
        type RetMarshal = PtrcallReturnUnit;
        type CallSig = ((), Variant);
        let args = (value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(578usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "push_front", self.sys_ptr, args)
        }
    }
    pub fn append(&mut self, value: Variant,) {
        type RetMarshal = PtrcallReturnUnit;
        type CallSig = ((), Variant);
        let args = (value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(579usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "append", self.sys_ptr, args)
        }
    }
    pub fn append_array(&mut self, array: VariantArray,) {
        type RetMarshal = PtrcallReturnUnit;
        type CallSig = ((), VariantArray);
        let args = (array,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(580usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "append_array", self.sys_ptr, args)
        }
    }
    pub fn resize(&mut self, size: i64,) -> i64 {
        type RetMarshal = PtrcallReturnT < i64 >;
        type CallSig = (i64, i64);
        let args = (size,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(581usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "resize", self.sys_ptr, args)
        }
    }
    pub fn insert(&mut self, position: i64, value: Variant,) -> i64 {
        type RetMarshal = PtrcallReturnT < i64 >;
        type CallSig = (i64, i64, Variant);
        let args = (position, value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(582usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "insert", self.sys_ptr, args)
        }
    }
    pub fn remove_at(&mut self, position: i64,) {
        type RetMarshal = PtrcallReturnUnit;
        type CallSig = ((), i64);
        let args = (position,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(583usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "remove_at", self.sys_ptr, args)
        }
    }
    pub fn fill(&mut self, value: Variant,) {
        type RetMarshal = PtrcallReturnUnit;
        type CallSig = ((), Variant);
        let args = (value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(584usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "fill", self.sys_ptr, args)
        }
    }
    pub fn erase(&mut self, value: Variant,) {
        type RetMarshal = PtrcallReturnUnit;
        type CallSig = ((), Variant);
        let args = (value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(585usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "erase", self.sys_ptr, args)
        }
    }
    pub fn front(&self,) -> Variant {
        type RetMarshal = PtrcallReturnT < Variant >;
        type CallSig = (Variant,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(586usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "front", self.sys_ptr, args)
        }
    }
    pub fn back(&self,) -> Variant {
        type RetMarshal = PtrcallReturnT < Variant >;
        type CallSig = (Variant,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(587usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "back", self.sys_ptr, args)
        }
    }
    pub fn pick_random(&self,) -> Variant {
        type RetMarshal = PtrcallReturnT < Variant >;
        type CallSig = (Variant,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(588usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "pick_random", self.sys_ptr, args)
        }
    }
    pub fn find(&self, what: Variant, from: i64,) -> i64 {
        type RetMarshal = PtrcallReturnT < i64 >;
        type CallSig = (i64, Variant, i64);
        let args = (what, from,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(589usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "find", self.sys_ptr, args)
        }
    }
    pub fn rfind(&self, what: Variant, from: i64,) -> i64 {
        type RetMarshal = PtrcallReturnT < i64 >;
        type CallSig = (i64, Variant, i64);
        let args = (what, from,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(590usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "rfind", self.sys_ptr, args)
        }
    }
    pub fn count(&self, value: Variant,) -> i64 {
        type RetMarshal = PtrcallReturnT < i64 >;
        type CallSig = (i64, Variant);
        let args = (value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(591usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "count", self.sys_ptr, args)
        }
    }
    pub fn has(&self, value: Variant,) -> bool {
        type RetMarshal = PtrcallReturnT < bool >;
        type CallSig = (bool, Variant);
        let args = (value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(592usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "has", self.sys_ptr, args)
        }
    }
    pub fn pop_back(&mut self,) -> Variant {
        type RetMarshal = PtrcallReturnT < Variant >;
        type CallSig = (Variant,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(593usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "pop_back", self.sys_ptr, args)
        }
    }
    pub fn pop_front(&mut self,) -> Variant {
        type RetMarshal = PtrcallReturnT < Variant >;
        type CallSig = (Variant,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(594usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "pop_front", self.sys_ptr, args)
        }
    }
    pub fn pop_at(&mut self, position: i64,) -> Variant {
        type RetMarshal = PtrcallReturnT < Variant >;
        type CallSig = (Variant, i64);
        let args = (position,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(595usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "pop_at", self.sys_ptr, args)
        }
    }
    pub fn sort(&mut self,) {
        type RetMarshal = PtrcallReturnUnit;
        type CallSig = ((),);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(596usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "sort", self.sys_ptr, args)
        }
    }
    pub fn sort_custom(&mut self, func: Callable,) {
        type RetMarshal = PtrcallReturnUnit;
        type CallSig = ((), Callable);
        let args = (func,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(597usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "sort_custom", self.sys_ptr, args)
        }
    }
    pub fn shuffle(&mut self,) {
        type RetMarshal = PtrcallReturnUnit;
        type CallSig = ((),);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(598usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "shuffle", self.sys_ptr, args)
        }
    }
    pub fn bsearch(&self, value: Variant, before: bool,) -> i64 {
        type RetMarshal = PtrcallReturnT < i64 >;
        type CallSig = (i64, Variant, bool);
        let args = (value, before,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(599usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "bsearch", self.sys_ptr, args)
        }
    }
    pub fn bsearch_custom(&self, value: Variant, func: Callable, before: bool,) -> i64 {
        type RetMarshal = PtrcallReturnT < i64 >;
        type CallSig = (i64, Variant, Callable, bool);
        let args = (value, func, before,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(600usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "bsearch_custom", self.sys_ptr, args)
        }
    }
    pub fn reverse(&mut self,) {
        type RetMarshal = PtrcallReturnUnit;
        type CallSig = ((),);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(601usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "reverse", self.sys_ptr, args)
        }
    }
    pub fn duplicate(&self, deep: bool,) -> VariantArray {
        type RetMarshal = PtrcallReturnT < VariantArray >;
        type CallSig = (VariantArray, bool);
        let args = (deep,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(602usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "duplicate", self.sys_ptr, args)
        }
    }
    pub fn slice(&self, begin: i64, end: i64, step: i64, deep: bool,) -> VariantArray {
        type RetMarshal = PtrcallReturnT < VariantArray >;
        type CallSig = (VariantArray, i64, i64, i64, bool);
        let args = (begin, end, step, deep,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(603usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "slice", self.sys_ptr, args)
        }
    }
    pub fn filter(&self, method: Callable,) -> VariantArray {
        type RetMarshal = PtrcallReturnT < VariantArray >;
        type CallSig = (VariantArray, Callable);
        let args = (method,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(604usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "filter", self.sys_ptr, args)
        }
    }
    pub fn map(&self, method: Callable,) -> VariantArray {
        type RetMarshal = PtrcallReturnT < VariantArray >;
        type CallSig = (VariantArray, Callable);
        let args = (method,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(605usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "map", self.sys_ptr, args)
        }
    }
    pub fn reduce(&self, method: Callable, accum: Variant,) -> Variant {
        type RetMarshal = PtrcallReturnT < Variant >;
        type CallSig = (Variant, Callable, Variant);
        let args = (method, accum,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(606usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "reduce", self.sys_ptr, args)
        }
    }
    pub fn any(&self, method: Callable,) -> bool {
        type RetMarshal = PtrcallReturnT < bool >;
        type CallSig = (bool, Callable);
        let args = (method,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(607usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "any", self.sys_ptr, args)
        }
    }
    pub fn all(&self, method: Callable,) -> bool {
        type RetMarshal = PtrcallReturnT < bool >;
        type CallSig = (bool, Callable);
        let args = (method,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(608usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "all", self.sys_ptr, args)
        }
    }
    pub fn max(&self,) -> Variant {
        type RetMarshal = PtrcallReturnT < Variant >;
        type CallSig = (Variant,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(609usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "max", self.sys_ptr, args)
        }
    }
    pub fn min(&self,) -> Variant {
        type RetMarshal = PtrcallReturnT < Variant >;
        type CallSig = (Variant,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(610usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "min", self.sys_ptr, args)
        }
    }
    pub fn is_typed(&self,) -> bool {
        type RetMarshal = PtrcallReturnT < bool >;
        type CallSig = (bool,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(611usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "is_typed", self.sys_ptr, args)
        }
    }
    pub fn is_same_typed(&self, array: VariantArray,) -> bool {
        type RetMarshal = PtrcallReturnT < bool >;
        type CallSig = (bool, VariantArray);
        let args = (array,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(612usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "is_same_typed", self.sys_ptr, args)
        }
    }
    pub fn get_typed_builtin(&self,) -> i64 {
        type RetMarshal = PtrcallReturnT < i64 >;
        type CallSig = (i64,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(613usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "get_typed_builtin", self.sys_ptr, args)
        }
    }
    pub fn get_typed_class_name(&self,) -> StringName {
        type RetMarshal = PtrcallReturnT < StringName >;
        type CallSig = (StringName,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(614usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "get_typed_class_name", self.sys_ptr, args)
        }
    }
    pub fn get_typed_script(&self,) -> Variant {
        type RetMarshal = PtrcallReturnT < Variant >;
        type CallSig = (Variant,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(615usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "get_typed_script", self.sys_ptr, args)
        }
    }
    pub fn make_read_only(&mut self,) {
        type RetMarshal = PtrcallReturnUnit;
        type CallSig = ((),);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(616usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "make_read_only", self.sys_ptr, args)
        }
    }
    pub fn is_read_only(&self,) -> bool {
        type RetMarshal = PtrcallReturnT < bool >;
        type CallSig = (bool,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(617usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "is_read_only", self.sys_ptr, args)
        }
    }
}