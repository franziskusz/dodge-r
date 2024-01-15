#![doc = "Sidecar module for class [`ArrayMesh`][crate::engine::ArrayMesh].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `ArrayMesh` enums](https://docs.godotengine.org/en/stable/classes/class_arraymesh.html#enumerations).\n\n"]
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
    #[doc = "Godot class `ArrayMesh.`\n\nInherits [`Mesh`][crate::engine::Mesh].\n\nRelated symbols:\n\n* [`array_mesh`][crate::engine::array_mesh]: sidecar module with related enum/flag types\n* [`IArrayMesh`][crate::engine::IArrayMesh]: virtual methods\n\n\nSee also [Godot docs for `ArrayMesh`](https://docs.godotengine.org/en/stable/classes/class_arraymesh.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct ArrayMesh {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`ArrayMesh`][crate::engine::ArrayMesh].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `ArrayMesh` methods](https://docs.godotengine.org/en/stable/classes/class_arraymesh.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IArrayMesh: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
    impl ArrayMesh {
        #[deprecated = "Replaced with `new_gd` in extension trait `NewGd`."]
        pub fn new() -> Gd < Self > {
            crate::obj::Gd::default()
        }
        pub fn add_blend_shape(&mut self, name: StringName,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), StringName);
            let args = (name,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(558usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_blend_shape", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_blend_shape_count(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(559usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_blend_shape_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_blend_shape_name(&self, index: i32,) -> StringName {
            type RetMarshal = PtrcallReturnT < StringName >;
            type CallSig = (StringName, i32);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(560usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_blend_shape_name", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_blend_shape_name(&mut self, index: i32, name: StringName,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, StringName);
            let args = (index, name,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(561usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_blend_shape_name", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn clear_blend_shapes(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(562usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "clear_blend_shapes", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_blend_shape_mode(&mut self, mode: crate::engine::mesh::BlendShapeMode,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::mesh::BlendShapeMode);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(563usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_blend_shape_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_blend_shape_mode(&self,) -> crate::engine::mesh::BlendShapeMode {
            type RetMarshal = PtrcallReturnT < crate::engine::mesh::BlendShapeMode >;
            type CallSig = (crate::engine::mesh::BlendShapeMode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(564usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_blend_shape_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn add_surface_from_arrays_full(&mut self, primitive: crate::engine::mesh::PrimitiveType, arrays: VariantArray, blend_shapes: Array < VariantArray >, lods: Dictionary, flags: crate::engine::mesh::ArrayFormat,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::mesh::PrimitiveType, VariantArray, Array < VariantArray >, Dictionary, crate::engine::mesh::ArrayFormat);
            let args = (primitive, arrays, blend_shapes, lods, flags,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(565usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_surface_from_arrays", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn add_surface_from_arrays(&mut self, primitive: crate::engine::mesh::PrimitiveType, arrays: VariantArray,) {
            self.add_surface_from_arrays_ex(primitive, arrays,) . done()
        }
        #[inline]
        pub fn add_surface_from_arrays_ex(&mut self, primitive: crate::engine::mesh::PrimitiveType, arrays: VariantArray,) -> ExAddSurfaceFromArrays < '_ > {
            ExAddSurfaceFromArrays::new(self, primitive, arrays,)
        }
        pub fn clear_surfaces(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(566usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "clear_surfaces", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn surface_update_vertex_region(&mut self, surf_idx: i32, offset: i32, data: PackedByteArray,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, i32, PackedByteArray);
            let args = (surf_idx, offset, data,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(567usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "surface_update_vertex_region", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn surface_update_attribute_region(&mut self, surf_idx: i32, offset: i32, data: PackedByteArray,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, i32, PackedByteArray);
            let args = (surf_idx, offset, data,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(568usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "surface_update_attribute_region", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn surface_update_skin_region(&mut self, surf_idx: i32, offset: i32, data: PackedByteArray,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, i32, PackedByteArray);
            let args = (surf_idx, offset, data,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(569usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "surface_update_skin_region", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn surface_get_array_len(&self, surf_idx: i32,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, i32);
            let args = (surf_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(570usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "surface_get_array_len", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn surface_get_array_index_len(&self, surf_idx: i32,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, i32);
            let args = (surf_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(571usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "surface_get_array_index_len", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn surface_get_format(&self, surf_idx: i32,) -> crate::engine::mesh::ArrayFormat {
            type RetMarshal = PtrcallReturnT < crate::engine::mesh::ArrayFormat >;
            type CallSig = (crate::engine::mesh::ArrayFormat, i32);
            let args = (surf_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(572usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "surface_get_format", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn surface_get_primitive_type(&self, surf_idx: i32,) -> crate::engine::mesh::PrimitiveType {
            type RetMarshal = PtrcallReturnT < crate::engine::mesh::PrimitiveType >;
            type CallSig = (crate::engine::mesh::PrimitiveType, i32);
            let args = (surf_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(573usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "surface_get_primitive_type", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn surface_find_by_name(&self, name: GString,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, GString);
            let args = (name,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(574usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "surface_find_by_name", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn surface_set_name(&mut self, surf_idx: i32, name: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, GString);
            let args = (surf_idx, name,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(575usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "surface_set_name", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn surface_get_name(&self, surf_idx: i32,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString, i32);
            let args = (surf_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(576usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "surface_get_name", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn regen_normal_maps(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(577usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "regen_normal_maps", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn lightmap_unwrap(&mut self, transform: Transform3D, texel_size: f32,) -> crate::engine::global::Error {
            type RetMarshal = PtrcallReturnT < crate::engine::global::Error >;
            type CallSig = (crate::engine::global::Error, Transform3D, f32);
            let args = (transform, texel_size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(578usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "lightmap_unwrap", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_custom_aabb(&mut self, aabb: Aabb,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Aabb);
            let args = (aabb,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(579usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_custom_aabb", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_custom_aabb(&self,) -> Aabb {
            type RetMarshal = PtrcallReturnT < Aabb >;
            type CallSig = (Aabb,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(580usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_custom_aabb", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_shadow_mesh(&mut self, mesh: Gd < crate::engine::ArrayMesh >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::ArrayMesh >);
            let args = (mesh,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(581usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_shadow_mesh", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_shadow_mesh(&self,) -> Option < Gd < crate::engine::ArrayMesh > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::ArrayMesh > >;
            type CallSig = (Option < Gd < crate::engine::ArrayMesh > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(582usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_shadow_mesh", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for ArrayMesh {
        type Base = crate::engine::Mesh;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"ArrayMesh\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for ArrayMesh {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for ArrayMesh {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::Mesh > for ArrayMesh {
        
    }
    impl crate::obj::Inherits < crate::engine::Resource > for ArrayMesh {
        
    }
    impl crate::obj::Inherits < crate::engine::RefCounted > for ArrayMesh {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for ArrayMesh {
        
    }
    impl crate::obj::ExportableObject for ArrayMesh {
        
    }
    impl crate::obj::cap::GodotDefault for ArrayMesh {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for ArrayMesh {
        type Target = crate::engine::Mesh;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for ArrayMesh {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_ArrayMesh {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::ArrayMesh > for $Class {
                
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
#[doc = "Default-param extender for [`ArrayMesh::add_surface_from_arrays_ex`][super::ArrayMesh::add_surface_from_arrays_ex]."]
#[must_use]
pub struct ExAddSurfaceFromArrays < 'a > {
    surround_object: &'a mut re_export::ArrayMesh, primitive: crate::engine::mesh::PrimitiveType, arrays: VariantArray, blend_shapes: Array < VariantArray >, lods: Dictionary, flags: crate::engine::mesh::ArrayFormat,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddSurfaceFromArrays < 'a > {
    fn new(surround_object: &'a mut re_export::ArrayMesh, primitive: crate::engine::mesh::PrimitiveType, arrays: VariantArray,) -> Self {
        Self {
            surround_object, primitive, arrays, blend_shapes: Array::new(), lods: Dictionary::new(), flags: crate::obj::EngineBitfield::from_ord(0),
        }
    }
    #[inline]
    pub fn blend_shapes(self, value: Array < VariantArray >) -> Self {
        Self {
            blend_shapes: value, .. self
        }
    }
    #[inline]
    pub fn lods(self, value: Dictionary) -> Self {
        Self {
            lods: value, .. self
        }
    }
    #[inline]
    pub fn flags(self, value: crate::engine::mesh::ArrayFormat) -> Self {
        Self {
            flags: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::ArrayMesh::add_surface_from_arrays_full(self.surround_object, self.primitive, self.arrays, self.blend_shapes, self.lods, self.flags,)
    }
}