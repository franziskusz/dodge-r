#![doc = "Sidecar module for class [`Translation`][crate::engine::Translation].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `Translation` enums](https://docs.godotengine.org/en/stable/classes/class_translation.html#enumerations).\n\n"]
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
    #[doc = "Godot class `Translation.`\n\nInherits [`Resource`][crate::engine::Resource].\n\nRelated symbols:\n\n* [`translation`][crate::engine::translation]: sidecar module with related enum/flag types\n* [`ITranslation`][crate::engine::ITranslation]: virtual methods\n\n\nSee also [Godot docs for `Translation`](https://docs.godotengine.org/en/stable/classes/class_translation.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct Translation {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`Translation`][crate::engine::Translation].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `Translation` methods](https://docs.godotengine.org/en/stable/classes/class_translation.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ITranslation: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
        fn get_plural_message(&self, src_message: StringName, src_plural_message: StringName, n: i32, context: StringName,) -> StringName {
            unimplemented !()
        }
        fn get_message(&self, src_message: StringName, context: StringName,) -> StringName {
            unimplemented !()
        }
        fn setup_local_to_scene(&mut self,) {
            unimplemented !()
        }
    }
    impl Translation {
        #[deprecated = "Replaced with `new_gd` in extension trait `NewGd`."]
        pub fn new() -> Gd < Self > {
            crate::obj::Gd::default()
        }
        pub fn set_locale(&mut self, locale: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString);
            let args = (locale,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8948usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_locale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_locale(&self,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8949usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_locale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn add_message_full(&mut self, src_message: StringName, xlated_message: StringName, context: StringName,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), StringName, StringName, StringName);
            let args = (src_message, xlated_message, context,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8950usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_message", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn add_message(&mut self, src_message: StringName, xlated_message: StringName,) {
            self.add_message_ex(src_message, xlated_message,) . done()
        }
        #[inline]
        pub fn add_message_ex(&mut self, src_message: StringName, xlated_message: StringName,) -> ExAddMessage < '_ > {
            ExAddMessage::new(self, src_message, xlated_message,)
        }
        pub(crate) fn add_plural_message_full(&mut self, src_message: StringName, xlated_messages: PackedStringArray, context: StringName,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), StringName, PackedStringArray, StringName);
            let args = (src_message, xlated_messages, context,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8951usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_plural_message", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn add_plural_message(&mut self, src_message: StringName, xlated_messages: PackedStringArray,) {
            self.add_plural_message_ex(src_message, xlated_messages,) . done()
        }
        #[inline]
        pub fn add_plural_message_ex(&mut self, src_message: StringName, xlated_messages: PackedStringArray,) -> ExAddPluralMessage < '_ > {
            ExAddPluralMessage::new(self, src_message, xlated_messages,)
        }
        pub(crate) fn get_message_full(&self, src_message: StringName, context: StringName,) -> StringName {
            type RetMarshal = PtrcallReturnT < StringName >;
            type CallSig = (StringName, StringName, StringName);
            let args = (src_message, context,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8952usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_message", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn get_message(&self, src_message: StringName,) -> StringName {
            self.get_message_ex(src_message,) . done()
        }
        #[inline]
        pub fn get_message_ex(&self, src_message: StringName,) -> ExGetMessage < '_ > {
            ExGetMessage::new(self, src_message,)
        }
        pub(crate) fn get_plural_message_full(&self, src_message: StringName, src_plural_message: StringName, n: i32, context: StringName,) -> StringName {
            type RetMarshal = PtrcallReturnT < StringName >;
            type CallSig = (StringName, StringName, StringName, i32, StringName);
            let args = (src_message, src_plural_message, n, context,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8953usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_plural_message", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn get_plural_message(&self, src_message: StringName, src_plural_message: StringName, n: i32,) -> StringName {
            self.get_plural_message_ex(src_message, src_plural_message, n,) . done()
        }
        #[inline]
        pub fn get_plural_message_ex(&self, src_message: StringName, src_plural_message: StringName, n: i32,) -> ExGetPluralMessage < '_ > {
            ExGetPluralMessage::new(self, src_message, src_plural_message, n,)
        }
        pub(crate) fn erase_message_full(&mut self, src_message: StringName, context: StringName,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), StringName, StringName);
            let args = (src_message, context,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8954usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "erase_message", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn erase_message(&mut self, src_message: StringName,) {
            self.erase_message_ex(src_message,) . done()
        }
        #[inline]
        pub fn erase_message_ex(&mut self, src_message: StringName,) -> ExEraseMessage < '_ > {
            ExEraseMessage::new(self, src_message,)
        }
        pub fn get_message_list(&self,) -> PackedStringArray {
            type RetMarshal = PtrcallReturnT < PackedStringArray >;
            type CallSig = (PackedStringArray,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8955usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_message_list", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_translated_message_list(&self,) -> PackedStringArray {
            type RetMarshal = PtrcallReturnT < PackedStringArray >;
            type CallSig = (PackedStringArray,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8956usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_translated_message_list", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_message_count(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8957usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_message_count", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for Translation {
        type Base = crate::engine::Resource;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"Translation\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for Translation {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for Translation {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::Resource > for Translation {
        
    }
    impl crate::obj::Inherits < crate::engine::RefCounted > for Translation {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for Translation {
        
    }
    impl crate::obj::ExportableObject for Translation {
        
    }
    impl crate::obj::cap::GodotDefault for Translation {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for Translation {
        type Target = crate::engine::Resource;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for Translation {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_Translation {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::Translation > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::Resource > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::RefCounted > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::Object > for $Class {
                
            }
            impl::godot::obj::ExportableObject for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`Translation::add_message_ex`][super::Translation::add_message_ex]."]
#[must_use]
pub struct ExAddMessage < 'a > {
    surround_object: &'a mut re_export::Translation, src_message: StringName, xlated_message: StringName, context: StringName,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddMessage < 'a > {
    fn new(surround_object: &'a mut re_export::Translation, src_message: StringName, xlated_message: StringName,) -> Self {
        Self {
            surround_object, src_message, xlated_message, context: StringName::from(""),
        }
    }
    #[inline]
    pub fn context(self, value: StringName) -> Self {
        Self {
            context: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::Translation::add_message_full(self.surround_object, self.src_message, self.xlated_message, self.context,)
    }
}
#[doc = "Default-param extender for [`Translation::add_plural_message_ex`][super::Translation::add_plural_message_ex]."]
#[must_use]
pub struct ExAddPluralMessage < 'a > {
    surround_object: &'a mut re_export::Translation, src_message: StringName, xlated_messages: PackedStringArray, context: StringName,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddPluralMessage < 'a > {
    fn new(surround_object: &'a mut re_export::Translation, src_message: StringName, xlated_messages: PackedStringArray,) -> Self {
        Self {
            surround_object, src_message, xlated_messages, context: StringName::from(""),
        }
    }
    #[inline]
    pub fn context(self, value: StringName) -> Self {
        Self {
            context: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::Translation::add_plural_message_full(self.surround_object, self.src_message, self.xlated_messages, self.context,)
    }
}
#[doc = "Default-param extender for [`Translation::get_message_ex`][super::Translation::get_message_ex]."]
#[must_use]
pub struct ExGetMessage < 'a > {
    surround_object: &'a re_export::Translation, src_message: StringName, context: StringName,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetMessage < 'a > {
    fn new(surround_object: &'a re_export::Translation, src_message: StringName,) -> Self {
        Self {
            surround_object, src_message, context: StringName::from(""),
        }
    }
    #[inline]
    pub fn context(self, value: StringName) -> Self {
        Self {
            context: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> StringName {
        re_export::Translation::get_message_full(self.surround_object, self.src_message, self.context,)
    }
}
#[doc = "Default-param extender for [`Translation::get_plural_message_ex`][super::Translation::get_plural_message_ex]."]
#[must_use]
pub struct ExGetPluralMessage < 'a > {
    surround_object: &'a re_export::Translation, src_message: StringName, src_plural_message: StringName, n: i32, context: StringName,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetPluralMessage < 'a > {
    fn new(surround_object: &'a re_export::Translation, src_message: StringName, src_plural_message: StringName, n: i32,) -> Self {
        Self {
            surround_object, src_message, src_plural_message, n, context: StringName::from(""),
        }
    }
    #[inline]
    pub fn context(self, value: StringName) -> Self {
        Self {
            context: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> StringName {
        re_export::Translation::get_plural_message_full(self.surround_object, self.src_message, self.src_plural_message, self.n, self.context,)
    }
}
#[doc = "Default-param extender for [`Translation::erase_message_ex`][super::Translation::erase_message_ex]."]
#[must_use]
pub struct ExEraseMessage < 'a > {
    surround_object: &'a mut re_export::Translation, src_message: StringName, context: StringName,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExEraseMessage < 'a > {
    fn new(surround_object: &'a mut re_export::Translation, src_message: StringName,) -> Self {
        Self {
            surround_object, src_message, context: StringName::from(""),
        }
    }
    #[inline]
    pub fn context(self, value: StringName) -> Self {
        Self {
            context: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::Translation::erase_message_full(self.surround_object, self.src_message, self.context,)
    }
}