#![doc = "Sidecar module for class [`RdPipelineColorBlendStateAttachment`][crate::engine::RdPipelineColorBlendStateAttachment].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `RDPipelineColorBlendStateAttachment` enums](https://docs.godotengine.org/en/stable/classes/class_rdpipelinecolorblendstateattachment.html#enumerations).\n\n"]
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
    #[doc = "Godot class `RDPipelineColorBlendStateAttachment.`\n\nInherits [`RefCounted`][crate::engine::RefCounted].\n\nRelated symbols:\n\n* [`IRdPipelineColorBlendStateAttachment`][crate::engine::IRdPipelineColorBlendStateAttachment]: virtual methods\n\n\nSee also [Godot docs for `RDPipelineColorBlendStateAttachment`](https://docs.godotengine.org/en/stable/classes/class_rdpipelinecolorblendstateattachment.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct RdPipelineColorBlendStateAttachment {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`RdPipelineColorBlendStateAttachment`][crate::engine::RdPipelineColorBlendStateAttachment].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `RDPipelineColorBlendStateAttachment` methods](https://docs.godotengine.org/en/stable/classes/class_rdpipelinecolorblendstateattachment.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IRdPipelineColorBlendStateAttachment: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
    impl RdPipelineColorBlendStateAttachment {
        #[deprecated = "Replaced with `new_gd` in extension trait `NewGd`."]
        pub fn new() -> Gd < Self > {
            crate::obj::Gd::default()
        }
        pub fn set_as_mix(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6473usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_as_mix", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_enable_blend(&mut self, p_member: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (p_member,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6474usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_enable_blend", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_enable_blend(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6475usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_enable_blend", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_src_color_blend_factor(&mut self, p_member: crate::engine::rendering_device::BlendFactor,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::rendering_device::BlendFactor);
            let args = (p_member,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6476usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_src_color_blend_factor", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_src_color_blend_factor(&self,) -> crate::engine::rendering_device::BlendFactor {
            type RetMarshal = PtrcallReturnT < crate::engine::rendering_device::BlendFactor >;
            type CallSig = (crate::engine::rendering_device::BlendFactor,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6477usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_src_color_blend_factor", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_dst_color_blend_factor(&mut self, p_member: crate::engine::rendering_device::BlendFactor,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::rendering_device::BlendFactor);
            let args = (p_member,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6478usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_dst_color_blend_factor", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_dst_color_blend_factor(&self,) -> crate::engine::rendering_device::BlendFactor {
            type RetMarshal = PtrcallReturnT < crate::engine::rendering_device::BlendFactor >;
            type CallSig = (crate::engine::rendering_device::BlendFactor,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6479usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_dst_color_blend_factor", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_color_blend_op(&mut self, p_member: crate::engine::rendering_device::BlendOperation,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::rendering_device::BlendOperation);
            let args = (p_member,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6480usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_color_blend_op", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_color_blend_op(&self,) -> crate::engine::rendering_device::BlendOperation {
            type RetMarshal = PtrcallReturnT < crate::engine::rendering_device::BlendOperation >;
            type CallSig = (crate::engine::rendering_device::BlendOperation,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6481usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_color_blend_op", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_src_alpha_blend_factor(&mut self, p_member: crate::engine::rendering_device::BlendFactor,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::rendering_device::BlendFactor);
            let args = (p_member,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6482usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_src_alpha_blend_factor", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_src_alpha_blend_factor(&self,) -> crate::engine::rendering_device::BlendFactor {
            type RetMarshal = PtrcallReturnT < crate::engine::rendering_device::BlendFactor >;
            type CallSig = (crate::engine::rendering_device::BlendFactor,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6483usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_src_alpha_blend_factor", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_dst_alpha_blend_factor(&mut self, p_member: crate::engine::rendering_device::BlendFactor,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::rendering_device::BlendFactor);
            let args = (p_member,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6484usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_dst_alpha_blend_factor", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_dst_alpha_blend_factor(&self,) -> crate::engine::rendering_device::BlendFactor {
            type RetMarshal = PtrcallReturnT < crate::engine::rendering_device::BlendFactor >;
            type CallSig = (crate::engine::rendering_device::BlendFactor,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6485usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_dst_alpha_blend_factor", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_alpha_blend_op(&mut self, p_member: crate::engine::rendering_device::BlendOperation,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::rendering_device::BlendOperation);
            let args = (p_member,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6486usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_alpha_blend_op", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_alpha_blend_op(&self,) -> crate::engine::rendering_device::BlendOperation {
            type RetMarshal = PtrcallReturnT < crate::engine::rendering_device::BlendOperation >;
            type CallSig = (crate::engine::rendering_device::BlendOperation,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6487usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_alpha_blend_op", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_write_r(&mut self, p_member: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (p_member,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6488usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_write_r", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_write_r(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6489usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_write_r", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_write_g(&mut self, p_member: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (p_member,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6490usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_write_g", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_write_g(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6491usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_write_g", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_write_b(&mut self, p_member: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (p_member,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6492usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_write_b", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_write_b(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6493usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_write_b", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_write_a(&mut self, p_member: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (p_member,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6494usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_write_a", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_write_a(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6495usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_write_a", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for RdPipelineColorBlendStateAttachment {
        type Base = crate::engine::RefCounted;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"RDPipelineColorBlendStateAttachment\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for RdPipelineColorBlendStateAttachment {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for RdPipelineColorBlendStateAttachment {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::RefCounted > for RdPipelineColorBlendStateAttachment {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for RdPipelineColorBlendStateAttachment {
        
    }
    impl crate::obj::cap::GodotDefault for RdPipelineColorBlendStateAttachment {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for RdPipelineColorBlendStateAttachment {
        type Target = crate::engine::RefCounted;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for RdPipelineColorBlendStateAttachment {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_RdPipelineColorBlendStateAttachment {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::RdPipelineColorBlendStateAttachment > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::RefCounted > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::Object > for $Class {
                
            }
        }
    }
}