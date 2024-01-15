#![doc = "Sidecar module for class [`RdShaderFile`][crate::engine::RdShaderFile].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `RDShaderFile` enums](https://docs.godotengine.org/en/stable/classes/class_rdshaderfile.html#enumerations).\n\n"]
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
    #[doc = "Godot class `RDShaderFile.`\n\nInherits [`Resource`][crate::engine::Resource].\n\nRelated symbols:\n\n* [`rd_shader_file`][crate::engine::rd_shader_file]: sidecar module with related enum/flag types\n* [`IRdShaderFile`][crate::engine::IRdShaderFile]: virtual methods\n\n\nSee also [Godot docs for `RDShaderFile`](https://docs.godotengine.org/en/stable/classes/class_rdshaderfile.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct RdShaderFile {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`RdShaderFile`][crate::engine::RdShaderFile].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `RDShaderFile` methods](https://docs.godotengine.org/en/stable/classes/class_rdshaderfile.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IRdShaderFile: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
    impl RdShaderFile {
        #[deprecated = "Replaced with `new_gd` in extension trait `NewGd`."]
        pub fn new() -> Gd < Self > {
            crate::obj::Gd::default()
        }
        pub(crate) fn set_bytecode_full(&mut self, bytecode: Gd < crate::engine::RdShaderSpirv >, version: StringName,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::RdShaderSpirv >, StringName);
            let args = (bytecode, version,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6606usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_bytecode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn set_bytecode(&mut self, bytecode: Gd < crate::engine::RdShaderSpirv >,) {
            self.set_bytecode_ex(bytecode,) . done()
        }
        #[inline]
        pub fn set_bytecode_ex(&mut self, bytecode: Gd < crate::engine::RdShaderSpirv >,) -> ExSetBytecode < '_ > {
            ExSetBytecode::new(self, bytecode,)
        }
        pub(crate) fn get_spirv_full(&self, version: StringName,) -> Option < Gd < crate::engine::RdShaderSpirv > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::RdShaderSpirv > >;
            type CallSig = (Option < Gd < crate::engine::RdShaderSpirv > >, StringName);
            let args = (version,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6607usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_spirv", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn get_spirv(&self,) -> Option < Gd < crate::engine::RdShaderSpirv > > {
            self.get_spirv_ex() . done()
        }
        #[inline]
        pub fn get_spirv_ex(&self,) -> ExGetSpirv < '_ > {
            ExGetSpirv::new(self,)
        }
        pub fn get_version_list(&self,) -> Array < StringName > {
            type RetMarshal = PtrcallReturnT < Array < StringName > >;
            type CallSig = (Array < StringName >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6608usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_version_list", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_base_error(&mut self, error: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString);
            let args = (error,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6609usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_base_error", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_base_error(&self,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6610usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_base_error", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for RdShaderFile {
        type Base = crate::engine::Resource;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"RDShaderFile\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for RdShaderFile {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for RdShaderFile {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::Resource > for RdShaderFile {
        
    }
    impl crate::obj::Inherits < crate::engine::RefCounted > for RdShaderFile {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for RdShaderFile {
        
    }
    impl crate::obj::ExportableObject for RdShaderFile {
        
    }
    impl crate::obj::cap::GodotDefault for RdShaderFile {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for RdShaderFile {
        type Target = crate::engine::Resource;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for RdShaderFile {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_RdShaderFile {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::RdShaderFile > for $Class {
                
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
#[doc = "Default-param extender for [`RdShaderFile::set_bytecode_ex`][super::RdShaderFile::set_bytecode_ex]."]
#[must_use]
pub struct ExSetBytecode < 'a > {
    surround_object: &'a mut re_export::RdShaderFile, bytecode: Gd < crate::engine::RdShaderSpirv >, version: StringName,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExSetBytecode < 'a > {
    fn new(surround_object: &'a mut re_export::RdShaderFile, bytecode: Gd < crate::engine::RdShaderSpirv >,) -> Self {
        Self {
            surround_object, bytecode, version: StringName::from(""),
        }
    }
    #[inline]
    pub fn version(self, value: StringName) -> Self {
        Self {
            version: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::RdShaderFile::set_bytecode_full(self.surround_object, self.bytecode, self.version,)
    }
}
#[doc = "Default-param extender for [`RdShaderFile::get_spirv_ex`][super::RdShaderFile::get_spirv_ex]."]
#[must_use]
pub struct ExGetSpirv < 'a > {
    surround_object: &'a re_export::RdShaderFile, version: StringName,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetSpirv < 'a > {
    fn new(surround_object: &'a re_export::RdShaderFile,) -> Self {
        Self {
            surround_object, version: StringName::from(""),
        }
    }
    #[inline]
    pub fn version(self, value: StringName) -> Self {
        Self {
            version: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::engine::RdShaderSpirv > > {
        re_export::RdShaderFile::get_spirv_full(self.surround_object, self.version,)
    }
}