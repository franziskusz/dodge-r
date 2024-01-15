#![doc = "Sidecar module for class [`ImmediateMesh`][crate::engine::ImmediateMesh].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `ImmediateMesh` enums](https://docs.godotengine.org/en/stable/classes/class_immediatemesh.html#enumerations).\n\n"]
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
    #[doc = "Godot class `ImmediateMesh.`\n\nInherits [`Mesh`][crate::engine::Mesh].\n\nRelated symbols:\n\n* [`immediate_mesh`][crate::engine::immediate_mesh]: sidecar module with related enum/flag types\n* [`IImmediateMesh`][crate::engine::IImmediateMesh]: virtual methods\n\n\nSee also [Godot docs for `ImmediateMesh`](https://docs.godotengine.org/en/stable/classes/class_immediatemesh.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct ImmediateMesh {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`ImmediateMesh`][crate::engine::ImmediateMesh].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `ImmediateMesh` methods](https://docs.godotengine.org/en/stable/classes/class_immediatemesh.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IImmediateMesh: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
    impl ImmediateMesh {
        #[deprecated = "Replaced with `new_gd` in extension trait `NewGd`."]
        pub fn new() -> Gd < Self > {
            crate::obj::Gd::default()
        }
        pub(crate) fn surface_begin_full(&mut self, primitive: crate::engine::mesh::PrimitiveType, material: Gd < crate::engine::Material >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::mesh::PrimitiveType, Gd < crate::engine::Material >);
            let args = (primitive, material,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3841usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "surface_begin", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn surface_begin(&mut self, primitive: crate::engine::mesh::PrimitiveType,) {
            self.surface_begin_ex(primitive,) . done()
        }
        #[inline]
        pub fn surface_begin_ex(&mut self, primitive: crate::engine::mesh::PrimitiveType,) -> ExSurfaceBegin < '_ > {
            ExSurfaceBegin::new(self, primitive,)
        }
        pub fn surface_set_color(&mut self, color: Color,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Color);
            let args = (color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3842usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "surface_set_color", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn surface_set_normal(&mut self, normal: Vector3,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Vector3);
            let args = (normal,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3843usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "surface_set_normal", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn surface_set_tangent(&mut self, tangent: Plane,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Plane);
            let args = (tangent,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3844usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "surface_set_tangent", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn surface_set_uv(&mut self, uv: Vector2,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Vector2);
            let args = (uv,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3845usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "surface_set_uv", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn surface_set_uv2(&mut self, uv2: Vector2,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Vector2);
            let args = (uv2,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3846usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "surface_set_uv2", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn surface_add_vertex(&mut self, vertex: Vector3,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Vector3);
            let args = (vertex,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3847usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "surface_add_vertex", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn surface_add_vertex_2d(&mut self, vertex: Vector2,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Vector2);
            let args = (vertex,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3848usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "surface_add_vertex_2d", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn surface_end(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3849usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "surface_end", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn clear_surfaces(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3850usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "clear_surfaces", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for ImmediateMesh {
        type Base = crate::engine::Mesh;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"ImmediateMesh\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for ImmediateMesh {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for ImmediateMesh {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::Mesh > for ImmediateMesh {
        
    }
    impl crate::obj::Inherits < crate::engine::Resource > for ImmediateMesh {
        
    }
    impl crate::obj::Inherits < crate::engine::RefCounted > for ImmediateMesh {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for ImmediateMesh {
        
    }
    impl crate::obj::ExportableObject for ImmediateMesh {
        
    }
    impl crate::obj::cap::GodotDefault for ImmediateMesh {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for ImmediateMesh {
        type Target = crate::engine::Mesh;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for ImmediateMesh {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_ImmediateMesh {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::ImmediateMesh > for $Class {
                
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
#[doc = "Default-param extender for [`ImmediateMesh::surface_begin_ex`][super::ImmediateMesh::surface_begin_ex]."]
#[must_use]
pub struct ExSurfaceBegin < 'a > {
    surround_object: &'a mut re_export::ImmediateMesh, primitive: crate::engine::mesh::PrimitiveType, material: Gd < crate::engine::Material >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExSurfaceBegin < 'a > {
    fn new(surround_object: &'a mut re_export::ImmediateMesh, primitive: crate::engine::mesh::PrimitiveType,) -> Self {
        Self {
            surround_object, primitive, material: unimplemented !("see #156"),
        }
    }
    #[inline]
    pub fn material(self, value: Gd < crate::engine::Material >) -> Self {
        Self {
            material: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::ImmediateMesh::surface_begin_full(self.surround_object, self.primitive, self.material,)
    }
}