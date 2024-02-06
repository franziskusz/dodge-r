#![doc = "Sidecar module for class [`TorusMesh`][crate::engine::TorusMesh].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `TorusMesh` enums](https://docs.godotengine.org/en/stable/classes/class_torusmesh.html#enumerations).\n\n"]
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
    #[doc = "Godot class `TorusMesh.`\n\nInherits [`PrimitiveMesh`][crate::engine::PrimitiveMesh].\n\nRelated symbols:\n\n* [`ITorusMesh`][crate::engine::ITorusMesh]: virtual methods\n\n\nSee also [Godot docs for `TorusMesh`](https://docs.godotengine.org/en/stable/classes/class_torusmesh.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct TorusMesh {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`TorusMesh`][crate::engine::TorusMesh].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `TorusMesh` methods](https://docs.godotengine.org/en/stable/classes/class_torusmesh.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ITorusMesh: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
        fn create_mesh_array(&self,) -> VariantArray {
            unimplemented !()
        }
        fn get_surface_count(&self,) -> i32 {
            unimplemented !()
        }
        fn surface_get_array_len(&self, index: i32,) -> i32 {
            unimplemented !()
        }
        fn surface_get_array_index_len(&self, index: i32,) -> i32 {
            unimplemented !()
        }
        fn surface_get_arrays(&self, index: i32,) -> VariantArray {
            unimplemented !()
        }
        fn surface_get_blend_shape_arrays(&self, index: i32,) -> Array < VariantArray > {
            unimplemented !()
        }
        fn surface_get_lods(&self, index: i32,) -> Dictionary {
            unimplemented !()
        }
        fn surface_get_format(&self, index: i32,) -> u32 {
            unimplemented !()
        }
        fn surface_get_primitive_type(&self, index: i32,) -> u32 {
            unimplemented !()
        }
        fn surface_set_material(&mut self, index: i32, material: Gd < crate::engine::Material >,) {
            unimplemented !()
        }
        fn surface_get_material(&self, index: i32,) -> Option < Gd < crate::engine::Material > > {
            unimplemented !()
        }
        fn get_blend_shape_count(&self,) -> i32 {
            unimplemented !()
        }
        fn get_blend_shape_name(&self, index: i32,) -> StringName {
            unimplemented !()
        }
        fn set_blend_shape_name(&mut self, index: i32, name: StringName,) {
            unimplemented !()
        }
        fn get_aabb(&self,) -> Aabb {
            unimplemented !()
        }
        fn setup_local_to_scene(&mut self,) {
            unimplemented !()
        }
    }
    impl TorusMesh {
        #[deprecated = "Replaced with `new_gd` in extension trait `NewGd`."]
        pub fn new() -> Gd < Self > {
            crate::obj::Gd::default()
        }
        pub fn set_inner_radius(&mut self, radius: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (radius,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8921usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_inner_radius", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_inner_radius(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8922usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_inner_radius", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_outer_radius(&mut self, radius: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (radius,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8923usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_outer_radius", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_outer_radius(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8924usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_outer_radius", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_rings(&mut self, rings: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (rings,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8925usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_rings", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_rings(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8926usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_rings", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_ring_segments(&mut self, rings: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (rings,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8927usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_ring_segments", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_ring_segments(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8928usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_ring_segments", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for TorusMesh {
        type Base = crate::engine::PrimitiveMesh;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"TorusMesh\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for TorusMesh {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for TorusMesh {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::PrimitiveMesh > for TorusMesh {
        
    }
    impl crate::obj::Inherits < crate::engine::Mesh > for TorusMesh {
        
    }
    impl crate::obj::Inherits < crate::engine::Resource > for TorusMesh {
        
    }
    impl crate::obj::Inherits < crate::engine::RefCounted > for TorusMesh {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for TorusMesh {
        
    }
    impl crate::obj::ExportableObject for TorusMesh {
        
    }
    impl crate::obj::cap::GodotDefault for TorusMesh {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for TorusMesh {
        type Target = crate::engine::PrimitiveMesh;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for TorusMesh {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_TorusMesh {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::TorusMesh > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::PrimitiveMesh > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::Mesh > for $Class {
                
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