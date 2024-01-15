#![doc = "Sidecar module for class [`RenderSceneBuffersRd`][crate::engine::RenderSceneBuffersRd].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `RenderSceneBuffersRD` enums](https://docs.godotengine.org/en/stable/classes/class_renderscenebuffersrd.html#enumerations).\n\n"]
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
    #[doc = "Godot class `RenderSceneBuffersRD.`\n\nInherits [`RenderSceneBuffers`][crate::engine::RenderSceneBuffers].\n\nRelated symbols:\n\n* [`IRenderSceneBuffersRd`][crate::engine::IRenderSceneBuffersRd]: virtual methods\n\n\nSee also [Godot docs for `RenderSceneBuffersRD`](https://docs.godotengine.org/en/stable/classes/class_renderscenebuffersrd.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct RenderSceneBuffersRd {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`RenderSceneBuffersRd`][crate::engine::RenderSceneBuffersRd].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `RenderSceneBuffersRD` methods](https://docs.godotengine.org/en/stable/classes/class_renderscenebuffersrd.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IRenderSceneBuffersRd: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
    impl RenderSceneBuffersRd {
        pub fn has_texture(&self, context: StringName, name: StringName,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, StringName, StringName);
            let args = (context, name,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6858usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "has_texture", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn create_texture(&mut self, context: StringName, name: StringName, data_format: crate::engine::rendering_device::DataFormat, usage_bits: u32, texture_samples: crate::engine::rendering_device::TextureSamples, size: Vector2i, layers: u32, mipmaps: u32, unique: bool,) -> Rid {
            type RetMarshal = PtrcallReturnT < Rid >;
            type CallSig = (Rid, StringName, StringName, crate::engine::rendering_device::DataFormat, u32, crate::engine::rendering_device::TextureSamples, Vector2i, u32, u32, bool);
            let args = (context, name, data_format, usage_bits, texture_samples, size, layers, mipmaps, unique,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6859usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "create_texture", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn create_texture_from_format(&mut self, context: StringName, name: StringName, format: Gd < crate::engine::RdTextureFormat >, view: Gd < crate::engine::RdTextureView >, unique: bool,) -> Rid {
            type RetMarshal = PtrcallReturnT < Rid >;
            type CallSig = (Rid, StringName, StringName, Gd < crate::engine::RdTextureFormat >, Gd < crate::engine::RdTextureView >, bool);
            let args = (context, name, format, view, unique,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6860usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "create_texture_from_format", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn create_texture_view(&mut self, context: StringName, name: StringName, view_name: StringName, view: Gd < crate::engine::RdTextureView >,) -> Rid {
            type RetMarshal = PtrcallReturnT < Rid >;
            type CallSig = (Rid, StringName, StringName, StringName, Gd < crate::engine::RdTextureView >);
            let args = (context, name, view_name, view,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6861usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "create_texture_view", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_texture(&self, context: StringName, name: StringName,) -> Rid {
            type RetMarshal = PtrcallReturnT < Rid >;
            type CallSig = (Rid, StringName, StringName);
            let args = (context, name,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6862usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_texture", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_texture_format(&self, context: StringName, name: StringName,) -> Option < Gd < crate::engine::RdTextureFormat > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::RdTextureFormat > >;
            type CallSig = (Option < Gd < crate::engine::RdTextureFormat > >, StringName, StringName);
            let args = (context, name,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6863usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_texture_format", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_texture_slice(&mut self, context: StringName, name: StringName, layer: u32, mipmap: u32, layers: u32, mipmaps: u32,) -> Rid {
            type RetMarshal = PtrcallReturnT < Rid >;
            type CallSig = (Rid, StringName, StringName, u32, u32, u32, u32);
            let args = (context, name, layer, mipmap, layers, mipmaps,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6864usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_texture_slice", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_texture_slice_view(&mut self, context: StringName, name: StringName, layer: u32, mipmap: u32, layers: u32, mipmaps: u32, view: Gd < crate::engine::RdTextureView >,) -> Rid {
            type RetMarshal = PtrcallReturnT < Rid >;
            type CallSig = (Rid, StringName, StringName, u32, u32, u32, u32, Gd < crate::engine::RdTextureView >);
            let args = (context, name, layer, mipmap, layers, mipmaps, view,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6865usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_texture_slice_view", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_texture_slice_size(&mut self, context: StringName, name: StringName, mipmap: u32,) -> Vector2i {
            type RetMarshal = PtrcallReturnT < Vector2i >;
            type CallSig = (Vector2i, StringName, StringName, u32);
            let args = (context, name, mipmap,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6866usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_texture_slice_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn clear_context(&mut self, context: StringName,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), StringName);
            let args = (context,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6867usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "clear_context", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_color_texture(&mut self,) -> Rid {
            type RetMarshal = PtrcallReturnT < Rid >;
            type CallSig = (Rid,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6868usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_color_texture", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_color_layer(&mut self, layer: u32,) -> Rid {
            type RetMarshal = PtrcallReturnT < Rid >;
            type CallSig = (Rid, u32);
            let args = (layer,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6869usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_color_layer", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_depth_texture(&mut self,) -> Rid {
            type RetMarshal = PtrcallReturnT < Rid >;
            type CallSig = (Rid,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6870usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_depth_texture", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_depth_layer(&mut self, layer: u32,) -> Rid {
            type RetMarshal = PtrcallReturnT < Rid >;
            type CallSig = (Rid, u32);
            let args = (layer,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6871usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_depth_layer", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_velocity_texture(&mut self,) -> Rid {
            type RetMarshal = PtrcallReturnT < Rid >;
            type CallSig = (Rid,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6872usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_velocity_texture", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_velocity_layer(&mut self, layer: u32,) -> Rid {
            type RetMarshal = PtrcallReturnT < Rid >;
            type CallSig = (Rid, u32);
            let args = (layer,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6873usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_velocity_layer", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_render_target(&self,) -> Rid {
            type RetMarshal = PtrcallReturnT < Rid >;
            type CallSig = (Rid,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6874usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_render_target", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_view_count(&self,) -> u32 {
            type RetMarshal = PtrcallReturnT < u32 >;
            type CallSig = (u32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6875usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_view_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_internal_size(&self,) -> Vector2i {
            type RetMarshal = PtrcallReturnT < Vector2i >;
            type CallSig = (Vector2i,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6876usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_internal_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_use_taa(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6877usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_use_taa", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for RenderSceneBuffersRd {
        type Base = crate::engine::RenderSceneBuffers;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"RenderSceneBuffersRD\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for RenderSceneBuffersRd {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for RenderSceneBuffersRd {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::RenderSceneBuffers > for RenderSceneBuffersRd {
        
    }
    impl crate::obj::Inherits < crate::engine::RefCounted > for RenderSceneBuffersRd {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for RenderSceneBuffersRd {
        
    }
    impl std::ops::Deref for RenderSceneBuffersRd {
        type Target = crate::engine::RenderSceneBuffers;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for RenderSceneBuffersRd {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_RenderSceneBuffersRd {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::RenderSceneBuffersRd > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::RenderSceneBuffers > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::RefCounted > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::Object > for $Class {
                
            }
        }
    }
}