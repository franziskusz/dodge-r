#![doc = "Sidecar module for class [`FileAccess`][crate::engine::FileAccess].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `FileAccess` enums](https://docs.godotengine.org/en/stable/classes/class_fileaccess.html#enumerations).\n\n"]
use godot_ffi as sys;
use crate::builtin::*;
use crate::builtin::meta::{
    ClassName, PtrcallReturnUnit, PtrcallReturnT, PtrcallReturnOptionGdT, PtrcallSignatureTuple, VarcallSignatureTuple
};
use crate::engine::native::*;
use crate::engine::Object;
use crate::obj::Gd;
use crate::sys::GodotFfi as _;
use crate::engine::notify::*;
use std::ffi::c_void;
pub(super) mod re_export {
    use super::*;
    #[doc = "Godot class `FileAccess.`\n\nInherits [`RefCounted`][crate::engine::RefCounted].\n\nRelated symbols:\n\n* [`file_access`][crate::engine::file_access]: sidecar module with related enum/flag types\n* [`IFileAccess`][crate::engine::IFileAccess]: virtual methods\n\n\nSee also [Godot docs for `FileAccess`](https://docs.godotengine.org/en/stable/classes/class_fileaccess.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct FileAccess {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`FileAccess`][crate::engine::FileAccess].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `FileAccess` methods](https://docs.godotengine.org/en/stable/classes/class_fileaccess.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IFileAccess: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
        #[doc(hidden)]
        fn register_class(builder: &mut crate::builder::ClassBuilder < Self >) {
            unimplemented !()
        }
        #[doc = r" Godot constructor, accepting an injected `base` object."]
        #[doc = r""]
        #[doc = r" `base` refers to the base instance of the class, which can either be stored in a `#[base]` field or discarded."]
        #[doc = r" This method returns a fully-constructed instance, which will then be moved into a [`Gd<T>`][crate::obj::Gd] pointer."]
        #[doc = r""]
        #[doc = r" If the class has a `#[class(init)]` attribute, this method will be auto-generated and must not be overridden."]
        fn init(base: crate::obj::Base < Self::Base >) -> Self {
            unimplemented !()
        }
        #[doc = r" String representation of the Godot instance."]
        #[doc = r""]
        #[doc = r" Override this method to define how the instance is represented as a string."]
        #[doc = r" Used by `impl Display for Gd<T>`, as well as `str()` and `print()` in GDScript."]
        fn to_string(&self) -> crate::builtin::GString {
            unimplemented !()
        }
        #[doc = r" Called when the object receives a Godot notification."]
        #[doc = r""]
        #[doc = r" The type of notification can be identified through `what`. The enum is designed to hold all possible `NOTIFICATION_*`"]
        #[doc = r" constants that the current class can handle. However, this is not validated in Godot, so an enum variant `Unknown` exists"]
        #[doc = r" to represent integers out of known constants (mistakes or future additions)."]
        #[doc = r""]
        #[doc = r" This method is named `_notification` in Godot, but `on_notification` in Rust. To _send_ notifications, use the"]
        #[doc = r" [`Object::notify`][crate::engine::Object::notify] method."]
        #[doc = r""]
        #[doc = r" See also in Godot docs:"]
        #[doc = r" * [`Object::_notification`](https://docs.godotengine.org/en/stable/classes/class_object.html#class-object-method-notification)."]
        #[doc = r" * [Notifications tutorial](https://docs.godotengine.org/en/stable/tutorials/best_practices/godot_notifications.html)."]
        fn on_notification(&mut self, what: ObjectNotification) {
            unimplemented !()
        }
    }
    impl FileAccess {
        pub fn open(path: GString, flags: crate::engine::file_access::ModeFlags,) -> Option < Gd < crate::engine::FileAccess > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::FileAccess > >;
            type CallSig = (Option < Gd < crate::engine::FileAccess > >, GString, crate::engine::file_access::ModeFlags);
            let args = (path, flags,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2835usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "open", std::ptr::null_mut(), None, args,)
            }
        }
        pub fn open_encrypted(path: GString, mode_flags: crate::engine::file_access::ModeFlags, key: PackedByteArray,) -> Option < Gd < crate::engine::FileAccess > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::FileAccess > >;
            type CallSig = (Option < Gd < crate::engine::FileAccess > >, GString, crate::engine::file_access::ModeFlags, PackedByteArray);
            let args = (path, mode_flags, key,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2836usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "open_encrypted", std::ptr::null_mut(), None, args,)
            }
        }
        pub fn open_encrypted_with_pass(path: GString, mode_flags: crate::engine::file_access::ModeFlags, pass: GString,) -> Option < Gd < crate::engine::FileAccess > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::FileAccess > >;
            type CallSig = (Option < Gd < crate::engine::FileAccess > >, GString, crate::engine::file_access::ModeFlags, GString);
            let args = (path, mode_flags, pass,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2837usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "open_encrypted_with_pass", std::ptr::null_mut(), None, args,)
            }
        }
        pub(crate) fn open_compressed_full(path: GString, mode_flags: crate::engine::file_access::ModeFlags, compression_mode: crate::engine::file_access::CompressionMode,) -> Option < Gd < crate::engine::FileAccess > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::FileAccess > >;
            type CallSig = (Option < Gd < crate::engine::FileAccess > >, GString, crate::engine::file_access::ModeFlags, crate::engine::file_access::CompressionMode);
            let args = (path, mode_flags, compression_mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2838usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "open_compressed", std::ptr::null_mut(), None, args,)
            }
        }
        #[inline]
        pub fn open_compressed(path: GString, mode_flags: crate::engine::file_access::ModeFlags,) -> Option < Gd < crate::engine::FileAccess > > {
            Self::open_compressed_ex(path, mode_flags,) . done()
        }
        #[inline]
        pub fn open_compressed_ex(path: GString, mode_flags: crate::engine::file_access::ModeFlags,) -> ExOpenCompressed {
            ExOpenCompressed::new(path, mode_flags,)
        }
        pub fn get_open_error() -> crate::engine::global::Error {
            type RetMarshal = PtrcallReturnT < crate::engine::global::Error >;
            type CallSig = (crate::engine::global::Error,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2839usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_open_error", std::ptr::null_mut(), None, args,)
            }
        }
        pub fn get_file_as_bytes(path: GString,) -> PackedByteArray {
            type RetMarshal = PtrcallReturnT < PackedByteArray >;
            type CallSig = (PackedByteArray, GString);
            let args = (path,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2840usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_file_as_bytes", std::ptr::null_mut(), None, args,)
            }
        }
        pub fn get_file_as_string(path: GString,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString, GString);
            let args = (path,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2841usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_file_as_string", std::ptr::null_mut(), None, args,)
            }
        }
        pub fn flush(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2842usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "flush", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_path(&self,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2843usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_path", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_path_absolute(&self,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2844usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_path_absolute", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_open(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2845usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_open", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn seek(&mut self, position: u64,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), u64);
            let args = (position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2846usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "seek", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn seek_end_full(&mut self, position: i64,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i64);
            let args = (position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2847usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "seek_end", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn seek_end(&mut self,) {
            self.seek_end_ex() . done()
        }
        #[inline]
        pub fn seek_end_ex(&mut self,) -> ExSeekEnd < '_ > {
            ExSeekEnd::new(self,)
        }
        pub fn get_position(&self,) -> u64 {
            type RetMarshal = PtrcallReturnT < u64 >;
            type CallSig = (u64,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2848usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_position", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_length(&self,) -> u64 {
            type RetMarshal = PtrcallReturnT < u64 >;
            type CallSig = (u64,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2849usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_length", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn eof_reached(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2850usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "eof_reached", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_8(&self,) -> u8 {
            type RetMarshal = PtrcallReturnT < u8 >;
            type CallSig = (u8,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2851usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_8", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_16(&self,) -> u16 {
            type RetMarshal = PtrcallReturnT < u16 >;
            type CallSig = (u16,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2852usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_16", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_32(&self,) -> u32 {
            type RetMarshal = PtrcallReturnT < u32 >;
            type CallSig = (u32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2853usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_32", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_64(&self,) -> u64 {
            type RetMarshal = PtrcallReturnT < u64 >;
            type CallSig = (u64,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2854usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_64", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_float(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2855usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_float", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_double(&self,) -> f64 {
            type RetMarshal = PtrcallReturnT < f64 >;
            type CallSig = (f64,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2856usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_double", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_real(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2857usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_real", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_buffer(&self, length: i64,) -> PackedByteArray {
            type RetMarshal = PtrcallReturnT < PackedByteArray >;
            type CallSig = (PackedByteArray, i64);
            let args = (length,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2858usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_buffer", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_line(&self,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2859usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_line", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn get_csv_line_full(&self, delim: GString,) -> PackedStringArray {
            type RetMarshal = PtrcallReturnT < PackedStringArray >;
            type CallSig = (PackedStringArray, GString);
            let args = (delim,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2860usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_csv_line", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn get_csv_line(&self,) -> PackedStringArray {
            self.get_csv_line_ex() . done()
        }
        #[inline]
        pub fn get_csv_line_ex(&self,) -> ExGetCsvLine < '_ > {
            ExGetCsvLine::new(self,)
        }
        pub(crate) fn get_as_text_full(&self, skip_cr: bool,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString, bool);
            let args = (skip_cr,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2861usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_as_text", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn get_as_text(&self,) -> GString {
            self.get_as_text_ex() . done()
        }
        #[inline]
        pub fn get_as_text_ex(&self,) -> ExGetAsText < '_ > {
            ExGetAsText::new(self,)
        }
        pub fn get_md5(path: GString,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString, GString);
            let args = (path,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2862usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_md5", std::ptr::null_mut(), None, args,)
            }
        }
        pub fn get_sha256(path: GString,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString, GString);
            let args = (path,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2863usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_sha256", std::ptr::null_mut(), None, args,)
            }
        }
        pub fn is_big_endian(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2864usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_big_endian", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_big_endian(&mut self, big_endian: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (big_endian,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2865usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_big_endian", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_error(&self,) -> crate::engine::global::Error {
            type RetMarshal = PtrcallReturnT < crate::engine::global::Error >;
            type CallSig = (crate::engine::global::Error,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2866usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_error", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn get_var_full(&self, allow_objects: bool,) -> Variant {
            type RetMarshal = PtrcallReturnT < Variant >;
            type CallSig = (Variant, bool);
            let args = (allow_objects,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2867usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_var", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn get_var(&self,) -> Variant {
            self.get_var_ex() . done()
        }
        #[inline]
        pub fn get_var_ex(&self,) -> ExGetVar < '_ > {
            ExGetVar::new(self,)
        }
        pub fn store_8(&mut self, value: u8,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), u8);
            let args = (value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2868usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "store_8", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn store_16(&mut self, value: u16,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), u16);
            let args = (value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2869usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "store_16", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn store_32(&mut self, value: u32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), u32);
            let args = (value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2870usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "store_32", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn store_64(&mut self, value: u64,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), u64);
            let args = (value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2871usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "store_64", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn store_float(&mut self, value: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2872usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "store_float", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn store_double(&mut self, value: f64,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f64);
            let args = (value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2873usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "store_double", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn store_real(&mut self, value: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2874usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "store_real", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn store_buffer(&mut self, buffer: PackedByteArray,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), PackedByteArray);
            let args = (buffer,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2875usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "store_buffer", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn store_line(&mut self, line: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString);
            let args = (line,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2876usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "store_line", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn store_csv_line_full(&mut self, values: PackedStringArray, delim: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), PackedStringArray, GString);
            let args = (values, delim,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2877usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "store_csv_line", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn store_csv_line(&mut self, values: PackedStringArray,) {
            self.store_csv_line_ex(values,) . done()
        }
        #[inline]
        pub fn store_csv_line_ex(&mut self, values: PackedStringArray,) -> ExStoreCsvLine < '_ > {
            ExStoreCsvLine::new(self, values,)
        }
        pub fn store_string(&mut self, string: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString);
            let args = (string,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2878usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "store_string", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn store_var_full(&mut self, value: Variant, full_objects: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Variant, bool);
            let args = (value, full_objects,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2879usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "store_var", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn store_var(&mut self, value: Variant,) {
            self.store_var_ex(value,) . done()
        }
        #[inline]
        pub fn store_var_ex(&mut self, value: Variant,) -> ExStoreVar < '_ > {
            ExStoreVar::new(self, value,)
        }
        pub fn store_pascal_string(&mut self, string: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString);
            let args = (string,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2880usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "store_pascal_string", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_pascal_string(&mut self,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2881usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_pascal_string", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn close(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2882usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "close", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn file_exists(path: GString,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, GString);
            let args = (path,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2883usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "file_exists", std::ptr::null_mut(), None, args,)
            }
        }
        pub fn get_modified_time(file: GString,) -> u64 {
            type RetMarshal = PtrcallReturnT < u64 >;
            type CallSig = (u64, GString);
            let args = (file,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2884usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_modified_time", std::ptr::null_mut(), None, args,)
            }
        }
        pub fn get_unix_permissions(file: GString,) -> crate::engine::file_access::UnixPermissionFlags {
            type RetMarshal = PtrcallReturnT < crate::engine::file_access::UnixPermissionFlags >;
            type CallSig = (crate::engine::file_access::UnixPermissionFlags, GString);
            let args = (file,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2885usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_unix_permissions", std::ptr::null_mut(), None, args,)
            }
        }
        pub fn set_unix_permissions(file: GString, permissions: crate::engine::file_access::UnixPermissionFlags,) -> crate::engine::global::Error {
            type RetMarshal = PtrcallReturnT < crate::engine::global::Error >;
            type CallSig = (crate::engine::global::Error, GString, crate::engine::file_access::UnixPermissionFlags);
            let args = (file, permissions,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2886usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_unix_permissions", std::ptr::null_mut(), None, args,)
            }
        }
        pub fn get_hidden_attribute(file: GString,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, GString);
            let args = (file,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2887usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_hidden_attribute", std::ptr::null_mut(), None, args,)
            }
        }
        pub fn set_hidden_attribute(file: GString, hidden: bool,) -> crate::engine::global::Error {
            type RetMarshal = PtrcallReturnT < crate::engine::global::Error >;
            type CallSig = (crate::engine::global::Error, GString, bool);
            let args = (file, hidden,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2888usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_hidden_attribute", std::ptr::null_mut(), None, args,)
            }
        }
        pub fn set_read_only_attribute(file: GString, ro: bool,) -> crate::engine::global::Error {
            type RetMarshal = PtrcallReturnT < crate::engine::global::Error >;
            type CallSig = (crate::engine::global::Error, GString, bool);
            let args = (file, ro,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2889usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_read_only_attribute", std::ptr::null_mut(), None, args,)
            }
        }
        pub fn get_read_only_attribute(file: GString,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, GString);
            let args = (file,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2890usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_read_only_attribute", std::ptr::null_mut(), None, args,)
            }
        }
        fn __checked_id(&self) -> Option < crate::obj::InstanceId > {
            let rtti = unsafe {
                self.rtti.as_ref() . unwrap_unchecked()
            };
            let instance_id = rtti.check_type::< Self > ();
            Some(instance_id)
        }
    }
    impl crate::obj::GodotClass for FileAccess {
        type Base = crate::engine::RefCounted;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"FileAccess\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for FileAccess {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for FileAccess {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::RefCounted > for FileAccess {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for FileAccess {
        
    }
    impl std::ops::Deref for FileAccess {
        type Target = crate::engine::RefCounted;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for FileAccess {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_FileAccess {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::FileAccess > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::RefCounted > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`FileAccess::open_compressed_ex`][super::FileAccess::open_compressed_ex]."]
#[must_use]
pub struct ExOpenCompressed {
    path: GString, mode_flags: crate::engine::file_access::ModeFlags, compression_mode: crate::engine::file_access::CompressionMode,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl ExOpenCompressed {
    fn new(path: GString, mode_flags: crate::engine::file_access::ModeFlags,) -> Self {
        Self {
            path, mode_flags, compression_mode: crate::obj::EngineEnum::from_ord(0),
        }
    }
    #[inline]
    pub fn compression_mode(self, value: crate::engine::file_access::CompressionMode) -> Self {
        Self {
            compression_mode: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::engine::FileAccess > > {
        re_export::FileAccess::open_compressed_full(self.path, self.mode_flags, self.compression_mode,)
    }
}
#[doc = "Default-param extender for [`FileAccess::seek_end_ex`][super::FileAccess::seek_end_ex]."]
#[must_use]
pub struct ExSeekEnd < 'a > {
    surround_object: &'a mut re_export::FileAccess, position: i64,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExSeekEnd < 'a > {
    fn new(surround_object: &'a mut re_export::FileAccess,) -> Self {
        Self {
            surround_object, position: 0i64,
        }
    }
    #[inline]
    pub fn position(self, value: i64) -> Self {
        Self {
            position: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::FileAccess::seek_end_full(self.surround_object, self.position,)
    }
}
#[doc = "Default-param extender for [`FileAccess::get_csv_line_ex`][super::FileAccess::get_csv_line_ex]."]
#[must_use]
pub struct ExGetCsvLine < 'a > {
    surround_object: &'a re_export::FileAccess, delim: GString,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetCsvLine < 'a > {
    fn new(surround_object: &'a re_export::FileAccess,) -> Self {
        Self {
            surround_object, delim: GString::from(","),
        }
    }
    #[inline]
    pub fn delim(self, value: GString) -> Self {
        Self {
            delim: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> PackedStringArray {
        re_export::FileAccess::get_csv_line_full(self.surround_object, self.delim,)
    }
}
#[doc = "Default-param extender for [`FileAccess::get_as_text_ex`][super::FileAccess::get_as_text_ex]."]
#[must_use]
pub struct ExGetAsText < 'a > {
    surround_object: &'a re_export::FileAccess, skip_cr: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetAsText < 'a > {
    fn new(surround_object: &'a re_export::FileAccess,) -> Self {
        Self {
            surround_object, skip_cr: false,
        }
    }
    #[inline]
    pub fn skip_cr(self, value: bool) -> Self {
        Self {
            skip_cr: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> GString {
        re_export::FileAccess::get_as_text_full(self.surround_object, self.skip_cr,)
    }
}
#[doc = "Default-param extender for [`FileAccess::get_var_ex`][super::FileAccess::get_var_ex]."]
#[must_use]
pub struct ExGetVar < 'a > {
    surround_object: &'a re_export::FileAccess, allow_objects: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetVar < 'a > {
    fn new(surround_object: &'a re_export::FileAccess,) -> Self {
        Self {
            surround_object, allow_objects: false,
        }
    }
    #[inline]
    pub fn allow_objects(self, value: bool) -> Self {
        Self {
            allow_objects: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Variant {
        re_export::FileAccess::get_var_full(self.surround_object, self.allow_objects,)
    }
}
#[doc = "Default-param extender for [`FileAccess::store_csv_line_ex`][super::FileAccess::store_csv_line_ex]."]
#[must_use]
pub struct ExStoreCsvLine < 'a > {
    surround_object: &'a mut re_export::FileAccess, values: PackedStringArray, delim: GString,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExStoreCsvLine < 'a > {
    fn new(surround_object: &'a mut re_export::FileAccess, values: PackedStringArray,) -> Self {
        Self {
            surround_object, values, delim: GString::from(","),
        }
    }
    #[inline]
    pub fn delim(self, value: GString) -> Self {
        Self {
            delim: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::FileAccess::store_csv_line_full(self.surround_object, self.values, self.delim,)
    }
}
#[doc = "Default-param extender for [`FileAccess::store_var_ex`][super::FileAccess::store_var_ex]."]
#[must_use]
pub struct ExStoreVar < 'a > {
    surround_object: &'a mut re_export::FileAccess, value: Variant, full_objects: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExStoreVar < 'a > {
    fn new(surround_object: &'a mut re_export::FileAccess, value: Variant,) -> Self {
        Self {
            surround_object, value, full_objects: false,
        }
    }
    #[inline]
    pub fn full_objects(self, value: bool) -> Self {
        Self {
            full_objects: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::FileAccess::store_var_full(self.surround_object, self.value, self.full_objects,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct ModeFlags {
    ord: i32
}
impl ModeFlags {
    pub const READ: Self = Self {
        ord: 1i32
    };
    pub const WRITE: Self = Self {
        ord: 2i32
    };
    pub const READ_WRITE: Self = Self {
        ord: 3i32
    };
    pub const WRITE_READ: Self = Self {
        ord: 7i32
    };
    
}
impl crate::obj::EngineEnum for ModeFlags {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 7i32 => Some(Self {
                ord
            }), _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
impl crate::builtin::meta::GodotConvert for ModeFlags {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for ModeFlags {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for ModeFlags {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct CompressionMode {
    ord: i32
}
impl CompressionMode {
    pub const COMPRESSION_FASTLZ: Self = Self {
        ord: 0i32
    };
    pub const COMPRESSION_DEFLATE: Self = Self {
        ord: 1i32
    };
    pub const COMPRESSION_ZSTD: Self = Self {
        ord: 2i32
    };
    pub const COMPRESSION_GZIP: Self = Self {
        ord: 3i32
    };
    pub const COMPRESSION_BROTLI: Self = Self {
        ord: 4i32
    };
    
}
impl crate::obj::EngineEnum for CompressionMode {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 => Some(Self {
                ord
            }), _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
impl crate::builtin::meta::GodotConvert for CompressionMode {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for CompressionMode {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for CompressionMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub struct UnixPermissionFlags {
    ord: u64
}
impl UnixPermissionFlags {
    pub const UNIX_READ_OWNER: Self = Self {
        ord: 256u64
    };
    pub const UNIX_WRITE_OWNER: Self = Self {
        ord: 128u64
    };
    pub const UNIX_EXECUTE_OWNER: Self = Self {
        ord: 64u64
    };
    pub const UNIX_READ_GROUP: Self = Self {
        ord: 32u64
    };
    pub const UNIX_WRITE_GROUP: Self = Self {
        ord: 16u64
    };
    pub const UNIX_EXECUTE_GROUP: Self = Self {
        ord: 8u64
    };
    pub const UNIX_READ_OTHER: Self = Self {
        ord: 4u64
    };
    pub const UNIX_WRITE_OTHER: Self = Self {
        ord: 2u64
    };
    pub const UNIX_EXECUTE_OTHER: Self = Self {
        ord: 1u64
    };
    pub const UNIX_SET_USER_ID: Self = Self {
        ord: 2048u64
    };
    pub const UNIX_SET_GROUP_ID: Self = Self {
        ord: 1024u64
    };
    pub const UNIX_RESTRICTED_DELETE: Self = Self {
        ord: 512u64
    };
    
}
impl crate::obj::EngineBitfield for UnixPermissionFlags {
    fn try_from_ord(ord: u64) -> Option < Self > {
        Some(Self {
            ord
        })
    }
    fn ord(self) -> u64 {
        self.ord
    }
}
impl std::ops::BitOr for UnixPermissionFlags {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            ord: self.ord | rhs.ord
        }
    }
}
impl crate::builtin::meta::GodotConvert for UnixPermissionFlags {
    type Via = u64;
    
}
impl crate::builtin::meta::ToGodot for UnixPermissionFlags {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineBitfield > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for UnixPermissionFlags {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineBitfield > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}