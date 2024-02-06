#![doc = "Sidecar module for class [`Json`][crate::engine::Json].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `JSON` enums](https://docs.godotengine.org/en/stable/classes/class_json.html#enumerations).\n\n"]
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
    #[doc = "Godot class `JSON.`\n\nInherits [`Resource`][crate::engine::Resource].\n\nRelated symbols:\n\n* [`json`][crate::engine::json]: sidecar module with related enum/flag types\n* [`IJson`][crate::engine::IJson]: virtual methods\n\n\nSee also [Godot docs for `JSON`](https://docs.godotengine.org/en/stable/classes/class_json.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct Json {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`Json`][crate::engine::Json].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `JSON` methods](https://docs.godotengine.org/en/stable/classes/class_json.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IJson: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
    impl Json {
        #[deprecated = "Replaced with `new_gd` in extension trait `NewGd`."]
        pub fn new() -> Gd < Self > {
            crate::obj::Gd::default()
        }
        pub(crate) fn stringify_full(data: Variant, indent: GString, sort_keys: bool, full_precision: bool,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString, Variant, GString, bool, bool);
            let args = (data, indent, sort_keys, full_precision,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4161usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "stringify", std::ptr::null_mut(), None, args,)
            }
        }
        #[inline]
        pub fn stringify(data: Variant,) -> GString {
            Self::stringify_ex(data,) . done()
        }
        #[inline]
        pub fn stringify_ex(data: Variant,) -> ExStringify {
            ExStringify::new(data,)
        }
        pub fn parse_string(json_string: GString,) -> Variant {
            type RetMarshal = PtrcallReturnT < Variant >;
            type CallSig = (Variant, GString);
            let args = (json_string,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4162usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "parse_string", std::ptr::null_mut(), None, args,)
            }
        }
        pub(crate) fn parse_full(&mut self, json_text: GString, keep_text: bool,) -> crate::engine::global::Error {
            type RetMarshal = PtrcallReturnT < crate::engine::global::Error >;
            type CallSig = (crate::engine::global::Error, GString, bool);
            let args = (json_text, keep_text,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4163usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "parse", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn parse(&mut self, json_text: GString,) -> crate::engine::global::Error {
            self.parse_ex(json_text,) . done()
        }
        #[inline]
        pub fn parse_ex(&mut self, json_text: GString,) -> ExParse < '_ > {
            ExParse::new(self, json_text,)
        }
        pub fn get_data(&self,) -> Variant {
            type RetMarshal = PtrcallReturnT < Variant >;
            type CallSig = (Variant,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4164usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_data", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_data(&mut self, data: Variant,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Variant);
            let args = (data,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4165usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_data", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_parsed_text(&self,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4166usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_parsed_text", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_error_line(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4167usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_error_line", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_error_message(&self,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4168usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_error_message", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for Json {
        type Base = crate::engine::Resource;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"JSON\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for Json {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for Json {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::Resource > for Json {
        
    }
    impl crate::obj::Inherits < crate::engine::RefCounted > for Json {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for Json {
        
    }
    impl crate::obj::ExportableObject for Json {
        
    }
    impl crate::obj::cap::GodotDefault for Json {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for Json {
        type Target = crate::engine::Resource;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for Json {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_Json {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::Json > for $Class {
                
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
#[doc = "Default-param extender for [`Json::stringify_ex`][super::Json::stringify_ex]."]
#[must_use]
pub struct ExStringify {
    data: Variant, indent: GString, sort_keys: bool, full_precision: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl ExStringify {
    fn new(data: Variant,) -> Self {
        Self {
            data, indent: GString::from(""), sort_keys: true, full_precision: false,
        }
    }
    #[inline]
    pub fn indent(self, value: GString) -> Self {
        Self {
            indent: value, .. self
        }
    }
    #[inline]
    pub fn sort_keys(self, value: bool) -> Self {
        Self {
            sort_keys: value, .. self
        }
    }
    #[inline]
    pub fn full_precision(self, value: bool) -> Self {
        Self {
            full_precision: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> GString {
        re_export::Json::stringify_full(self.data, self.indent, self.sort_keys, self.full_precision,)
    }
}
#[doc = "Default-param extender for [`Json::parse_ex`][super::Json::parse_ex]."]
#[must_use]
pub struct ExParse < 'a > {
    surround_object: &'a mut re_export::Json, json_text: GString, keep_text: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExParse < 'a > {
    fn new(surround_object: &'a mut re_export::Json, json_text: GString,) -> Self {
        Self {
            surround_object, json_text, keep_text: false,
        }
    }
    #[inline]
    pub fn keep_text(self, value: bool) -> Self {
        Self {
            keep_text: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::engine::global::Error {
        re_export::Json::parse_full(self.surround_object, self.json_text, self.keep_text,)
    }
}