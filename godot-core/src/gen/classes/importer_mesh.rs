#![doc = "Sidecar module for class [`ImporterMesh`][crate::engine::ImporterMesh].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `ImporterMesh` enums](https://docs.godotengine.org/en/stable/classes/class_importermesh.html#enumerations).\n\n"]
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
    #[doc = "Godot class `ImporterMesh.`\n\nInherits [`Resource`][crate::engine::Resource].\n\nRelated symbols:\n\n* [`importer_mesh`][crate::engine::importer_mesh]: sidecar module with related enum/flag types\n* [`IImporterMesh`][crate::engine::IImporterMesh]: virtual methods\n\n\nSee also [Godot docs for `ImporterMesh`](https://docs.godotengine.org/en/stable/classes/class_importermesh.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct ImporterMesh {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`ImporterMesh`][crate::engine::ImporterMesh].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `ImporterMesh` methods](https://docs.godotengine.org/en/stable/classes/class_importermesh.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IImporterMesh: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
    impl ImporterMesh {
        #[deprecated = "Replaced with `new_gd` in extension trait `NewGd`."]
        pub fn new() -> Gd < Self > {
            crate::obj::Gd::default()
        }
        pub fn add_blend_shape(&mut self, name: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString);
            let args = (name,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3851usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_blend_shape", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_blend_shape_count(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3852usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_blend_shape_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_blend_shape_name(&self, blend_shape_idx: i32,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString, i32);
            let args = (blend_shape_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3853usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_blend_shape_name", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_blend_shape_mode(&mut self, mode: crate::engine::mesh::BlendShapeMode,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::mesh::BlendShapeMode);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3854usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_blend_shape_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_blend_shape_mode(&self,) -> crate::engine::mesh::BlendShapeMode {
            type RetMarshal = PtrcallReturnT < crate::engine::mesh::BlendShapeMode >;
            type CallSig = (crate::engine::mesh::BlendShapeMode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3855usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_blend_shape_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn add_surface_full(&mut self, primitive: crate::engine::mesh::PrimitiveType, arrays: VariantArray, blend_shapes: Array < VariantArray >, lods: Dictionary, material: Gd < crate::engine::Material >, name: GString, flags: u64,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::mesh::PrimitiveType, VariantArray, Array < VariantArray >, Dictionary, Gd < crate::engine::Material >, GString, u64);
            let args = (primitive, arrays, blend_shapes, lods, material, name, flags,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3856usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_surface", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn add_surface(&mut self, primitive: crate::engine::mesh::PrimitiveType, arrays: VariantArray,) {
            self.add_surface_ex(primitive, arrays,) . done()
        }
        #[inline]
        pub fn add_surface_ex(&mut self, primitive: crate::engine::mesh::PrimitiveType, arrays: VariantArray,) -> ExAddSurface < '_ > {
            ExAddSurface::new(self, primitive, arrays,)
        }
        pub fn get_surface_count(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3857usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_surface_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_surface_primitive_type(&mut self, surface_idx: i32,) -> crate::engine::mesh::PrimitiveType {
            type RetMarshal = PtrcallReturnT < crate::engine::mesh::PrimitiveType >;
            type CallSig = (crate::engine::mesh::PrimitiveType, i32);
            let args = (surface_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3858usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_surface_primitive_type", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_surface_name(&self, surface_idx: i32,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString, i32);
            let args = (surface_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3859usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_surface_name", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_surface_arrays(&self, surface_idx: i32,) -> VariantArray {
            type RetMarshal = PtrcallReturnT < VariantArray >;
            type CallSig = (VariantArray, i32);
            let args = (surface_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3860usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_surface_arrays", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_surface_blend_shape_arrays(&self, surface_idx: i32, blend_shape_idx: i32,) -> VariantArray {
            type RetMarshal = PtrcallReturnT < VariantArray >;
            type CallSig = (VariantArray, i32, i32);
            let args = (surface_idx, blend_shape_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3861usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_surface_blend_shape_arrays", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_surface_lod_count(&self, surface_idx: i32,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, i32);
            let args = (surface_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3862usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_surface_lod_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_surface_lod_size(&self, surface_idx: i32, lod_idx: i32,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32, i32, i32);
            let args = (surface_idx, lod_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3863usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_surface_lod_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_surface_lod_indices(&self, surface_idx: i32, lod_idx: i32,) -> PackedInt32Array {
            type RetMarshal = PtrcallReturnT < PackedInt32Array >;
            type CallSig = (PackedInt32Array, i32, i32);
            let args = (surface_idx, lod_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3864usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_surface_lod_indices", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_surface_material(&self, surface_idx: i32,) -> Option < Gd < crate::engine::Material > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::Material > >;
            type CallSig = (Option < Gd < crate::engine::Material > >, i32);
            let args = (surface_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3865usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_surface_material", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_surface_format(&self, surface_idx: i32,) -> u64 {
            type RetMarshal = PtrcallReturnT < u64 >;
            type CallSig = (u64, i32);
            let args = (surface_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3866usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_surface_format", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_surface_name(&mut self, surface_idx: i32, name: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, GString);
            let args = (surface_idx, name,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3867usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_surface_name", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_surface_material(&mut self, surface_idx: i32, material: Gd < crate::engine::Material >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, Gd < crate::engine::Material >);
            let args = (surface_idx, material,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3868usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_surface_material", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn generate_lods(&mut self, normal_merge_angle: f32, normal_split_angle: f32, bone_transform_array: VariantArray,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32, f32, VariantArray);
            let args = (normal_merge_angle, normal_split_angle, bone_transform_array,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3869usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "generate_lods", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn get_mesh_full(&mut self, base_mesh: Gd < crate::engine::ArrayMesh >,) -> Option < Gd < crate::engine::ArrayMesh > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::ArrayMesh > >;
            type CallSig = (Option < Gd < crate::engine::ArrayMesh > >, Gd < crate::engine::ArrayMesh >);
            let args = (base_mesh,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3870usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_mesh", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn get_mesh(&mut self,) -> Option < Gd < crate::engine::ArrayMesh > > {
            self.get_mesh_ex() . done()
        }
        #[inline]
        pub fn get_mesh_ex(&mut self,) -> ExGetMesh < '_ > {
            ExGetMesh::new(self,)
        }
        pub fn clear(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3871usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "clear", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_lightmap_size_hint(&mut self, size: Vector2i,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Vector2i);
            let args = (size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3872usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_lightmap_size_hint", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_lightmap_size_hint(&self,) -> Vector2i {
            type RetMarshal = PtrcallReturnT < Vector2i >;
            type CallSig = (Vector2i,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3873usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_lightmap_size_hint", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for ImporterMesh {
        type Base = crate::engine::Resource;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"ImporterMesh\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for ImporterMesh {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for ImporterMesh {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::Resource > for ImporterMesh {
        
    }
    impl crate::obj::Inherits < crate::engine::RefCounted > for ImporterMesh {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for ImporterMesh {
        
    }
    impl crate::obj::ExportableObject for ImporterMesh {
        
    }
    impl crate::obj::cap::GodotDefault for ImporterMesh {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for ImporterMesh {
        type Target = crate::engine::Resource;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for ImporterMesh {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_ImporterMesh {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::ImporterMesh > for $Class {
                
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
#[doc = "Default-param extender for [`ImporterMesh::add_surface_ex`][super::ImporterMesh::add_surface_ex]."]
#[must_use]
pub struct ExAddSurface < 'a > {
    surround_object: &'a mut re_export::ImporterMesh, primitive: crate::engine::mesh::PrimitiveType, arrays: VariantArray, blend_shapes: Array < VariantArray >, lods: Dictionary, material: Gd < crate::engine::Material >, name: GString, flags: u64,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddSurface < 'a > {
    fn new(surround_object: &'a mut re_export::ImporterMesh, primitive: crate::engine::mesh::PrimitiveType, arrays: VariantArray,) -> Self {
        Self {
            surround_object, primitive, arrays, blend_shapes: Array::new(), lods: Dictionary::new(), material: unimplemented !("see #156"), name: GString::from(""), flags: 0u64,
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
    pub fn material(self, value: Gd < crate::engine::Material >) -> Self {
        Self {
            material: value, .. self
        }
    }
    #[inline]
    pub fn name(self, value: GString) -> Self {
        Self {
            name: value, .. self
        }
    }
    #[inline]
    pub fn flags(self, value: u64) -> Self {
        Self {
            flags: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::ImporterMesh::add_surface_full(self.surround_object, self.primitive, self.arrays, self.blend_shapes, self.lods, self.material, self.name, self.flags,)
    }
}
#[doc = "Default-param extender for [`ImporterMesh::get_mesh_ex`][super::ImporterMesh::get_mesh_ex]."]
#[must_use]
pub struct ExGetMesh < 'a > {
    surround_object: &'a mut re_export::ImporterMesh, base_mesh: Gd < crate::engine::ArrayMesh >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetMesh < 'a > {
    fn new(surround_object: &'a mut re_export::ImporterMesh,) -> Self {
        Self {
            surround_object, base_mesh: unimplemented !("see #156"),
        }
    }
    #[inline]
    pub fn base_mesh(self, value: Gd < crate::engine::ArrayMesh >) -> Self {
        Self {
            base_mesh: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::engine::ArrayMesh > > {
        re_export::ImporterMesh::get_mesh_full(self.surround_object, self.base_mesh,)
    }
}