#![doc = "Sidecar module for class [`ZipReader`][crate::engine::ZipReader].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `ZIPReader` enums](https://docs.godotengine.org/en/stable/classes/class_zipreader.html#enumerations).\n\n"]
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
    #[doc = "Godot class `ZIPReader.`\n\nInherits [`RefCounted`][crate::engine::RefCounted].\n\nRelated symbols:\n\n* [`zip_reader`][crate::engine::zip_reader]: sidecar module with related enum/flag types\n* [`IZipReader`][crate::engine::IZipReader]: virtual methods\n\n\nSee also [Godot docs for `ZIPReader`](https://docs.godotengine.org/en/stable/classes/class_zipreader.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct ZipReader {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`ZipReader`][crate::engine::ZipReader].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `ZIPReader` methods](https://docs.godotengine.org/en/stable/classes/class_zipreader.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IZipReader: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
    impl ZipReader {
        #[deprecated = "Replaced with `new_gd` in extension trait `NewGd`."]
        pub fn new() -> Gd < Self > {
            crate::obj::Gd::default()
        }
        pub fn open(&mut self, path: GString,) -> crate::engine::global::Error {
            type RetMarshal = PtrcallReturnT < crate::engine::global::Error >;
            type CallSig = (crate::engine::global::Error, GString);
            let args = (path,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10053usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "open", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn close(&mut self,) -> crate::engine::global::Error {
            type RetMarshal = PtrcallReturnT < crate::engine::global::Error >;
            type CallSig = (crate::engine::global::Error,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10054usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "close", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_files(&mut self,) -> PackedStringArray {
            type RetMarshal = PtrcallReturnT < PackedStringArray >;
            type CallSig = (PackedStringArray,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10055usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_files", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn read_file_full(&mut self, path: GString, case_sensitive: bool,) -> PackedByteArray {
            type RetMarshal = PtrcallReturnT < PackedByteArray >;
            type CallSig = (PackedByteArray, GString, bool);
            let args = (path, case_sensitive,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10056usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "read_file", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn read_file(&mut self, path: GString,) -> PackedByteArray {
            self.read_file_ex(path,) . done()
        }
        #[inline]
        pub fn read_file_ex(&mut self, path: GString,) -> ExReadFile < '_ > {
            ExReadFile::new(self, path,)
        }
        pub(crate) fn file_exists_full(&mut self, path: GString, case_sensitive: bool,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, GString, bool);
            let args = (path, case_sensitive,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10057usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "file_exists", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn file_exists(&mut self, path: GString,) -> bool {
            self.file_exists_ex(path,) . done()
        }
        #[inline]
        pub fn file_exists_ex(&mut self, path: GString,) -> ExFileExists < '_ > {
            ExFileExists::new(self, path,)
        }
        fn __checked_id(&self) -> Option < crate::obj::InstanceId > {
            let rtti = unsafe {
                self.rtti.as_ref() . unwrap_unchecked()
            };
            let instance_id = rtti.check_type::< Self > ();
            Some(instance_id)
        }
    }
    impl crate::obj::GodotClass for ZipReader {
        type Base = crate::engine::RefCounted;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"ZIPReader\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for ZipReader {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for ZipReader {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::RefCounted > for ZipReader {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for ZipReader {
        
    }
    impl crate::obj::cap::GodotDefault for ZipReader {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for ZipReader {
        type Target = crate::engine::RefCounted;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for ZipReader {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_ZipReader {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::ZipReader > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::RefCounted > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`ZipReader::read_file_ex`][super::ZipReader::read_file_ex]."]
#[must_use]
pub struct ExReadFile < 'a > {
    surround_object: &'a mut re_export::ZipReader, path: GString, case_sensitive: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExReadFile < 'a > {
    fn new(surround_object: &'a mut re_export::ZipReader, path: GString,) -> Self {
        Self {
            surround_object, path, case_sensitive: true,
        }
    }
    #[inline]
    pub fn case_sensitive(self, value: bool) -> Self {
        Self {
            case_sensitive: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> PackedByteArray {
        re_export::ZipReader::read_file_full(self.surround_object, self.path, self.case_sensitive,)
    }
}
#[doc = "Default-param extender for [`ZipReader::file_exists_ex`][super::ZipReader::file_exists_ex]."]
#[must_use]
pub struct ExFileExists < 'a > {
    surround_object: &'a mut re_export::ZipReader, path: GString, case_sensitive: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExFileExists < 'a > {
    fn new(surround_object: &'a mut re_export::ZipReader, path: GString,) -> Self {
        Self {
            surround_object, path, case_sensitive: true,
        }
    }
    #[inline]
    pub fn case_sensitive(self, value: bool) -> Self {
        Self {
            case_sensitive: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> bool {
        re_export::ZipReader::file_exists_full(self.surround_object, self.path, self.case_sensitive,)
    }
}