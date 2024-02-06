#![doc = "Sidecar module for class [`ConfigFile`][crate::engine::ConfigFile].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `ConfigFile` enums](https://docs.godotengine.org/en/stable/classes/class_configfile.html#enumerations).\n\n"]
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
    #[doc = "Godot class `ConfigFile.`\n\nInherits [`RefCounted`][crate::engine::RefCounted].\n\nRelated symbols:\n\n* [`config_file`][crate::engine::config_file]: sidecar module with related enum/flag types\n* [`IConfigFile`][crate::engine::IConfigFile]: virtual methods\n\n\nSee also [Godot docs for `ConfigFile`](https://docs.godotengine.org/en/stable/classes/class_configfile.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct ConfigFile {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`ConfigFile`][crate::engine::ConfigFile].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `ConfigFile` methods](https://docs.godotengine.org/en/stable/classes/class_configfile.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IConfigFile: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
    impl ConfigFile {
        #[deprecated = "Replaced with `new_gd` in extension trait `NewGd`."]
        pub fn new() -> Gd < Self > {
            crate::obj::Gd::default()
        }
        pub fn set_value(&mut self, section: GString, key: GString, value: Variant,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString, GString, Variant);
            let args = (section, key, value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2169usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_value", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn get_value_full(&self, section: GString, key: GString, default: Variant,) -> Variant {
            type RetMarshal = PtrcallReturnT < Variant >;
            type CallSig = (Variant, GString, GString, Variant);
            let args = (section, key, default,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2170usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_value", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn get_value(&self, section: GString, key: GString,) -> Variant {
            self.get_value_ex(section, key,) . done()
        }
        #[inline]
        pub fn get_value_ex(&self, section: GString, key: GString,) -> ExGetValue < '_ > {
            ExGetValue::new(self, section, key,)
        }
        pub fn has_section(&self, section: GString,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, GString);
            let args = (section,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2171usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "has_section", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn has_section_key(&self, section: GString, key: GString,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, GString, GString);
            let args = (section, key,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2172usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "has_section_key", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_sections(&self,) -> PackedStringArray {
            type RetMarshal = PtrcallReturnT < PackedStringArray >;
            type CallSig = (PackedStringArray,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2173usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_sections", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_section_keys(&self, section: GString,) -> PackedStringArray {
            type RetMarshal = PtrcallReturnT < PackedStringArray >;
            type CallSig = (PackedStringArray, GString);
            let args = (section,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2174usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_section_keys", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn erase_section(&mut self, section: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString);
            let args = (section,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2175usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "erase_section", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn erase_section_key(&mut self, section: GString, key: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString, GString);
            let args = (section, key,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2176usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "erase_section_key", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn load(&mut self, path: GString,) -> crate::engine::global::Error {
            type RetMarshal = PtrcallReturnT < crate::engine::global::Error >;
            type CallSig = (crate::engine::global::Error, GString);
            let args = (path,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2177usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "load", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn parse(&mut self, data: GString,) -> crate::engine::global::Error {
            type RetMarshal = PtrcallReturnT < crate::engine::global::Error >;
            type CallSig = (crate::engine::global::Error, GString);
            let args = (data,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2178usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "parse", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn save(&mut self, path: GString,) -> crate::engine::global::Error {
            type RetMarshal = PtrcallReturnT < crate::engine::global::Error >;
            type CallSig = (crate::engine::global::Error, GString);
            let args = (path,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2179usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "save", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn encode_to_text(&self,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2180usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "encode_to_text", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn load_encrypted(&mut self, path: GString, key: PackedByteArray,) -> crate::engine::global::Error {
            type RetMarshal = PtrcallReturnT < crate::engine::global::Error >;
            type CallSig = (crate::engine::global::Error, GString, PackedByteArray);
            let args = (path, key,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2181usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "load_encrypted", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn load_encrypted_pass(&mut self, path: GString, password: GString,) -> crate::engine::global::Error {
            type RetMarshal = PtrcallReturnT < crate::engine::global::Error >;
            type CallSig = (crate::engine::global::Error, GString, GString);
            let args = (path, password,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2182usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "load_encrypted_pass", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn save_encrypted(&mut self, path: GString, key: PackedByteArray,) -> crate::engine::global::Error {
            type RetMarshal = PtrcallReturnT < crate::engine::global::Error >;
            type CallSig = (crate::engine::global::Error, GString, PackedByteArray);
            let args = (path, key,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2183usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "save_encrypted", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn save_encrypted_pass(&mut self, path: GString, password: GString,) -> crate::engine::global::Error {
            type RetMarshal = PtrcallReturnT < crate::engine::global::Error >;
            type CallSig = (crate::engine::global::Error, GString, GString);
            let args = (path, password,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2184usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "save_encrypted_pass", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn clear(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2185usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "clear", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for ConfigFile {
        type Base = crate::engine::RefCounted;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"ConfigFile\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for ConfigFile {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for ConfigFile {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::RefCounted > for ConfigFile {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for ConfigFile {
        
    }
    impl crate::obj::cap::GodotDefault for ConfigFile {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for ConfigFile {
        type Target = crate::engine::RefCounted;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for ConfigFile {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_ConfigFile {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::ConfigFile > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::RefCounted > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`ConfigFile::get_value_ex`][super::ConfigFile::get_value_ex]."]
#[must_use]
pub struct ExGetValue < 'a > {
    surround_object: &'a re_export::ConfigFile, section: GString, key: GString, default: Variant,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetValue < 'a > {
    fn new(surround_object: &'a re_export::ConfigFile, section: GString, key: GString,) -> Self {
        Self {
            surround_object, section, key, default: Variant::nil(),
        }
    }
    #[inline]
    pub fn default(self, value: Variant) -> Self {
        Self {
            default: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Variant {
        re_export::ConfigFile::get_value_full(self.surround_object, self.section, self.key, self.default,)
    }
}