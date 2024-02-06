#![doc = "Sidecar module for class [`RdTextureFormat`][crate::engine::RdTextureFormat].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `RDTextureFormat` enums](https://docs.godotengine.org/en/stable/classes/class_rdtextureformat.html#enumerations).\n\n"]
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
    #[doc = "Godot class `RDTextureFormat.`\n\nInherits [`RefCounted`][crate::engine::RefCounted].\n\nRelated symbols:\n\n* [`IRdTextureFormat`][crate::engine::IRdTextureFormat]: virtual methods\n\n\nSee also [Godot docs for `RDTextureFormat`](https://docs.godotengine.org/en/stable/classes/class_rdtextureformat.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct RdTextureFormat {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`RdTextureFormat`][crate::engine::RdTextureFormat].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `RDTextureFormat` methods](https://docs.godotengine.org/en/stable/classes/class_rdtextureformat.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IRdTextureFormat: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
    impl RdTextureFormat {
        #[deprecated = "Replaced with `new_gd` in extension trait `NewGd`."]
        pub fn new() -> Gd < Self > {
            crate::obj::Gd::default()
        }
        pub fn set_format(&mut self, p_member: crate::engine::rendering_device::DataFormat,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::rendering_device::DataFormat);
            let args = (p_member,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6619usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_format", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_format(&self,) -> crate::engine::rendering_device::DataFormat {
            type RetMarshal = PtrcallReturnT < crate::engine::rendering_device::DataFormat >;
            type CallSig = (crate::engine::rendering_device::DataFormat,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6620usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_format", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_width(&mut self, p_member: u32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), u32);
            let args = (p_member,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6621usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_width", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_width(&self,) -> u32 {
            type RetMarshal = PtrcallReturnT < u32 >;
            type CallSig = (u32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6622usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_width", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_height(&mut self, p_member: u32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), u32);
            let args = (p_member,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6623usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_height", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_height(&self,) -> u32 {
            type RetMarshal = PtrcallReturnT < u32 >;
            type CallSig = (u32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6624usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_height", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_depth(&mut self, p_member: u32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), u32);
            let args = (p_member,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6625usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_depth", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_depth(&self,) -> u32 {
            type RetMarshal = PtrcallReturnT < u32 >;
            type CallSig = (u32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6626usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_depth", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_array_layers(&mut self, p_member: u32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), u32);
            let args = (p_member,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6627usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_array_layers", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_array_layers(&self,) -> u32 {
            type RetMarshal = PtrcallReturnT < u32 >;
            type CallSig = (u32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6628usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_array_layers", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_mipmaps(&mut self, p_member: u32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), u32);
            let args = (p_member,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6629usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_mipmaps", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_mipmaps(&self,) -> u32 {
            type RetMarshal = PtrcallReturnT < u32 >;
            type CallSig = (u32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6630usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_mipmaps", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_texture_type(&mut self, p_member: crate::engine::rendering_device::TextureType,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::rendering_device::TextureType);
            let args = (p_member,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6631usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_texture_type", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_texture_type(&self,) -> crate::engine::rendering_device::TextureType {
            type RetMarshal = PtrcallReturnT < crate::engine::rendering_device::TextureType >;
            type CallSig = (crate::engine::rendering_device::TextureType,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6632usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_texture_type", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_samples(&mut self, p_member: crate::engine::rendering_device::TextureSamples,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::rendering_device::TextureSamples);
            let args = (p_member,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6633usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_samples", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_samples(&self,) -> crate::engine::rendering_device::TextureSamples {
            type RetMarshal = PtrcallReturnT < crate::engine::rendering_device::TextureSamples >;
            type CallSig = (crate::engine::rendering_device::TextureSamples,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6634usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_samples", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_usage_bits(&mut self, p_member: crate::engine::rendering_device::TextureUsageBits,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::rendering_device::TextureUsageBits);
            let args = (p_member,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6635usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_usage_bits", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_usage_bits(&self,) -> crate::engine::rendering_device::TextureUsageBits {
            type RetMarshal = PtrcallReturnT < crate::engine::rendering_device::TextureUsageBits >;
            type CallSig = (crate::engine::rendering_device::TextureUsageBits,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6636usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_usage_bits", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn add_shareable_format(&mut self, format: crate::engine::rendering_device::DataFormat,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::rendering_device::DataFormat);
            let args = (format,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6637usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_shareable_format", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn remove_shareable_format(&mut self, format: crate::engine::rendering_device::DataFormat,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::rendering_device::DataFormat);
            let args = (format,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6638usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "remove_shareable_format", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for RdTextureFormat {
        type Base = crate::engine::RefCounted;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"RDTextureFormat\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for RdTextureFormat {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for RdTextureFormat {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::RefCounted > for RdTextureFormat {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for RdTextureFormat {
        
    }
    impl crate::obj::cap::GodotDefault for RdTextureFormat {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for RdTextureFormat {
        type Target = crate::engine::RefCounted;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for RdTextureFormat {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_RdTextureFormat {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::RdTextureFormat > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::RefCounted > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::Object > for $Class {
                
            }
        }
    }
}