#![doc = "Sidecar module for class [`StreamPeer`][crate::engine::StreamPeer].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `StreamPeer` enums](https://docs.godotengine.org/en/stable/classes/class_streampeer.html#enumerations).\n\n"]
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
    #[doc = "Godot class `StreamPeer.`\n\nInherits [`RefCounted`][crate::engine::RefCounted].\n\nRelated symbols:\n\n* [`stream_peer`][crate::engine::stream_peer]: sidecar module with related enum/flag types\n* [`IStreamPeer`][crate::engine::IStreamPeer]: virtual methods\n\n\nSee also [Godot docs for `StreamPeer`](https://docs.godotengine.org/en/stable/classes/class_streampeer.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct StreamPeer {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`StreamPeer`][crate::engine::StreamPeer].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `StreamPeer` methods](https://docs.godotengine.org/en/stable/classes/class_streampeer.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IStreamPeer: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
    impl StreamPeer {
        pub fn put_data(&mut self, data: PackedByteArray,) -> crate::engine::global::Error {
            type RetMarshal = PtrcallReturnT < crate::engine::global::Error >;
            type CallSig = (crate::engine::global::Error, PackedByteArray);
            let args = (data,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7838usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "put_data", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn put_partial_data(&mut self, data: PackedByteArray,) -> VariantArray {
            type RetMarshal = PtrcallReturnT < VariantArray >;
            type CallSig = (VariantArray, PackedByteArray);
            let args = (data,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7839usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "put_partial_data", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_data(&mut self, bytes: i32,) -> VariantArray {
            type RetMarshal = PtrcallReturnT < VariantArray >;
            type CallSig = (VariantArray, i32);
            let args = (bytes,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7840usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_data", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_partial_data(&mut self, bytes: i32,) -> VariantArray {
            type RetMarshal = PtrcallReturnT < VariantArray >;
            type CallSig = (VariantArray, i32);
            let args = (bytes,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7841usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_partial_data", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_available_bytes(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7842usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_available_bytes", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_big_endian(&mut self, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7843usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_big_endian", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_big_endian_enabled(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7844usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_big_endian_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn put_8(&mut self, value: i8,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i8);
            let args = (value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7845usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "put_8", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn put_u8(&mut self, value: u8,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), u8);
            let args = (value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7846usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "put_u8", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn put_16(&mut self, value: i16,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i16);
            let args = (value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7847usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "put_16", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn put_u16(&mut self, value: u16,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), u16);
            let args = (value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7848usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "put_u16", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn put_32(&mut self, value: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7849usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "put_32", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn put_u32(&mut self, value: u32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), u32);
            let args = (value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7850usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "put_u32", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn put_64(&mut self, value: i64,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i64);
            let args = (value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7851usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "put_64", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn put_u64(&mut self, value: u64,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), u64);
            let args = (value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7852usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "put_u64", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn put_float(&mut self, value: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7853usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "put_float", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn put_double(&mut self, value: f64,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f64);
            let args = (value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7854usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "put_double", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn put_string(&mut self, value: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString);
            let args = (value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7855usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "put_string", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn put_utf8_string(&mut self, value: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString);
            let args = (value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7856usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "put_utf8_string", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn put_var_full(&mut self, value: Variant, full_objects: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Variant, bool);
            let args = (value, full_objects,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7857usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "put_var", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn put_var(&mut self, value: Variant,) {
            self.put_var_ex(value,) . done()
        }
        #[inline]
        pub fn put_var_ex(&mut self, value: Variant,) -> ExPutVar < '_ > {
            ExPutVar::new(self, value,)
        }
        pub fn get_8(&mut self,) -> i8 {
            type RetMarshal = PtrcallReturnT < i8 >;
            type CallSig = (i8,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7858usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_8", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_u8(&mut self,) -> u8 {
            type RetMarshal = PtrcallReturnT < u8 >;
            type CallSig = (u8,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7859usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_u8", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_16(&mut self,) -> i16 {
            type RetMarshal = PtrcallReturnT < i16 >;
            type CallSig = (i16,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7860usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_16", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_u16(&mut self,) -> u16 {
            type RetMarshal = PtrcallReturnT < u16 >;
            type CallSig = (u16,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7861usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_u16", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_32(&mut self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7862usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_32", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_u32(&mut self,) -> u32 {
            type RetMarshal = PtrcallReturnT < u32 >;
            type CallSig = (u32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7863usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_u32", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_64(&mut self,) -> i64 {
            type RetMarshal = PtrcallReturnT < i64 >;
            type CallSig = (i64,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7864usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_64", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_u64(&mut self,) -> u64 {
            type RetMarshal = PtrcallReturnT < u64 >;
            type CallSig = (u64,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7865usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_u64", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_float(&mut self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7866usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_float", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_double(&mut self,) -> f64 {
            type RetMarshal = PtrcallReturnT < f64 >;
            type CallSig = (f64,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7867usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_double", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn get_string_full(&mut self, bytes: i32,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString, i32);
            let args = (bytes,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7868usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_string", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn get_string(&mut self,) -> GString {
            self.get_string_ex() . done()
        }
        #[inline]
        pub fn get_string_ex(&mut self,) -> ExGetString < '_ > {
            ExGetString::new(self,)
        }
        pub(crate) fn get_utf8_string_full(&mut self, bytes: i32,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString, i32);
            let args = (bytes,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7869usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_utf8_string", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn get_utf8_string(&mut self,) -> GString {
            self.get_utf8_string_ex() . done()
        }
        #[inline]
        pub fn get_utf8_string_ex(&mut self,) -> ExGetUtf8String < '_ > {
            ExGetUtf8String::new(self,)
        }
        pub(crate) fn get_var_full(&mut self, allow_objects: bool,) -> Variant {
            type RetMarshal = PtrcallReturnT < Variant >;
            type CallSig = (Variant, bool);
            let args = (allow_objects,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7870usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_var", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn get_var(&mut self,) -> Variant {
            self.get_var_ex() . done()
        }
        #[inline]
        pub fn get_var_ex(&mut self,) -> ExGetVar < '_ > {
            ExGetVar::new(self,)
        }
        fn __checked_id(&self) -> Option < crate::obj::InstanceId > {
            let rtti = unsafe {
                self.rtti.as_ref() . unwrap_unchecked()
            };
            let instance_id = rtti.check_type::< Self > ();
            Some(instance_id)
        }
    }
    impl crate::obj::GodotClass for StreamPeer {
        type Base = crate::engine::RefCounted;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"StreamPeer\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for StreamPeer {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for StreamPeer {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::RefCounted > for StreamPeer {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for StreamPeer {
        
    }
    impl std::ops::Deref for StreamPeer {
        type Target = crate::engine::RefCounted;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for StreamPeer {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_StreamPeer {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::StreamPeer > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::RefCounted > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`StreamPeer::put_var_ex`][super::StreamPeer::put_var_ex]."]
#[must_use]
pub struct ExPutVar < 'a > {
    surround_object: &'a mut re_export::StreamPeer, value: Variant, full_objects: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExPutVar < 'a > {
    fn new(surround_object: &'a mut re_export::StreamPeer, value: Variant,) -> Self {
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
        re_export::StreamPeer::put_var_full(self.surround_object, self.value, self.full_objects,)
    }
}
#[doc = "Default-param extender for [`StreamPeer::get_string_ex`][super::StreamPeer::get_string_ex]."]
#[must_use]
pub struct ExGetString < 'a > {
    surround_object: &'a mut re_export::StreamPeer, bytes: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetString < 'a > {
    fn new(surround_object: &'a mut re_export::StreamPeer,) -> Self {
        Self {
            surround_object, bytes: - 1i32,
        }
    }
    #[inline]
    pub fn bytes(self, value: i32) -> Self {
        Self {
            bytes: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> GString {
        re_export::StreamPeer::get_string_full(self.surround_object, self.bytes,)
    }
}
#[doc = "Default-param extender for [`StreamPeer::get_utf8_string_ex`][super::StreamPeer::get_utf8_string_ex]."]
#[must_use]
pub struct ExGetUtf8String < 'a > {
    surround_object: &'a mut re_export::StreamPeer, bytes: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetUtf8String < 'a > {
    fn new(surround_object: &'a mut re_export::StreamPeer,) -> Self {
        Self {
            surround_object, bytes: - 1i32,
        }
    }
    #[inline]
    pub fn bytes(self, value: i32) -> Self {
        Self {
            bytes: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> GString {
        re_export::StreamPeer::get_utf8_string_full(self.surround_object, self.bytes,)
    }
}
#[doc = "Default-param extender for [`StreamPeer::get_var_ex`][super::StreamPeer::get_var_ex]."]
#[must_use]
pub struct ExGetVar < 'a > {
    surround_object: &'a mut re_export::StreamPeer, allow_objects: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetVar < 'a > {
    fn new(surround_object: &'a mut re_export::StreamPeer,) -> Self {
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
        re_export::StreamPeer::get_var_full(self.surround_object, self.allow_objects,)
    }
}