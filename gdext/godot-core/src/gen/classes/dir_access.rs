#![doc = "Sidecar module for class [`DirAccess`][crate::engine::DirAccess].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `DirAccess` enums](https://docs.godotengine.org/en/stable/classes/class_diraccess.html#enumerations).\n\n"]
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
    #[doc = "Godot class `DirAccess.`\n\nInherits [`RefCounted`][crate::engine::RefCounted].\n\nRelated symbols:\n\n* [`dir_access`][crate::engine::dir_access]: sidecar module with related enum/flag types\n* [`IDirAccess`][crate::engine::IDirAccess]: virtual methods\n\n\nSee also [Godot docs for `DirAccess`](https://docs.godotengine.org/en/stable/classes/class_diraccess.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct DirAccess {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`DirAccess`][crate::engine::DirAccess].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `DirAccess` methods](https://docs.godotengine.org/en/stable/classes/class_diraccess.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IDirAccess: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
    impl DirAccess {
        pub fn open(path: GString,) -> Option < Gd < crate::engine::DirAccess > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::DirAccess > >;
            type CallSig = (Option < Gd < crate::engine::DirAccess > >, GString);
            let args = (path,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2481usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "open", std::ptr::null_mut(), None, args,)
            }
        }
        pub fn get_open_error() -> crate::engine::global::Error {
            type RetMarshal = PtrcallReturnT < crate::engine::global::Error >;
            type CallSig = (crate::engine::global::Error,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2482usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_open_error", std::ptr::null_mut(), None, args,)
            }
        }
        pub fn list_dir_begin(&mut self,) -> crate::engine::global::Error {
            type RetMarshal = PtrcallReturnT < crate::engine::global::Error >;
            type CallSig = (crate::engine::global::Error,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2483usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "list_dir_begin", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_next(&mut self,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2484usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_next", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn current_is_dir(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2485usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "current_is_dir", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn list_dir_end(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2486usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "list_dir_end", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_files(&mut self,) -> PackedStringArray {
            type RetMarshal = PtrcallReturnT < PackedStringArray >;
            type CallSig = (PackedStringArray,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2487usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_files", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_files_at(path: GString,) -> PackedStringArray {
            type RetMarshal = PtrcallReturnT < PackedStringArray >;
            type CallSig = (PackedStringArray, GString);
            let args = (path,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2488usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_files_at", std::ptr::null_mut(), None, args,)
            }
        }
        pub fn get_directories(&mut self,) -> PackedStringArray {
            type RetMarshal = PtrcallReturnT < PackedStringArray >;
            type CallSig = (PackedStringArray,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2489usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_directories", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_directories_at(path: GString,) -> PackedStringArray {
            type RetMarshal = PtrcallReturnT < PackedStringArray >;
            type CallSig = (PackedStringArray, GString);
            let args = (path,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2490usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_directories_at", std::ptr::null_mut(), None, args,)
            }
        }
        pub fn get_drive_count() -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2491usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_drive_count", std::ptr::null_mut(), None, args,)
            }
        }
        pub fn get_drive_name(idx: i32,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString, i32);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2492usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_drive_name", std::ptr::null_mut(), None, args,)
            }
        }
        pub fn get_current_drive(&mut self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2493usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_current_drive", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn change_dir(&mut self, to_dir: GString,) -> crate::engine::global::Error {
            type RetMarshal = PtrcallReturnT < crate::engine::global::Error >;
            type CallSig = (crate::engine::global::Error, GString);
            let args = (to_dir,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2494usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "change_dir", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn get_current_dir_full(&self, include_drive: bool,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString, bool);
            let args = (include_drive,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2495usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_current_dir", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn get_current_dir(&self,) -> GString {
            self.get_current_dir_ex() . done()
        }
        #[inline]
        pub fn get_current_dir_ex(&self,) -> ExGetCurrentDir < '_ > {
            ExGetCurrentDir::new(self,)
        }
        pub fn make_dir(&mut self, path: GString,) -> crate::engine::global::Error {
            type RetMarshal = PtrcallReturnT < crate::engine::global::Error >;
            type CallSig = (crate::engine::global::Error, GString);
            let args = (path,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2496usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "make_dir", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn make_dir_absolute(path: GString,) -> crate::engine::global::Error {
            type RetMarshal = PtrcallReturnT < crate::engine::global::Error >;
            type CallSig = (crate::engine::global::Error, GString);
            let args = (path,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2497usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "make_dir_absolute", std::ptr::null_mut(), None, args,)
            }
        }
        pub fn make_dir_recursive(&mut self, path: GString,) -> crate::engine::global::Error {
            type RetMarshal = PtrcallReturnT < crate::engine::global::Error >;
            type CallSig = (crate::engine::global::Error, GString);
            let args = (path,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2498usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "make_dir_recursive", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn make_dir_recursive_absolute(path: GString,) -> crate::engine::global::Error {
            type RetMarshal = PtrcallReturnT < crate::engine::global::Error >;
            type CallSig = (crate::engine::global::Error, GString);
            let args = (path,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2499usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "make_dir_recursive_absolute", std::ptr::null_mut(), None, args,)
            }
        }
        pub fn file_exists(&mut self, path: GString,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, GString);
            let args = (path,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2500usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "file_exists", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn dir_exists(&mut self, path: GString,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, GString);
            let args = (path,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2501usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "dir_exists", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn dir_exists_absolute(path: GString,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, GString);
            let args = (path,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2502usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "dir_exists_absolute", std::ptr::null_mut(), None, args,)
            }
        }
        pub fn get_space_left(&mut self,) -> u64 {
            type RetMarshal = PtrcallReturnT < u64 >;
            type CallSig = (u64,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2503usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_space_left", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn copy_full(&mut self, from: GString, to: GString, chmod_flags: i32,) -> crate::engine::global::Error {
            type RetMarshal = PtrcallReturnT < crate::engine::global::Error >;
            type CallSig = (crate::engine::global::Error, GString, GString, i32);
            let args = (from, to, chmod_flags,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2504usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "copy", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn copy(&mut self, from: GString, to: GString,) -> crate::engine::global::Error {
            self.copy_ex(from, to,) . done()
        }
        #[inline]
        pub fn copy_ex(&mut self, from: GString, to: GString,) -> ExCopy < '_ > {
            ExCopy::new(self, from, to,)
        }
        pub(crate) fn copy_absolute_full(from: GString, to: GString, chmod_flags: i32,) -> crate::engine::global::Error {
            type RetMarshal = PtrcallReturnT < crate::engine::global::Error >;
            type CallSig = (crate::engine::global::Error, GString, GString, i32);
            let args = (from, to, chmod_flags,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2505usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "copy_absolute", std::ptr::null_mut(), None, args,)
            }
        }
        #[inline]
        pub fn copy_absolute(from: GString, to: GString,) -> crate::engine::global::Error {
            Self::copy_absolute_ex(from, to,) . done()
        }
        #[inline]
        pub fn copy_absolute_ex(from: GString, to: GString,) -> ExCopyAbsolute {
            ExCopyAbsolute::new(from, to,)
        }
        pub fn rename(&mut self, from: GString, to: GString,) -> crate::engine::global::Error {
            type RetMarshal = PtrcallReturnT < crate::engine::global::Error >;
            type CallSig = (crate::engine::global::Error, GString, GString);
            let args = (from, to,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2506usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "rename", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn rename_absolute(from: GString, to: GString,) -> crate::engine::global::Error {
            type RetMarshal = PtrcallReturnT < crate::engine::global::Error >;
            type CallSig = (crate::engine::global::Error, GString, GString);
            let args = (from, to,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2507usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "rename_absolute", std::ptr::null_mut(), None, args,)
            }
        }
        pub fn remove(&mut self, path: GString,) -> crate::engine::global::Error {
            type RetMarshal = PtrcallReturnT < crate::engine::global::Error >;
            type CallSig = (crate::engine::global::Error, GString);
            let args = (path,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2508usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "remove", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn remove_absolute(path: GString,) -> crate::engine::global::Error {
            type RetMarshal = PtrcallReturnT < crate::engine::global::Error >;
            type CallSig = (crate::engine::global::Error, GString);
            let args = (path,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2509usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "remove_absolute", std::ptr::null_mut(), None, args,)
            }
        }
        pub fn set_include_navigational(&mut self, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2510usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_include_navigational", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_include_navigational(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2511usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_include_navigational", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_include_hidden(&mut self, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2512usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_include_hidden", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_include_hidden(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2513usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_include_hidden", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_case_sensitive(&self, path: GString,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, GString);
            let args = (path,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2514usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_case_sensitive", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for DirAccess {
        type Base = crate::engine::RefCounted;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"DirAccess\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for DirAccess {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for DirAccess {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::RefCounted > for DirAccess {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for DirAccess {
        
    }
    impl std::ops::Deref for DirAccess {
        type Target = crate::engine::RefCounted;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for DirAccess {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_DirAccess {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::DirAccess > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::RefCounted > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`DirAccess::get_current_dir_ex`][super::DirAccess::get_current_dir_ex]."]
#[must_use]
pub struct ExGetCurrentDir < 'a > {
    surround_object: &'a re_export::DirAccess, include_drive: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetCurrentDir < 'a > {
    fn new(surround_object: &'a re_export::DirAccess,) -> Self {
        Self {
            surround_object, include_drive: true,
        }
    }
    #[inline]
    pub fn include_drive(self, value: bool) -> Self {
        Self {
            include_drive: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> GString {
        re_export::DirAccess::get_current_dir_full(self.surround_object, self.include_drive,)
    }
}
#[doc = "Default-param extender for [`DirAccess::copy_ex`][super::DirAccess::copy_ex]."]
#[must_use]
pub struct ExCopy < 'a > {
    surround_object: &'a mut re_export::DirAccess, from: GString, to: GString, chmod_flags: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExCopy < 'a > {
    fn new(surround_object: &'a mut re_export::DirAccess, from: GString, to: GString,) -> Self {
        Self {
            surround_object, from, to, chmod_flags: - 1i32,
        }
    }
    #[inline]
    pub fn chmod_flags(self, value: i32) -> Self {
        Self {
            chmod_flags: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::engine::global::Error {
        re_export::DirAccess::copy_full(self.surround_object, self.from, self.to, self.chmod_flags,)
    }
}
#[doc = "Default-param extender for [`DirAccess::copy_absolute_ex`][super::DirAccess::copy_absolute_ex]."]
#[must_use]
pub struct ExCopyAbsolute {
    from: GString, to: GString, chmod_flags: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl ExCopyAbsolute {
    fn new(from: GString, to: GString,) -> Self {
        Self {
            from, to, chmod_flags: - 1i32,
        }
    }
    #[inline]
    pub fn chmod_flags(self, value: i32) -> Self {
        Self {
            chmod_flags: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::engine::global::Error {
        re_export::DirAccess::copy_absolute_full(self.from, self.to, self.chmod_flags,)
    }
}