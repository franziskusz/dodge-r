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
pub struct InnerPackedByteArray < 'a > {
    _outer_lifetime: std::marker::PhantomData < &'a() >, sys_ptr: sys::GDExtensionTypePtr,
}
impl < 'a > InnerPackedByteArray < 'a > {
    pub fn from_outer(outer: &PackedByteArray) -> Self {
        Self {
            _outer_lifetime: std::marker::PhantomData, sys_ptr: outer.sys(),
        }
    }
    pub fn size(&self,) -> i64 {
        type RetMarshal = PtrcallReturnT < i64 >;
        type CallSig = (i64,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(618usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "size", self.sys_ptr, args)
        }
    }
    pub fn is_empty(&self,) -> bool {
        type RetMarshal = PtrcallReturnT < bool >;
        type CallSig = (bool,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(619usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "is_empty", self.sys_ptr, args)
        }
    }
    pub fn set(&mut self, index: i64, value: i64,) {
        type RetMarshal = PtrcallReturnUnit;
        type CallSig = ((), i64, i64);
        let args = (index, value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(620usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "set", self.sys_ptr, args)
        }
    }
    pub fn push_back(&mut self, value: i64,) -> bool {
        type RetMarshal = PtrcallReturnT < bool >;
        type CallSig = (bool, i64);
        let args = (value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(621usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "push_back", self.sys_ptr, args)
        }
    }
    pub fn append(&mut self, value: i64,) -> bool {
        type RetMarshal = PtrcallReturnT < bool >;
        type CallSig = (bool, i64);
        let args = (value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(622usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "append", self.sys_ptr, args)
        }
    }
    pub fn append_array(&mut self, array: PackedByteArray,) {
        type RetMarshal = PtrcallReturnUnit;
        type CallSig = ((), PackedByteArray);
        let args = (array,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(623usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "append_array", self.sys_ptr, args)
        }
    }
    pub fn remove_at(&mut self, index: i64,) {
        type RetMarshal = PtrcallReturnUnit;
        type CallSig = ((), i64);
        let args = (index,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(624usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "remove_at", self.sys_ptr, args)
        }
    }
    pub fn insert(&mut self, at_index: i64, value: i64,) -> i64 {
        type RetMarshal = PtrcallReturnT < i64 >;
        type CallSig = (i64, i64, i64);
        let args = (at_index, value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(625usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "insert", self.sys_ptr, args)
        }
    }
    pub fn fill(&mut self, value: i64,) {
        type RetMarshal = PtrcallReturnUnit;
        type CallSig = ((), i64);
        let args = (value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(626usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "fill", self.sys_ptr, args)
        }
    }
    pub fn resize(&mut self, new_size: i64,) -> i64 {
        type RetMarshal = PtrcallReturnT < i64 >;
        type CallSig = (i64, i64);
        let args = (new_size,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(627usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "resize", self.sys_ptr, args)
        }
    }
    pub fn clear(&mut self,) {
        type RetMarshal = PtrcallReturnUnit;
        type CallSig = ((),);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(628usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "clear", self.sys_ptr, args)
        }
    }
    pub fn has(&self, value: i64,) -> bool {
        type RetMarshal = PtrcallReturnT < bool >;
        type CallSig = (bool, i64);
        let args = (value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(629usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "has", self.sys_ptr, args)
        }
    }
    pub fn reverse(&mut self,) {
        type RetMarshal = PtrcallReturnUnit;
        type CallSig = ((),);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(630usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "reverse", self.sys_ptr, args)
        }
    }
    pub fn slice(&self, begin: i64, end: i64,) -> PackedByteArray {
        type RetMarshal = PtrcallReturnT < PackedByteArray >;
        type CallSig = (PackedByteArray, i64, i64);
        let args = (begin, end,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(631usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "slice", self.sys_ptr, args)
        }
    }
    pub fn sort(&mut self,) {
        type RetMarshal = PtrcallReturnUnit;
        type CallSig = ((),);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(632usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "sort", self.sys_ptr, args)
        }
    }
    pub fn bsearch(&mut self, value: i64, before: bool,) -> i64 {
        type RetMarshal = PtrcallReturnT < i64 >;
        type CallSig = (i64, i64, bool);
        let args = (value, before,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(633usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "bsearch", self.sys_ptr, args)
        }
    }
    pub fn duplicate(&mut self,) -> PackedByteArray {
        type RetMarshal = PtrcallReturnT < PackedByteArray >;
        type CallSig = (PackedByteArray,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(634usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "duplicate", self.sys_ptr, args)
        }
    }
    pub fn find(&self, value: i64, from: i64,) -> i64 {
        type RetMarshal = PtrcallReturnT < i64 >;
        type CallSig = (i64, i64, i64);
        let args = (value, from,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(635usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "find", self.sys_ptr, args)
        }
    }
    pub fn rfind(&self, value: i64, from: i64,) -> i64 {
        type RetMarshal = PtrcallReturnT < i64 >;
        type CallSig = (i64, i64, i64);
        let args = (value, from,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(636usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "rfind", self.sys_ptr, args)
        }
    }
    pub fn count(&self, value: i64,) -> i64 {
        type RetMarshal = PtrcallReturnT < i64 >;
        type CallSig = (i64, i64);
        let args = (value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(637usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "count", self.sys_ptr, args)
        }
    }
    pub fn get_string_from_ascii(&self,) -> GString {
        type RetMarshal = PtrcallReturnT < GString >;
        type CallSig = (GString,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(638usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "get_string_from_ascii", self.sys_ptr, args)
        }
    }
    pub fn get_string_from_utf8(&self,) -> GString {
        type RetMarshal = PtrcallReturnT < GString >;
        type CallSig = (GString,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(639usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "get_string_from_utf8", self.sys_ptr, args)
        }
    }
    pub fn get_string_from_utf16(&self,) -> GString {
        type RetMarshal = PtrcallReturnT < GString >;
        type CallSig = (GString,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(640usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "get_string_from_utf16", self.sys_ptr, args)
        }
    }
    pub fn get_string_from_utf32(&self,) -> GString {
        type RetMarshal = PtrcallReturnT < GString >;
        type CallSig = (GString,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(641usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "get_string_from_utf32", self.sys_ptr, args)
        }
    }
    pub fn get_string_from_wchar(&self,) -> GString {
        type RetMarshal = PtrcallReturnT < GString >;
        type CallSig = (GString,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(642usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "get_string_from_wchar", self.sys_ptr, args)
        }
    }
    pub fn hex_encode(&self,) -> GString {
        type RetMarshal = PtrcallReturnT < GString >;
        type CallSig = (GString,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(643usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "hex_encode", self.sys_ptr, args)
        }
    }
    pub fn compress(&self, compression_mode: i64,) -> PackedByteArray {
        type RetMarshal = PtrcallReturnT < PackedByteArray >;
        type CallSig = (PackedByteArray, i64);
        let args = (compression_mode,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(644usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "compress", self.sys_ptr, args)
        }
    }
    pub fn decompress(&self, buffer_size: i64, compression_mode: i64,) -> PackedByteArray {
        type RetMarshal = PtrcallReturnT < PackedByteArray >;
        type CallSig = (PackedByteArray, i64, i64);
        let args = (buffer_size, compression_mode,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(645usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "decompress", self.sys_ptr, args)
        }
    }
    pub fn decompress_dynamic(&self, max_output_size: i64, compression_mode: i64,) -> PackedByteArray {
        type RetMarshal = PtrcallReturnT < PackedByteArray >;
        type CallSig = (PackedByteArray, i64, i64);
        let args = (max_output_size, compression_mode,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(646usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "decompress_dynamic", self.sys_ptr, args)
        }
    }
    pub fn decode_u8(&self, byte_offset: i64,) -> i64 {
        type RetMarshal = PtrcallReturnT < i64 >;
        type CallSig = (i64, i64);
        let args = (byte_offset,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(647usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "decode_u8", self.sys_ptr, args)
        }
    }
    pub fn decode_s8(&self, byte_offset: i64,) -> i64 {
        type RetMarshal = PtrcallReturnT < i64 >;
        type CallSig = (i64, i64);
        let args = (byte_offset,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(648usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "decode_s8", self.sys_ptr, args)
        }
    }
    pub fn decode_u16(&self, byte_offset: i64,) -> i64 {
        type RetMarshal = PtrcallReturnT < i64 >;
        type CallSig = (i64, i64);
        let args = (byte_offset,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(649usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "decode_u16", self.sys_ptr, args)
        }
    }
    pub fn decode_s16(&self, byte_offset: i64,) -> i64 {
        type RetMarshal = PtrcallReturnT < i64 >;
        type CallSig = (i64, i64);
        let args = (byte_offset,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(650usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "decode_s16", self.sys_ptr, args)
        }
    }
    pub fn decode_u32(&self, byte_offset: i64,) -> i64 {
        type RetMarshal = PtrcallReturnT < i64 >;
        type CallSig = (i64, i64);
        let args = (byte_offset,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(651usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "decode_u32", self.sys_ptr, args)
        }
    }
    pub fn decode_s32(&self, byte_offset: i64,) -> i64 {
        type RetMarshal = PtrcallReturnT < i64 >;
        type CallSig = (i64, i64);
        let args = (byte_offset,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(652usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "decode_s32", self.sys_ptr, args)
        }
    }
    pub fn decode_u64(&self, byte_offset: i64,) -> i64 {
        type RetMarshal = PtrcallReturnT < i64 >;
        type CallSig = (i64, i64);
        let args = (byte_offset,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(653usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "decode_u64", self.sys_ptr, args)
        }
    }
    pub fn decode_s64(&self, byte_offset: i64,) -> i64 {
        type RetMarshal = PtrcallReturnT < i64 >;
        type CallSig = (i64, i64);
        let args = (byte_offset,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(654usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "decode_s64", self.sys_ptr, args)
        }
    }
    pub fn decode_half(&self, byte_offset: i64,) -> f64 {
        type RetMarshal = PtrcallReturnT < f64 >;
        type CallSig = (f64, i64);
        let args = (byte_offset,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(655usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "decode_half", self.sys_ptr, args)
        }
    }
    pub fn decode_float(&self, byte_offset: i64,) -> f64 {
        type RetMarshal = PtrcallReturnT < f64 >;
        type CallSig = (f64, i64);
        let args = (byte_offset,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(656usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "decode_float", self.sys_ptr, args)
        }
    }
    pub fn decode_double(&self, byte_offset: i64,) -> f64 {
        type RetMarshal = PtrcallReturnT < f64 >;
        type CallSig = (f64, i64);
        let args = (byte_offset,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(657usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "decode_double", self.sys_ptr, args)
        }
    }
    pub fn has_encoded_var(&self, byte_offset: i64, allow_objects: bool,) -> bool {
        type RetMarshal = PtrcallReturnT < bool >;
        type CallSig = (bool, i64, bool);
        let args = (byte_offset, allow_objects,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(658usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "has_encoded_var", self.sys_ptr, args)
        }
    }
    pub fn decode_var(&self, byte_offset: i64, allow_objects: bool,) -> Variant {
        type RetMarshal = PtrcallReturnT < Variant >;
        type CallSig = (Variant, i64, bool);
        let args = (byte_offset, allow_objects,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(659usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "decode_var", self.sys_ptr, args)
        }
    }
    pub fn decode_var_size(&self, byte_offset: i64, allow_objects: bool,) -> i64 {
        type RetMarshal = PtrcallReturnT < i64 >;
        type CallSig = (i64, i64, bool);
        let args = (byte_offset, allow_objects,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(660usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "decode_var_size", self.sys_ptr, args)
        }
    }
    pub fn to_int32_array(&self,) -> PackedInt32Array {
        type RetMarshal = PtrcallReturnT < PackedInt32Array >;
        type CallSig = (PackedInt32Array,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(661usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "to_int32_array", self.sys_ptr, args)
        }
    }
    pub fn to_int64_array(&self,) -> PackedInt64Array {
        type RetMarshal = PtrcallReturnT < PackedInt64Array >;
        type CallSig = (PackedInt64Array,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(662usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "to_int64_array", self.sys_ptr, args)
        }
    }
    pub fn to_float32_array(&self,) -> PackedFloat32Array {
        type RetMarshal = PtrcallReturnT < PackedFloat32Array >;
        type CallSig = (PackedFloat32Array,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(663usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "to_float32_array", self.sys_ptr, args)
        }
    }
    pub fn to_float64_array(&self,) -> PackedFloat64Array {
        type RetMarshal = PtrcallReturnT < PackedFloat64Array >;
        type CallSig = (PackedFloat64Array,);
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(664usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "to_float64_array", self.sys_ptr, args)
        }
    }
    pub fn encode_u8(&mut self, byte_offset: i64, value: i64,) {
        type RetMarshal = PtrcallReturnUnit;
        type CallSig = ((), i64, i64);
        let args = (byte_offset, value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(665usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "encode_u8", self.sys_ptr, args)
        }
    }
    pub fn encode_s8(&mut self, byte_offset: i64, value: i64,) {
        type RetMarshal = PtrcallReturnUnit;
        type CallSig = ((), i64, i64);
        let args = (byte_offset, value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(666usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "encode_s8", self.sys_ptr, args)
        }
    }
    pub fn encode_u16(&mut self, byte_offset: i64, value: i64,) {
        type RetMarshal = PtrcallReturnUnit;
        type CallSig = ((), i64, i64);
        let args = (byte_offset, value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(667usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "encode_u16", self.sys_ptr, args)
        }
    }
    pub fn encode_s16(&mut self, byte_offset: i64, value: i64,) {
        type RetMarshal = PtrcallReturnUnit;
        type CallSig = ((), i64, i64);
        let args = (byte_offset, value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(668usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "encode_s16", self.sys_ptr, args)
        }
    }
    pub fn encode_u32(&mut self, byte_offset: i64, value: i64,) {
        type RetMarshal = PtrcallReturnUnit;
        type CallSig = ((), i64, i64);
        let args = (byte_offset, value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(669usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "encode_u32", self.sys_ptr, args)
        }
    }
    pub fn encode_s32(&mut self, byte_offset: i64, value: i64,) {
        type RetMarshal = PtrcallReturnUnit;
        type CallSig = ((), i64, i64);
        let args = (byte_offset, value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(670usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "encode_s32", self.sys_ptr, args)
        }
    }
    pub fn encode_u64(&mut self, byte_offset: i64, value: i64,) {
        type RetMarshal = PtrcallReturnUnit;
        type CallSig = ((), i64, i64);
        let args = (byte_offset, value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(671usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "encode_u64", self.sys_ptr, args)
        }
    }
    pub fn encode_s64(&mut self, byte_offset: i64, value: i64,) {
        type RetMarshal = PtrcallReturnUnit;
        type CallSig = ((), i64, i64);
        let args = (byte_offset, value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(672usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "encode_s64", self.sys_ptr, args)
        }
    }
    pub fn encode_half(&mut self, byte_offset: i64, value: f64,) {
        type RetMarshal = PtrcallReturnUnit;
        type CallSig = ((), i64, f64);
        let args = (byte_offset, value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(673usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "encode_half", self.sys_ptr, args)
        }
    }
    pub fn encode_float(&mut self, byte_offset: i64, value: f64,) {
        type RetMarshal = PtrcallReturnUnit;
        type CallSig = ((), i64, f64);
        let args = (byte_offset, value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(674usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "encode_float", self.sys_ptr, args)
        }
    }
    pub fn encode_double(&mut self, byte_offset: i64, value: f64,) {
        type RetMarshal = PtrcallReturnUnit;
        type CallSig = ((), i64, f64);
        let args = (byte_offset, value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(675usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "encode_double", self.sys_ptr, args)
        }
    }
    pub fn encode_var(&mut self, byte_offset: i64, value: Variant, allow_objects: bool,) -> i64 {
        type RetMarshal = PtrcallReturnT < i64 >;
        type CallSig = (i64, i64, Variant, bool);
        let args = (byte_offset, value, allow_objects,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(676usize);
            < CallSig as PtrcallSignatureTuple > ::out_builtin_ptrcall::< RetMarshal > (method_bind, "encode_var", self.sys_ptr, args)
        }
    }
}