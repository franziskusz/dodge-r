#![doc = "Sidecar module for class [`CryptoKey`][crate::engine::CryptoKey].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `CryptoKey` enums](https://docs.godotengine.org/en/stable/classes/class_cryptokey.html#enumerations).\n\n"]
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
    #[doc = "Godot class `CryptoKey.`\n\nInherits [`Resource`][crate::engine::Resource].\n\nRelated symbols:\n\n* [`crypto_key`][crate::engine::crypto_key]: sidecar module with related enum/flag types\n* [`ICryptoKey`][crate::engine::ICryptoKey]: virtual methods\n\n\nSee also [Godot docs for `CryptoKey`](https://docs.godotengine.org/en/stable/classes/class_cryptokey.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct CryptoKey {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`CryptoKey`][crate::engine::CryptoKey].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `CryptoKey` methods](https://docs.godotengine.org/en/stable/classes/class_cryptokey.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ICryptoKey: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
        fn setup_local_to_scene(&mut self,) {
            unimplemented !()
        }
    }
    impl CryptoKey {
        #[deprecated = "Replaced with `new_gd` in extension trait `NewGd`."]
        pub fn new() -> Gd < Self > {
            crate::obj::Gd::default()
        }
        pub(crate) fn save_full(&mut self, path: GString, public_only: bool,) -> crate::engine::global::Error {
            type RetMarshal = PtrcallReturnT < crate::engine::global::Error >;
            type CallSig = (crate::engine::global::Error, GString, bool);
            let args = (path, public_only,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2333usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "save", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn save(&mut self, path: GString,) -> crate::engine::global::Error {
            self.save_ex(path,) . done()
        }
        #[inline]
        pub fn save_ex(&mut self, path: GString,) -> ExSave < '_ > {
            ExSave::new(self, path,)
        }
        pub(crate) fn load_full(&mut self, path: GString, public_only: bool,) -> crate::engine::global::Error {
            type RetMarshal = PtrcallReturnT < crate::engine::global::Error >;
            type CallSig = (crate::engine::global::Error, GString, bool);
            let args = (path, public_only,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2334usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "load", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn load(&mut self, path: GString,) -> crate::engine::global::Error {
            self.load_ex(path,) . done()
        }
        #[inline]
        pub fn load_ex(&mut self, path: GString,) -> ExLoad < '_ > {
            ExLoad::new(self, path,)
        }
        pub fn is_public_only(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2335usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_public_only", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn save_to_string_full(&mut self, public_only: bool,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString, bool);
            let args = (public_only,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2336usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "save_to_string", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn save_to_string(&mut self,) -> GString {
            self.save_to_string_ex() . done()
        }
        #[inline]
        pub fn save_to_string_ex(&mut self,) -> ExSaveToString < '_ > {
            ExSaveToString::new(self,)
        }
        pub(crate) fn load_from_string_full(&mut self, string_key: GString, public_only: bool,) -> crate::engine::global::Error {
            type RetMarshal = PtrcallReturnT < crate::engine::global::Error >;
            type CallSig = (crate::engine::global::Error, GString, bool);
            let args = (string_key, public_only,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2337usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "load_from_string", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn load_from_string(&mut self, string_key: GString,) -> crate::engine::global::Error {
            self.load_from_string_ex(string_key,) . done()
        }
        #[inline]
        pub fn load_from_string_ex(&mut self, string_key: GString,) -> ExLoadFromString < '_ > {
            ExLoadFromString::new(self, string_key,)
        }
        fn __checked_id(&self) -> Option < crate::obj::InstanceId > {
            let rtti = unsafe {
                self.rtti.as_ref() . unwrap_unchecked()
            };
            let instance_id = rtti.check_type::< Self > ();
            Some(instance_id)
        }
    }
    impl crate::obj::GodotClass for CryptoKey {
        type Base = crate::engine::Resource;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"CryptoKey\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for CryptoKey {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for CryptoKey {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::Resource > for CryptoKey {
        
    }
    impl crate::obj::Inherits < crate::engine::RefCounted > for CryptoKey {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for CryptoKey {
        
    }
    impl crate::obj::ExportableObject for CryptoKey {
        
    }
    impl crate::obj::cap::GodotDefault for CryptoKey {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for CryptoKey {
        type Target = crate::engine::Resource;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for CryptoKey {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_CryptoKey {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::CryptoKey > for $Class {
                
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
#[doc = "Default-param extender for [`CryptoKey::save_ex`][super::CryptoKey::save_ex]."]
#[must_use]
pub struct ExSave < 'a > {
    surround_object: &'a mut re_export::CryptoKey, path: GString, public_only: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExSave < 'a > {
    fn new(surround_object: &'a mut re_export::CryptoKey, path: GString,) -> Self {
        Self {
            surround_object, path, public_only: false,
        }
    }
    #[inline]
    pub fn public_only(self, value: bool) -> Self {
        Self {
            public_only: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::engine::global::Error {
        re_export::CryptoKey::save_full(self.surround_object, self.path, self.public_only,)
    }
}
#[doc = "Default-param extender for [`CryptoKey::load_ex`][super::CryptoKey::load_ex]."]
#[must_use]
pub struct ExLoad < 'a > {
    surround_object: &'a mut re_export::CryptoKey, path: GString, public_only: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExLoad < 'a > {
    fn new(surround_object: &'a mut re_export::CryptoKey, path: GString,) -> Self {
        Self {
            surround_object, path, public_only: false,
        }
    }
    #[inline]
    pub fn public_only(self, value: bool) -> Self {
        Self {
            public_only: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::engine::global::Error {
        re_export::CryptoKey::load_full(self.surround_object, self.path, self.public_only,)
    }
}
#[doc = "Default-param extender for [`CryptoKey::save_to_string_ex`][super::CryptoKey::save_to_string_ex]."]
#[must_use]
pub struct ExSaveToString < 'a > {
    surround_object: &'a mut re_export::CryptoKey, public_only: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExSaveToString < 'a > {
    fn new(surround_object: &'a mut re_export::CryptoKey,) -> Self {
        Self {
            surround_object, public_only: false,
        }
    }
    #[inline]
    pub fn public_only(self, value: bool) -> Self {
        Self {
            public_only: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> GString {
        re_export::CryptoKey::save_to_string_full(self.surround_object, self.public_only,)
    }
}
#[doc = "Default-param extender for [`CryptoKey::load_from_string_ex`][super::CryptoKey::load_from_string_ex]."]
#[must_use]
pub struct ExLoadFromString < 'a > {
    surround_object: &'a mut re_export::CryptoKey, string_key: GString, public_only: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExLoadFromString < 'a > {
    fn new(surround_object: &'a mut re_export::CryptoKey, string_key: GString,) -> Self {
        Self {
            surround_object, string_key, public_only: false,
        }
    }
    #[inline]
    pub fn public_only(self, value: bool) -> Self {
        Self {
            public_only: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::engine::global::Error {
        re_export::CryptoKey::load_from_string_full(self.surround_object, self.string_key, self.public_only,)
    }
}