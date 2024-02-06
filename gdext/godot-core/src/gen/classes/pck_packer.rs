#![doc = "Sidecar module for class [`PckPacker`][crate::engine::PckPacker].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `PCKPacker` enums](https://docs.godotengine.org/en/stable/classes/class_pckpacker.html#enumerations).\n\n"]
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
    #[doc = "Godot class `PCKPacker.`\n\nInherits [`RefCounted`][crate::engine::RefCounted].\n\nRelated symbols:\n\n* [`pck_packer`][crate::engine::pck_packer]: sidecar module with related enum/flag types\n* [`IPckPacker`][crate::engine::IPckPacker]: virtual methods\n\n\nSee also [Godot docs for `PCKPacker`](https://docs.godotengine.org/en/stable/classes/class_pckpacker.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct PckPacker {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`PckPacker`][crate::engine::PckPacker].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `PCKPacker` methods](https://docs.godotengine.org/en/stable/classes/class_pckpacker.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IPckPacker: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
    impl PckPacker {
        #[deprecated = "Replaced with `new_gd` in extension trait `NewGd`."]
        pub fn new() -> Gd < Self > {
            crate::obj::Gd::default()
        }
        pub(crate) fn pck_start_full(&mut self, pck_name: GString, alignment: i32, key: GString, encrypt_directory: bool,) -> crate::engine::global::Error {
            type RetMarshal = PtrcallReturnT < crate::engine::global::Error >;
            type CallSig = (crate::engine::global::Error, GString, i32, GString, bool);
            let args = (pck_name, alignment, key, encrypt_directory,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5401usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "pck_start", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn pck_start(&mut self, pck_name: GString,) -> crate::engine::global::Error {
            self.pck_start_ex(pck_name,) . done()
        }
        #[inline]
        pub fn pck_start_ex(&mut self, pck_name: GString,) -> ExPckStart < '_ > {
            ExPckStart::new(self, pck_name,)
        }
        pub(crate) fn add_file_full(&mut self, pck_path: GString, source_path: GString, encrypt: bool,) -> crate::engine::global::Error {
            type RetMarshal = PtrcallReturnT < crate::engine::global::Error >;
            type CallSig = (crate::engine::global::Error, GString, GString, bool);
            let args = (pck_path, source_path, encrypt,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5402usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_file", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn add_file(&mut self, pck_path: GString, source_path: GString,) -> crate::engine::global::Error {
            self.add_file_ex(pck_path, source_path,) . done()
        }
        #[inline]
        pub fn add_file_ex(&mut self, pck_path: GString, source_path: GString,) -> ExAddFile < '_ > {
            ExAddFile::new(self, pck_path, source_path,)
        }
        pub(crate) fn flush_full(&mut self, verbose: bool,) -> crate::engine::global::Error {
            type RetMarshal = PtrcallReturnT < crate::engine::global::Error >;
            type CallSig = (crate::engine::global::Error, bool);
            let args = (verbose,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5403usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "flush", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn flush(&mut self,) -> crate::engine::global::Error {
            self.flush_ex() . done()
        }
        #[inline]
        pub fn flush_ex(&mut self,) -> ExFlush < '_ > {
            ExFlush::new(self,)
        }
        fn __checked_id(&self) -> Option < crate::obj::InstanceId > {
            let rtti = unsafe {
                self.rtti.as_ref() . unwrap_unchecked()
            };
            let instance_id = rtti.check_type::< Self > ();
            Some(instance_id)
        }
    }
    impl crate::obj::GodotClass for PckPacker {
        type Base = crate::engine::RefCounted;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"PCKPacker\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for PckPacker {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for PckPacker {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::RefCounted > for PckPacker {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for PckPacker {
        
    }
    impl crate::obj::cap::GodotDefault for PckPacker {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for PckPacker {
        type Target = crate::engine::RefCounted;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for PckPacker {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_PckPacker {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::PckPacker > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::RefCounted > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`PckPacker::pck_start_ex`][super::PckPacker::pck_start_ex]."]
#[must_use]
pub struct ExPckStart < 'a > {
    surround_object: &'a mut re_export::PckPacker, pck_name: GString, alignment: i32, key: GString, encrypt_directory: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExPckStart < 'a > {
    fn new(surround_object: &'a mut re_export::PckPacker, pck_name: GString,) -> Self {
        Self {
            surround_object, pck_name, alignment: 32i32, key: GString::from("0000000000000000000000000000000000000000000000000000000000000000"), encrypt_directory: false,
        }
    }
    #[inline]
    pub fn alignment(self, value: i32) -> Self {
        Self {
            alignment: value, .. self
        }
    }
    #[inline]
    pub fn key(self, value: GString) -> Self {
        Self {
            key: value, .. self
        }
    }
    #[inline]
    pub fn encrypt_directory(self, value: bool) -> Self {
        Self {
            encrypt_directory: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::engine::global::Error {
        re_export::PckPacker::pck_start_full(self.surround_object, self.pck_name, self.alignment, self.key, self.encrypt_directory,)
    }
}
#[doc = "Default-param extender for [`PckPacker::add_file_ex`][super::PckPacker::add_file_ex]."]
#[must_use]
pub struct ExAddFile < 'a > {
    surround_object: &'a mut re_export::PckPacker, pck_path: GString, source_path: GString, encrypt: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddFile < 'a > {
    fn new(surround_object: &'a mut re_export::PckPacker, pck_path: GString, source_path: GString,) -> Self {
        Self {
            surround_object, pck_path, source_path, encrypt: false,
        }
    }
    #[inline]
    pub fn encrypt(self, value: bool) -> Self {
        Self {
            encrypt: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::engine::global::Error {
        re_export::PckPacker::add_file_full(self.surround_object, self.pck_path, self.source_path, self.encrypt,)
    }
}
#[doc = "Default-param extender for [`PckPacker::flush_ex`][super::PckPacker::flush_ex]."]
#[must_use]
pub struct ExFlush < 'a > {
    surround_object: &'a mut re_export::PckPacker, verbose: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExFlush < 'a > {
    fn new(surround_object: &'a mut re_export::PckPacker,) -> Self {
        Self {
            surround_object, verbose: false,
        }
    }
    #[inline]
    pub fn verbose(self, value: bool) -> Self {
        Self {
            verbose: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::engine::global::Error {
        re_export::PckPacker::flush_full(self.surround_object, self.verbose,)
    }
}