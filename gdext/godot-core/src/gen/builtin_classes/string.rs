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
pub struct InnerString < 'a > {
    _outer_lifetime: std::marker::PhantomData < &'a() >, sys_ptr: sys::GDExtensionTypePtr,
}
impl < 'a > InnerString < 'a > {
    pub fn from_outer(outer: &GString) -> Self {
        Self {
            _outer_lifetime: std::marker::PhantomData, sys_ptr: outer.sys(),
        }
    }
    pub fn casecmp_to(&self, to: GString,) -> i64 {
        type RetMarshal = PtrcallReturnT < i64 >;
        type CallSig = (i64, GString);
        let args = (to,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(0usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "casecmp_to", self.sys_ptr, args)
        }
    }
    pub fn nocasecmp_to(&self, to: GString,) -> i64 {
        type RetMarshal = PtrcallReturnT < i64 >;
        type CallSig = (i64, GString);
        let args = (to,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(1usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "nocasecmp_to", self.sys_ptr, args)
        }
    }
    pub fn naturalcasecmp_to(&self, to: GString,) -> i64 {
        type RetMarshal = PtrcallReturnT < i64 >;
        type CallSig = (i64, GString);
        let args = (to,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(2usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "naturalcasecmp_to", self.sys_ptr, args)
        }
    }
    pub fn naturalnocasecmp_to(&self, to: GString,) -> i64 {
        type RetMarshal = PtrcallReturnT < i64 >;
        type CallSig = (i64, GString);
        let args = (to,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(3usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "naturalnocasecmp_to", self.sys_ptr, args)
        }
    }
    pub fn length(&self,) -> i64 {
        type RetMarshal = PtrcallReturnT < i64 >;
        type CallSig = (i64,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(4usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "length", self.sys_ptr, args)
        }
    }
    pub fn substr(&self, from: i64, len: i64,) -> GString {
        type RetMarshal = PtrcallReturnT < GString >;
        type CallSig = (GString, i64, i64);
        let args = (from, len,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(5usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "substr", self.sys_ptr, args)
        }
    }
    pub fn get_slice(&self, delimiter: GString, slice: i64,) -> GString {
        type RetMarshal = PtrcallReturnT < GString >;
        type CallSig = (GString, GString, i64);
        let args = (delimiter, slice,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(6usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "get_slice", self.sys_ptr, args)
        }
    }
    pub fn get_slicec(&self, delimiter: i64, slice: i64,) -> GString {
        type RetMarshal = PtrcallReturnT < GString >;
        type CallSig = (GString, i64, i64);
        let args = (delimiter, slice,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(7usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "get_slicec", self.sys_ptr, args)
        }
    }
    pub fn get_slice_count(&self, delimiter: GString,) -> i64 {
        type RetMarshal = PtrcallReturnT < i64 >;
        type CallSig = (i64, GString);
        let args = (delimiter,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(8usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "get_slice_count", self.sys_ptr, args)
        }
    }
    pub fn find(&self, what: GString, from: i64,) -> i64 {
        type RetMarshal = PtrcallReturnT < i64 >;
        type CallSig = (i64, GString, i64);
        let args = (what, from,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(9usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "find", self.sys_ptr, args)
        }
    }
    pub fn count(&self, what: GString, from: i64, to: i64,) -> i64 {
        type RetMarshal = PtrcallReturnT < i64 >;
        type CallSig = (i64, GString, i64, i64);
        let args = (what, from, to,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(10usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "count", self.sys_ptr, args)
        }
    }
    pub fn countn(&self, what: GString, from: i64, to: i64,) -> i64 {
        type RetMarshal = PtrcallReturnT < i64 >;
        type CallSig = (i64, GString, i64, i64);
        let args = (what, from, to,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(11usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "countn", self.sys_ptr, args)
        }
    }
    pub fn findn(&self, what: GString, from: i64,) -> i64 {
        type RetMarshal = PtrcallReturnT < i64 >;
        type CallSig = (i64, GString, i64);
        let args = (what, from,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(12usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "findn", self.sys_ptr, args)
        }
    }
    pub fn rfind(&self, what: GString, from: i64,) -> i64 {
        type RetMarshal = PtrcallReturnT < i64 >;
        type CallSig = (i64, GString, i64);
        let args = (what, from,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(13usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "rfind", self.sys_ptr, args)
        }
    }
    pub fn rfindn(&self, what: GString, from: i64,) -> i64 {
        type RetMarshal = PtrcallReturnT < i64 >;
        type CallSig = (i64, GString, i64);
        let args = (what, from,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(14usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "rfindn", self.sys_ptr, args)
        }
    }
    pub fn match_(&self, expr: GString,) -> bool {
        type RetMarshal = PtrcallReturnT < bool >;
        type CallSig = (bool, GString);
        let args = (expr,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(15usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "match", self.sys_ptr, args)
        }
    }
    pub fn matchn(&self, expr: GString,) -> bool {
        type RetMarshal = PtrcallReturnT < bool >;
        type CallSig = (bool, GString);
        let args = (expr,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(16usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "matchn", self.sys_ptr, args)
        }
    }
    pub fn begins_with(&self, text: GString,) -> bool {
        type RetMarshal = PtrcallReturnT < bool >;
        type CallSig = (bool, GString);
        let args = (text,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(17usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "begins_with", self.sys_ptr, args)
        }
    }
    pub fn ends_with(&self, text: GString,) -> bool {
        type RetMarshal = PtrcallReturnT < bool >;
        type CallSig = (bool, GString);
        let args = (text,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(18usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "ends_with", self.sys_ptr, args)
        }
    }
    pub fn is_subsequence_of(&self, text: GString,) -> bool {
        type RetMarshal = PtrcallReturnT < bool >;
        type CallSig = (bool, GString);
        let args = (text,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(19usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "is_subsequence_of", self.sys_ptr, args)
        }
    }
    pub fn is_subsequence_ofn(&self, text: GString,) -> bool {
        type RetMarshal = PtrcallReturnT < bool >;
        type CallSig = (bool, GString);
        let args = (text,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(20usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "is_subsequence_ofn", self.sys_ptr, args)
        }
    }
    pub fn bigrams(&self,) -> PackedStringArray {
        type RetMarshal = PtrcallReturnT < PackedStringArray >;
        type CallSig = (PackedStringArray,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(21usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "bigrams", self.sys_ptr, args)
        }
    }
    pub fn similarity(&self, text: GString,) -> f64 {
        type RetMarshal = PtrcallReturnT < f64 >;
        type CallSig = (f64, GString);
        let args = (text,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(22usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "similarity", self.sys_ptr, args)
        }
    }
    pub fn format(&self, values: Variant, placeholder: GString,) -> GString {
        type RetMarshal = PtrcallReturnT < GString >;
        type CallSig = (GString, Variant, GString);
        let args = (values, placeholder,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(23usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "format", self.sys_ptr, args)
        }
    }
    pub fn replace(&self, what: GString, forwhat: GString,) -> GString {
        type RetMarshal = PtrcallReturnT < GString >;
        type CallSig = (GString, GString, GString);
        let args = (what, forwhat,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(24usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "replace", self.sys_ptr, args)
        }
    }
    pub fn replacen(&self, what: GString, forwhat: GString,) -> GString {
        type RetMarshal = PtrcallReturnT < GString >;
        type CallSig = (GString, GString, GString);
        let args = (what, forwhat,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(25usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "replacen", self.sys_ptr, args)
        }
    }
    pub fn repeat(&self, count: i64,) -> GString {
        type RetMarshal = PtrcallReturnT < GString >;
        type CallSig = (GString, i64);
        let args = (count,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(26usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "repeat", self.sys_ptr, args)
        }
    }
    pub fn reverse(&self,) -> GString {
        type RetMarshal = PtrcallReturnT < GString >;
        type CallSig = (GString,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(27usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "reverse", self.sys_ptr, args)
        }
    }
    pub fn insert(&self, position: i64, what: GString,) -> GString {
        type RetMarshal = PtrcallReturnT < GString >;
        type CallSig = (GString, i64, GString);
        let args = (position, what,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(28usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "insert", self.sys_ptr, args)
        }
    }
    pub fn erase(&self, position: i64, chars: i64,) -> GString {
        type RetMarshal = PtrcallReturnT < GString >;
        type CallSig = (GString, i64, i64);
        let args = (position, chars,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(29usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "erase", self.sys_ptr, args)
        }
    }
    pub fn capitalize(&self,) -> GString {
        type RetMarshal = PtrcallReturnT < GString >;
        type CallSig = (GString,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(30usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "capitalize", self.sys_ptr, args)
        }
    }
    pub fn to_camel_case(&self,) -> GString {
        type RetMarshal = PtrcallReturnT < GString >;
        type CallSig = (GString,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(31usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "to_camel_case", self.sys_ptr, args)
        }
    }
    pub fn to_pascal_case(&self,) -> GString {
        type RetMarshal = PtrcallReturnT < GString >;
        type CallSig = (GString,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(32usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "to_pascal_case", self.sys_ptr, args)
        }
    }
    pub fn to_snake_case(&self,) -> GString {
        type RetMarshal = PtrcallReturnT < GString >;
        type CallSig = (GString,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(33usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "to_snake_case", self.sys_ptr, args)
        }
    }
    pub fn split(&self, delimiter: GString, allow_empty: bool, maxsplit: i64,) -> PackedStringArray {
        type RetMarshal = PtrcallReturnT < PackedStringArray >;
        type CallSig = (PackedStringArray, GString, bool, i64);
        let args = (delimiter, allow_empty, maxsplit,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(34usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "split", self.sys_ptr, args)
        }
    }
    pub fn rsplit(&self, delimiter: GString, allow_empty: bool, maxsplit: i64,) -> PackedStringArray {
        type RetMarshal = PtrcallReturnT < PackedStringArray >;
        type CallSig = (PackedStringArray, GString, bool, i64);
        let args = (delimiter, allow_empty, maxsplit,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(35usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "rsplit", self.sys_ptr, args)
        }
    }
    pub fn split_floats(&self, delimiter: GString, allow_empty: bool,) -> PackedFloat64Array {
        type RetMarshal = PtrcallReturnT < PackedFloat64Array >;
        type CallSig = (PackedFloat64Array, GString, bool);
        let args = (delimiter, allow_empty,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(36usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "split_floats", self.sys_ptr, args)
        }
    }
    pub fn join(&self, parts: PackedStringArray,) -> GString {
        type RetMarshal = PtrcallReturnT < GString >;
        type CallSig = (GString, PackedStringArray);
        let args = (parts,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(37usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "join", self.sys_ptr, args)
        }
    }
    pub fn to_upper(&self,) -> GString {
        type RetMarshal = PtrcallReturnT < GString >;
        type CallSig = (GString,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(38usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "to_upper", self.sys_ptr, args)
        }
    }
    pub fn to_lower(&self,) -> GString {
        type RetMarshal = PtrcallReturnT < GString >;
        type CallSig = (GString,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(39usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "to_lower", self.sys_ptr, args)
        }
    }
    pub fn left(&self, length: i64,) -> GString {
        type RetMarshal = PtrcallReturnT < GString >;
        type CallSig = (GString, i64);
        let args = (length,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(40usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "left", self.sys_ptr, args)
        }
    }
    pub fn right(&self, length: i64,) -> GString {
        type RetMarshal = PtrcallReturnT < GString >;
        type CallSig = (GString, i64);
        let args = (length,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(41usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "right", self.sys_ptr, args)
        }
    }
    pub fn strip_edges(&self, left: bool, right: bool,) -> GString {
        type RetMarshal = PtrcallReturnT < GString >;
        type CallSig = (GString, bool, bool);
        let args = (left, right,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(42usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "strip_edges", self.sys_ptr, args)
        }
    }
    pub fn strip_escapes(&self,) -> GString {
        type RetMarshal = PtrcallReturnT < GString >;
        type CallSig = (GString,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(43usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "strip_escapes", self.sys_ptr, args)
        }
    }
    pub fn lstrip(&self, chars: GString,) -> GString {
        type RetMarshal = PtrcallReturnT < GString >;
        type CallSig = (GString, GString);
        let args = (chars,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(44usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "lstrip", self.sys_ptr, args)
        }
    }
    pub fn rstrip(&self, chars: GString,) -> GString {
        type RetMarshal = PtrcallReturnT < GString >;
        type CallSig = (GString, GString);
        let args = (chars,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(45usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "rstrip", self.sys_ptr, args)
        }
    }
    pub fn get_extension(&self,) -> GString {
        type RetMarshal = PtrcallReturnT < GString >;
        type CallSig = (GString,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(46usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "get_extension", self.sys_ptr, args)
        }
    }
    pub fn get_basename(&self,) -> GString {
        type RetMarshal = PtrcallReturnT < GString >;
        type CallSig = (GString,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(47usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "get_basename", self.sys_ptr, args)
        }
    }
    pub fn path_join(&self, file: GString,) -> GString {
        type RetMarshal = PtrcallReturnT < GString >;
        type CallSig = (GString, GString);
        let args = (file,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(48usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "path_join", self.sys_ptr, args)
        }
    }
    pub fn unicode_at(&self, at: i64,) -> i64 {
        type RetMarshal = PtrcallReturnT < i64 >;
        type CallSig = (i64, i64);
        let args = (at,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(49usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "unicode_at", self.sys_ptr, args)
        }
    }
    pub fn indent(&self, prefix: GString,) -> GString {
        type RetMarshal = PtrcallReturnT < GString >;
        type CallSig = (GString, GString);
        let args = (prefix,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(50usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "indent", self.sys_ptr, args)
        }
    }
    pub fn dedent(&self,) -> GString {
        type RetMarshal = PtrcallReturnT < GString >;
        type CallSig = (GString,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(51usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "dedent", self.sys_ptr, args)
        }
    }
    pub fn hash(&self,) -> i64 {
        type RetMarshal = PtrcallReturnT < i64 >;
        type CallSig = (i64,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(52usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "hash", self.sys_ptr, args)
        }
    }
    pub fn md5_text(&self,) -> GString {
        type RetMarshal = PtrcallReturnT < GString >;
        type CallSig = (GString,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(53usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "md5_text", self.sys_ptr, args)
        }
    }
    pub fn sha1_text(&self,) -> GString {
        type RetMarshal = PtrcallReturnT < GString >;
        type CallSig = (GString,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(54usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "sha1_text", self.sys_ptr, args)
        }
    }
    pub fn sha256_text(&self,) -> GString {
        type RetMarshal = PtrcallReturnT < GString >;
        type CallSig = (GString,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(55usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "sha256_text", self.sys_ptr, args)
        }
    }
    pub fn md5_buffer(&self,) -> PackedByteArray {
        type RetMarshal = PtrcallReturnT < PackedByteArray >;
        type CallSig = (PackedByteArray,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(56usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "md5_buffer", self.sys_ptr, args)
        }
    }
    pub fn sha1_buffer(&self,) -> PackedByteArray {
        type RetMarshal = PtrcallReturnT < PackedByteArray >;
        type CallSig = (PackedByteArray,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(57usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "sha1_buffer", self.sys_ptr, args)
        }
    }
    pub fn sha256_buffer(&self,) -> PackedByteArray {
        type RetMarshal = PtrcallReturnT < PackedByteArray >;
        type CallSig = (PackedByteArray,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(58usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "sha256_buffer", self.sys_ptr, args)
        }
    }
    pub fn is_empty(&self,) -> bool {
        type RetMarshal = PtrcallReturnT < bool >;
        type CallSig = (bool,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(59usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "is_empty", self.sys_ptr, args)
        }
    }
    pub fn contains(&self, what: GString,) -> bool {
        type RetMarshal = PtrcallReturnT < bool >;
        type CallSig = (bool, GString);
        let args = (what,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(60usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "contains", self.sys_ptr, args)
        }
    }
    pub fn is_absolute_path(&self,) -> bool {
        type RetMarshal = PtrcallReturnT < bool >;
        type CallSig = (bool,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(61usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "is_absolute_path", self.sys_ptr, args)
        }
    }
    pub fn is_relative_path(&self,) -> bool {
        type RetMarshal = PtrcallReturnT < bool >;
        type CallSig = (bool,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(62usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "is_relative_path", self.sys_ptr, args)
        }
    }
    pub fn simplify_path(&self,) -> GString {
        type RetMarshal = PtrcallReturnT < GString >;
        type CallSig = (GString,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(63usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "simplify_path", self.sys_ptr, args)
        }
    }
    pub fn get_base_dir(&self,) -> GString {
        type RetMarshal = PtrcallReturnT < GString >;
        type CallSig = (GString,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(64usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "get_base_dir", self.sys_ptr, args)
        }
    }
    pub fn get_file(&self,) -> GString {
        type RetMarshal = PtrcallReturnT < GString >;
        type CallSig = (GString,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(65usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "get_file", self.sys_ptr, args)
        }
    }
    pub fn xml_escape(&self, escape_quotes: bool,) -> GString {
        type RetMarshal = PtrcallReturnT < GString >;
        type CallSig = (GString, bool);
        let args = (escape_quotes,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(66usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "xml_escape", self.sys_ptr, args)
        }
    }
    pub fn xml_unescape(&self,) -> GString {
        type RetMarshal = PtrcallReturnT < GString >;
        type CallSig = (GString,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(67usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "xml_unescape", self.sys_ptr, args)
        }
    }
    pub fn uri_encode(&self,) -> GString {
        type RetMarshal = PtrcallReturnT < GString >;
        type CallSig = (GString,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(68usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "uri_encode", self.sys_ptr, args)
        }
    }
    pub fn uri_decode(&self,) -> GString {
        type RetMarshal = PtrcallReturnT < GString >;
        type CallSig = (GString,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(69usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "uri_decode", self.sys_ptr, args)
        }
    }
    pub fn c_escape(&self,) -> GString {
        type RetMarshal = PtrcallReturnT < GString >;
        type CallSig = (GString,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(70usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "c_escape", self.sys_ptr, args)
        }
    }
    pub fn c_unescape(&self,) -> GString {
        type RetMarshal = PtrcallReturnT < GString >;
        type CallSig = (GString,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(71usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "c_unescape", self.sys_ptr, args)
        }
    }
    pub fn json_escape(&self,) -> GString {
        type RetMarshal = PtrcallReturnT < GString >;
        type CallSig = (GString,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(72usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "json_escape", self.sys_ptr, args)
        }
    }
    pub fn validate_node_name(&self,) -> GString {
        type RetMarshal = PtrcallReturnT < GString >;
        type CallSig = (GString,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(73usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "validate_node_name", self.sys_ptr, args)
        }
    }
    pub fn validate_filename(&self,) -> GString {
        type RetMarshal = PtrcallReturnT < GString >;
        type CallSig = (GString,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(74usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "validate_filename", self.sys_ptr, args)
        }
    }
    pub fn is_valid_identifier(&self,) -> bool {
        type RetMarshal = PtrcallReturnT < bool >;
        type CallSig = (bool,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(75usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "is_valid_identifier", self.sys_ptr, args)
        }
    }
    pub fn is_valid_int(&self,) -> bool {
        type RetMarshal = PtrcallReturnT < bool >;
        type CallSig = (bool,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(76usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "is_valid_int", self.sys_ptr, args)
        }
    }
    pub fn is_valid_float(&self,) -> bool {
        type RetMarshal = PtrcallReturnT < bool >;
        type CallSig = (bool,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(77usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "is_valid_float", self.sys_ptr, args)
        }
    }
    pub fn is_valid_hex_number(&self, with_prefix: bool,) -> bool {
        type RetMarshal = PtrcallReturnT < bool >;
        type CallSig = (bool, bool);
        let args = (with_prefix,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(78usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "is_valid_hex_number", self.sys_ptr, args)
        }
    }
    pub fn is_valid_html_color(&self,) -> bool {
        type RetMarshal = PtrcallReturnT < bool >;
        type CallSig = (bool,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(79usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "is_valid_html_color", self.sys_ptr, args)
        }
    }
    pub fn is_valid_ip_address(&self,) -> bool {
        type RetMarshal = PtrcallReturnT < bool >;
        type CallSig = (bool,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(80usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "is_valid_ip_address", self.sys_ptr, args)
        }
    }
    pub fn is_valid_filename(&self,) -> bool {
        type RetMarshal = PtrcallReturnT < bool >;
        type CallSig = (bool,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(81usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "is_valid_filename", self.sys_ptr, args)
        }
    }
    pub fn to_int(&self,) -> i64 {
        type RetMarshal = PtrcallReturnT < i64 >;
        type CallSig = (i64,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(82usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "to_int", self.sys_ptr, args)
        }
    }
    pub fn to_float(&self,) -> f64 {
        type RetMarshal = PtrcallReturnT < f64 >;
        type CallSig = (f64,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(83usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "to_float", self.sys_ptr, args)
        }
    }
    pub fn hex_to_int(&self,) -> i64 {
        type RetMarshal = PtrcallReturnT < i64 >;
        type CallSig = (i64,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(84usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "hex_to_int", self.sys_ptr, args)
        }
    }
    pub fn bin_to_int(&self,) -> i64 {
        type RetMarshal = PtrcallReturnT < i64 >;
        type CallSig = (i64,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(85usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "bin_to_int", self.sys_ptr, args)
        }
    }
    pub fn lpad(&self, min_length: i64, character: GString,) -> GString {
        type RetMarshal = PtrcallReturnT < GString >;
        type CallSig = (GString, i64, GString);
        let args = (min_length, character,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(86usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "lpad", self.sys_ptr, args)
        }
    }
    pub fn rpad(&self, min_length: i64, character: GString,) -> GString {
        type RetMarshal = PtrcallReturnT < GString >;
        type CallSig = (GString, i64, GString);
        let args = (min_length, character,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(87usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "rpad", self.sys_ptr, args)
        }
    }
    pub fn pad_decimals(&self, digits: i64,) -> GString {
        type RetMarshal = PtrcallReturnT < GString >;
        type CallSig = (GString, i64);
        let args = (digits,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(88usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "pad_decimals", self.sys_ptr, args)
        }
    }
    pub fn pad_zeros(&self, digits: i64,) -> GString {
        type RetMarshal = PtrcallReturnT < GString >;
        type CallSig = (GString, i64);
        let args = (digits,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(89usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "pad_zeros", self.sys_ptr, args)
        }
    }
    pub fn trim_prefix(&self, prefix: GString,) -> GString {
        type RetMarshal = PtrcallReturnT < GString >;
        type CallSig = (GString, GString);
        let args = (prefix,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(90usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "trim_prefix", self.sys_ptr, args)
        }
    }
    pub fn trim_suffix(&self, suffix: GString,) -> GString {
        type RetMarshal = PtrcallReturnT < GString >;
        type CallSig = (GString, GString);
        let args = (suffix,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(91usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "trim_suffix", self.sys_ptr, args)
        }
    }
    pub fn to_ascii_buffer(&self,) -> PackedByteArray {
        type RetMarshal = PtrcallReturnT < PackedByteArray >;
        type CallSig = (PackedByteArray,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(92usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "to_ascii_buffer", self.sys_ptr, args)
        }
    }
    pub fn to_utf8_buffer(&self,) -> PackedByteArray {
        type RetMarshal = PtrcallReturnT < PackedByteArray >;
        type CallSig = (PackedByteArray,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(93usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "to_utf8_buffer", self.sys_ptr, args)
        }
    }
    pub fn to_utf16_buffer(&self,) -> PackedByteArray {
        type RetMarshal = PtrcallReturnT < PackedByteArray >;
        type CallSig = (PackedByteArray,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(94usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "to_utf16_buffer", self.sys_ptr, args)
        }
    }
    pub fn to_utf32_buffer(&self,) -> PackedByteArray {
        type RetMarshal = PtrcallReturnT < PackedByteArray >;
        type CallSig = (PackedByteArray,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(95usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "to_utf32_buffer", self.sys_ptr, args)
        }
    }
    pub fn hex_decode(&self,) -> PackedByteArray {
        type RetMarshal = PtrcallReturnT < PackedByteArray >;
        type CallSig = (PackedByteArray,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(96usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "hex_decode", self.sys_ptr, args)
        }
    }
    pub fn to_wchar_buffer(&self,) -> PackedByteArray {
        type RetMarshal = PtrcallReturnT < PackedByteArray >;
        type CallSig = (PackedByteArray,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(97usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "to_wchar_buffer", self.sys_ptr, args)
        }
    }
    pub fn num_scientific(number: f64,) -> GString {
        type RetMarshal = PtrcallReturnT < GString >;
        type CallSig = (GString, f64);
        let args = (number,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(98usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "num_scientific", std::ptr::null_mut(), args)
        }
    }
    pub fn num(number: f64, decimals: i64,) -> GString {
        type RetMarshal = PtrcallReturnT < GString >;
        type CallSig = (GString, f64, i64);
        let args = (number, decimals,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(99usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "num", std::ptr::null_mut(), args)
        }
    }
    pub fn num_int64(number: i64, base: i64, capitalize_hex: bool,) -> GString {
        type RetMarshal = PtrcallReturnT < GString >;
        type CallSig = (GString, i64, i64, bool);
        let args = (number, base, capitalize_hex,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(100usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "num_int64", std::ptr::null_mut(), args)
        }
    }
    pub fn num_uint64(number: i64, base: i64, capitalize_hex: bool,) -> GString {
        type RetMarshal = PtrcallReturnT < GString >;
        type CallSig = (GString, i64, i64, bool);
        let args = (number, base, capitalize_hex,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(101usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "num_uint64", std::ptr::null_mut(), args)
        }
    }
    pub fn chr(char: i64,) -> GString {
        type RetMarshal = PtrcallReturnT < GString >;
        type CallSig = (GString, i64);
        let args = (char,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(102usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "chr", std::ptr::null_mut(), args)
        }
    }
    pub fn humanize_size(size: i64,) -> GString {
        type RetMarshal = PtrcallReturnT < GString >;
        type CallSig = (GString, i64);
        let args = (size,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(103usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "humanize_size", std::ptr::null_mut(), args)
        }
    }
}