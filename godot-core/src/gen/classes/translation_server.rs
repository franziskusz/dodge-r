#![doc = "Sidecar module for class [`TranslationServer`][crate::engine::TranslationServer].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `TranslationServer` enums](https://docs.godotengine.org/en/stable/classes/class_translationserver.html#enumerations).\n\n"]
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
    #[doc = "Godot class `TranslationServer.`\n\nInherits [`Object`][crate::engine::Object].\n\nRelated symbols:\n\n* [`translation_server`][crate::engine::translation_server]: sidecar module with related enum/flag types\n* [`ITranslationServer`][crate::engine::ITranslationServer]: virtual methods\n\n\nSee also [Godot docs for `TranslationServer`](https://docs.godotengine.org/en/stable/classes/class_translationserver.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct TranslationServer {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`TranslationServer`][crate::engine::TranslationServer].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `TranslationServer` methods](https://docs.godotengine.org/en/stable/classes/class_translationserver.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ITranslationServer: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
    impl TranslationServer {
        pub fn singleton() -> Gd < Self > {
            unsafe {
                let __class_name = StringName::from_latin1_with_nul(b"TranslationServer\0");
                let __object_ptr = sys::interface_fn !(global_get_singleton) (__class_name.string_sys());
                Gd::from_obj_sys(__object_ptr)
            }
        }
        pub fn set_locale(&mut self, locale: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString);
            let args = (locale,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(900usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_locale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_locale(&self,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(901usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_locale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_tool_locale(&mut self,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(902usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_tool_locale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn compare_locales(&self, locale_a: GString, locale_b: GString,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, GString, GString);
            let args = (locale_a, locale_b,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(903usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "compare_locales", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn standardize_locale(&self, locale: GString,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString, GString);
            let args = (locale,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(904usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "standardize_locale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_all_languages(&self,) -> PackedStringArray {
            type RetMarshal = PtrcallReturnT < PackedStringArray >;
            type CallSig = (PackedStringArray,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(905usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_all_languages", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_language_name(&self, language: GString,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString, GString);
            let args = (language,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(906usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_language_name", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_all_scripts(&self,) -> PackedStringArray {
            type RetMarshal = PtrcallReturnT < PackedStringArray >;
            type CallSig = (PackedStringArray,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(907usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_all_scripts", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_script_name(&self, script: GString,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString, GString);
            let args = (script,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(908usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_script_name", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_all_countries(&self,) -> PackedStringArray {
            type RetMarshal = PtrcallReturnT < PackedStringArray >;
            type CallSig = (PackedStringArray,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(909usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_all_countries", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_country_name(&self, country: GString,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString, GString);
            let args = (country,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(910usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_country_name", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_locale_name(&self, locale: GString,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString, GString);
            let args = (locale,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(911usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_locale_name", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn translate_full(&self, message: StringName, context: StringName,) -> StringName {
            type RetMarshal = PtrcallReturnT < StringName >;
            type CallSig = (StringName, StringName, StringName);
            let args = (message, context,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(912usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "translate", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn translate(&self, message: StringName,) -> StringName {
            self.translate_ex(message,) . done()
        }
        #[inline]
        pub fn translate_ex(&self, message: StringName,) -> ExTranslate < '_ > {
            ExTranslate::new(self, message,)
        }
        pub(crate) fn translate_plural_full(&self, message: StringName, plural_message: StringName, n: i32, context: StringName,) -> StringName {
            type RetMarshal = PtrcallReturnT < StringName >;
            type CallSig = (StringName, StringName, StringName, i32, StringName);
            let args = (message, plural_message, n, context,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(913usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "translate_plural", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn translate_plural(&self, message: StringName, plural_message: StringName, n: i32,) -> StringName {
            self.translate_plural_ex(message, plural_message, n,) . done()
        }
        #[inline]
        pub fn translate_plural_ex(&self, message: StringName, plural_message: StringName, n: i32,) -> ExTranslatePlural < '_ > {
            ExTranslatePlural::new(self, message, plural_message, n,)
        }
        pub fn add_translation(&mut self, translation: Gd < crate::engine::Translation >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::Translation >);
            let args = (translation,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(914usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_translation", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn remove_translation(&mut self, translation: Gd < crate::engine::Translation >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::Translation >);
            let args = (translation,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(915usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "remove_translation", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_translation_object(&mut self, locale: GString,) -> Option < Gd < crate::engine::Translation > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::Translation > >;
            type CallSig = (Option < Gd < crate::engine::Translation > >, GString);
            let args = (locale,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(916usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_translation_object", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn clear(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(917usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "clear", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_loaded_locales(&self,) -> PackedStringArray {
            type RetMarshal = PtrcallReturnT < PackedStringArray >;
            type CallSig = (PackedStringArray,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(918usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_loaded_locales", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_pseudolocalization_enabled(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(919usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_pseudolocalization_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_pseudolocalization_enabled(&mut self, enabled: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(920usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_pseudolocalization_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn reload_pseudolocalization(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(921usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "reload_pseudolocalization", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn pseudolocalize(&self, message: StringName,) -> StringName {
            type RetMarshal = PtrcallReturnT < StringName >;
            type CallSig = (StringName, StringName);
            let args = (message,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(922usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "pseudolocalize", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for TranslationServer {
        type Base = crate::engine::Object;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"TranslationServer\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Servers;
        
    }
    unsafe impl crate::obj::Bounds for TranslationServer {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for TranslationServer {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::Object > for TranslationServer {
        
    }
    impl std::ops::Deref for TranslationServer {
        type Target = crate::engine::Object;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for TranslationServer {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_TranslationServer {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::TranslationServer > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`TranslationServer::translate_ex`][super::TranslationServer::translate_ex]."]
#[must_use]
pub struct ExTranslate < 'a > {
    surround_object: &'a re_export::TranslationServer, message: StringName, context: StringName,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExTranslate < 'a > {
    fn new(surround_object: &'a re_export::TranslationServer, message: StringName,) -> Self {
        Self {
            surround_object, message, context: StringName::from(""),
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
        re_export::TranslationServer::translate_full(self.surround_object, self.message, self.context,)
    }
}
#[doc = "Default-param extender for [`TranslationServer::translate_plural_ex`][super::TranslationServer::translate_plural_ex]."]
#[must_use]
pub struct ExTranslatePlural < 'a > {
    surround_object: &'a re_export::TranslationServer, message: StringName, plural_message: StringName, n: i32, context: StringName,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExTranslatePlural < 'a > {
    fn new(surround_object: &'a re_export::TranslationServer, message: StringName, plural_message: StringName, n: i32,) -> Self {
        Self {
            surround_object, message, plural_message, n, context: StringName::from(""),
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
        re_export::TranslationServer::translate_plural_full(self.surround_object, self.message, self.plural_message, self.n, self.context,)
    }
}