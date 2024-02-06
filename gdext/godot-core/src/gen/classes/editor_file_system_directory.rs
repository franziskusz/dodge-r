#![doc = "Sidecar module for class [`EditorFileSystemDirectory`][crate::engine::EditorFileSystemDirectory].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `EditorFileSystemDirectory` enums](https://docs.godotengine.org/en/stable/classes/class_editorfilesystemdirectory.html#enumerations).\n\n"]
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
    #[doc = "Godot class `EditorFileSystemDirectory.`\n\nInherits [`Object`][crate::engine::Object].\n\nRelated symbols:\n\n* [`IEditorFileSystemDirectory`][crate::engine::IEditorFileSystemDirectory]: virtual methods\n\n\nSee also [Godot docs for `EditorFileSystemDirectory`](https://docs.godotengine.org/en/stable/classes/class_editorfilesystemdirectory.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct EditorFileSystemDirectory {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`EditorFileSystemDirectory`][crate::engine::EditorFileSystemDirectory].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `EditorFileSystemDirectory` methods](https://docs.godotengine.org/en/stable/classes/class_editorfilesystemdirectory.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IEditorFileSystemDirectory: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
    impl EditorFileSystemDirectory {
        pub fn get_subdir_count(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(68usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_subdir_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_subdir(&mut self, idx: i32,) -> Option < Gd < crate::engine::EditorFileSystemDirectory > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::EditorFileSystemDirectory > >;
            type CallSig = (Option < Gd < crate::engine::EditorFileSystemDirectory > >, i32);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(69usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_subdir", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_file_count(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(70usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_file_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_file(&self, idx: i32,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString, i32);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(71usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_file", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_file_path(&self, idx: i32,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString, i32);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(72usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_file_path", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_file_type(&self, idx: i32,) -> StringName {
            type RetMarshal = PtrcallReturnT < StringName >;
            type CallSig = (StringName, i32);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(73usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_file_type", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_file_script_class_name(&self, idx: i32,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString, i32);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(74usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_file_script_class_name", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_file_script_class_extends(&self, idx: i32,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString, i32);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(75usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_file_script_class_extends", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_file_import_is_valid(&self, idx: i32,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, i32);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(76usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_file_import_is_valid", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_name(&mut self,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(77usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_name", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_path(&self,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(78usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_path", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_parent(&mut self,) -> Option < Gd < crate::engine::EditorFileSystemDirectory > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::EditorFileSystemDirectory > >;
            type CallSig = (Option < Gd < crate::engine::EditorFileSystemDirectory > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(79usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_parent", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn find_file_index(&self, name: GString,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, GString);
            let args = (name,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(80usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "find_file_index", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn find_dir_index(&self, name: GString,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, GString);
            let args = (name,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(81usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "find_dir_index", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for EditorFileSystemDirectory {
        type Base = crate::engine::Object;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"EditorFileSystemDirectory\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Editor;
        
    }
    unsafe impl crate::obj::Bounds for EditorFileSystemDirectory {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for EditorFileSystemDirectory {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::Object > for EditorFileSystemDirectory {
        
    }
    impl crate::obj::cap::GodotDefault for EditorFileSystemDirectory {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for EditorFileSystemDirectory {
        type Target = crate::engine::Object;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for EditorFileSystemDirectory {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_EditorFileSystemDirectory {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::EditorFileSystemDirectory > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::Object > for $Class {
                
            }
        }
    }
}