#![doc = "Sidecar module for class [`SurfaceTool`][crate::engine::SurfaceTool].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `SurfaceTool` enums](https://docs.godotengine.org/en/stable/classes/class_surfacetool.html#enumerations).\n\n"]
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
    #[doc = "Godot class `SurfaceTool.`\n\nInherits [`RefCounted`][crate::engine::RefCounted].\n\nRelated symbols:\n\n* [`surface_tool`][crate::engine::surface_tool]: sidecar module with related enum/flag types\n* [`ISurfaceTool`][crate::engine::ISurfaceTool]: virtual methods\n\n\nSee also [Godot docs for `SurfaceTool`](https://docs.godotengine.org/en/stable/classes/class_surfacetool.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct SurfaceTool {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`SurfaceTool`][crate::engine::SurfaceTool].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `SurfaceTool` methods](https://docs.godotengine.org/en/stable/classes/class_surfacetool.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ISurfaceTool: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
    impl SurfaceTool {
        #[deprecated = "Replaced with `new_gd` in extension trait `NewGd`."]
        pub fn new() -> Gd < Self > {
            crate::obj::Gd::default()
        }
        pub fn set_skin_weight_count(&mut self, count: crate::engine::surface_tool::SkinWeightCount,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::surface_tool::SkinWeightCount);
            let args = (count,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7977usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_skin_weight_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_skin_weight_count(&self,) -> crate::engine::surface_tool::SkinWeightCount {
            type RetMarshal = PtrcallReturnT < crate::engine::surface_tool::SkinWeightCount >;
            type CallSig = (crate::engine::surface_tool::SkinWeightCount,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7978usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_skin_weight_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_custom_format(&mut self, channel_index: i32, format: crate::engine::surface_tool::CustomFormat,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, crate::engine::surface_tool::CustomFormat);
            let args = (channel_index, format,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7979usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_custom_format", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_custom_format(&self, channel_index: i32,) -> crate::engine::surface_tool::CustomFormat {
            type RetMarshal = PtrcallReturnT < crate::engine::surface_tool::CustomFormat >;
            type CallSig = (crate::engine::surface_tool::CustomFormat, i32);
            let args = (channel_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7980usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_custom_format", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn begin(&mut self, primitive: crate::engine::mesh::PrimitiveType,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::mesh::PrimitiveType);
            let args = (primitive,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7981usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "begin", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn add_vertex(&mut self, vertex: Vector3,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Vector3);
            let args = (vertex,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7982usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_vertex", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_color(&mut self, color: Color,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Color);
            let args = (color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7983usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_color", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_normal(&mut self, normal: Vector3,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Vector3);
            let args = (normal,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7984usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_normal", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_tangent(&mut self, tangent: Plane,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Plane);
            let args = (tangent,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7985usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_tangent", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_uv(&mut self, uv: Vector2,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Vector2);
            let args = (uv,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7986usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_uv", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_uv2(&mut self, uv2: Vector2,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Vector2);
            let args = (uv2,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7987usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_uv2", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_bones(&mut self, bones: PackedInt32Array,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), PackedInt32Array);
            let args = (bones,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7988usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_bones", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_weights(&mut self, weights: PackedFloat32Array,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), PackedFloat32Array);
            let args = (weights,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7989usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_weights", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_custom(&mut self, channel_index: i32, custom_color: Color,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, Color);
            let args = (channel_index, custom_color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7990usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_custom", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_smooth_group(&mut self, index: u32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), u32);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7991usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_smooth_group", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn add_triangle_fan_full(&mut self, vertices: PackedVector3Array, uvs: PackedVector2Array, colors: PackedColorArray, uv2s: PackedVector2Array, normals: PackedVector3Array, tangents: Array < Plane >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), PackedVector3Array, PackedVector2Array, PackedColorArray, PackedVector2Array, PackedVector3Array, Array < Plane >);
            let args = (vertices, uvs, colors, uv2s, normals, tangents,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7992usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_triangle_fan", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn add_triangle_fan(&mut self, vertices: PackedVector3Array,) {
            self.add_triangle_fan_ex(vertices,) . done()
        }
        #[inline]
        pub fn add_triangle_fan_ex(&mut self, vertices: PackedVector3Array,) -> ExAddTriangleFan < '_ > {
            ExAddTriangleFan::new(self, vertices,)
        }
        pub fn add_index(&mut self, index: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7993usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_index", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn index(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7994usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "index", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn deindex(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7995usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "deindex", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn generate_normals_full(&mut self, flip: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (flip,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7996usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "generate_normals", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn generate_normals(&mut self,) {
            self.generate_normals_ex() . done()
        }
        #[inline]
        pub fn generate_normals_ex(&mut self,) -> ExGenerateNormals < '_ > {
            ExGenerateNormals::new(self,)
        }
        pub fn generate_tangents(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7997usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "generate_tangents", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn optimize_indices_for_cache(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7998usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "optimize_indices_for_cache", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_aabb(&self,) -> Aabb {
            type RetMarshal = PtrcallReturnT < Aabb >;
            type CallSig = (Aabb,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7999usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_aabb", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn generate_lod_full(&mut self, nd_threshold: f32, target_index_count: i32,) -> PackedInt32Array {
            type RetMarshal = PtrcallReturnT < PackedInt32Array >;
            type CallSig = (PackedInt32Array, f32, i32);
            let args = (nd_threshold, target_index_count,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8000usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "generate_lod", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn generate_lod(&mut self, nd_threshold: f32,) -> PackedInt32Array {
            self.generate_lod_ex(nd_threshold,) . done()
        }
        #[inline]
        pub fn generate_lod_ex(&mut self, nd_threshold: f32,) -> ExGenerateLod < '_ > {
            ExGenerateLod::new(self, nd_threshold,)
        }
        pub fn set_material(&mut self, material: Gd < crate::engine::Material >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::Material >);
            let args = (material,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8001usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_material", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_primitive_type(&self,) -> crate::engine::mesh::PrimitiveType {
            type RetMarshal = PtrcallReturnT < crate::engine::mesh::PrimitiveType >;
            type CallSig = (crate::engine::mesh::PrimitiveType,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8002usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_primitive_type", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn clear(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8003usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "clear", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn create_from(&mut self, existing: Gd < crate::engine::Mesh >, surface: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::Mesh >, i32);
            let args = (existing, surface,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8004usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "create_from", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn create_from_blend_shape(&mut self, existing: Gd < crate::engine::Mesh >, surface: i32, blend_shape: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::Mesh >, i32, GString);
            let args = (existing, surface, blend_shape,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8005usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "create_from_blend_shape", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn append_from(&mut self, existing: Gd < crate::engine::Mesh >, surface: i32, transform: Transform3D,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::Mesh >, i32, Transform3D);
            let args = (existing, surface, transform,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8006usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "append_from", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn commit_full(&mut self, existing: Gd < crate::engine::ArrayMesh >, flags: u64,) -> Option < Gd < crate::engine::ArrayMesh > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::ArrayMesh > >;
            type CallSig = (Option < Gd < crate::engine::ArrayMesh > >, Gd < crate::engine::ArrayMesh >, u64);
            let args = (existing, flags,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8007usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "commit", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn commit(&mut self,) -> Option < Gd < crate::engine::ArrayMesh > > {
            self.commit_ex() . done()
        }
        #[inline]
        pub fn commit_ex(&mut self,) -> ExCommit < '_ > {
            ExCommit::new(self,)
        }
        pub fn commit_to_arrays(&mut self,) -> VariantArray {
            type RetMarshal = PtrcallReturnT < VariantArray >;
            type CallSig = (VariantArray,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8008usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "commit_to_arrays", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for SurfaceTool {
        type Base = crate::engine::RefCounted;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"SurfaceTool\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for SurfaceTool {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for SurfaceTool {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::RefCounted > for SurfaceTool {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for SurfaceTool {
        
    }
    impl crate::obj::cap::GodotDefault for SurfaceTool {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for SurfaceTool {
        type Target = crate::engine::RefCounted;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for SurfaceTool {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_SurfaceTool {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::SurfaceTool > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::RefCounted > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`SurfaceTool::add_triangle_fan_ex`][super::SurfaceTool::add_triangle_fan_ex]."]
#[must_use]
pub struct ExAddTriangleFan < 'a > {
    surround_object: &'a mut re_export::SurfaceTool, vertices: PackedVector3Array, uvs: PackedVector2Array, colors: PackedColorArray, uv2s: PackedVector2Array, normals: PackedVector3Array, tangents: Array < Plane >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddTriangleFan < 'a > {
    fn new(surround_object: &'a mut re_export::SurfaceTool, vertices: PackedVector3Array,) -> Self {
        Self {
            surround_object, vertices, uvs: PackedVector2Array::new(), colors: PackedColorArray::new(), uv2s: PackedVector2Array::new(), normals: PackedVector3Array::new(), tangents: Array::new(),
        }
    }
    #[inline]
    pub fn uvs(self, value: PackedVector2Array) -> Self {
        Self {
            uvs: value, .. self
        }
    }
    #[inline]
    pub fn colors(self, value: PackedColorArray) -> Self {
        Self {
            colors: value, .. self
        }
    }
    #[inline]
    pub fn uv2s(self, value: PackedVector2Array) -> Self {
        Self {
            uv2s: value, .. self
        }
    }
    #[inline]
    pub fn normals(self, value: PackedVector3Array) -> Self {
        Self {
            normals: value, .. self
        }
    }
    #[inline]
    pub fn tangents(self, value: Array < Plane >) -> Self {
        Self {
            tangents: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::SurfaceTool::add_triangle_fan_full(self.surround_object, self.vertices, self.uvs, self.colors, self.uv2s, self.normals, self.tangents,)
    }
}
#[doc = "Default-param extender for [`SurfaceTool::generate_normals_ex`][super::SurfaceTool::generate_normals_ex]."]
#[must_use]
pub struct ExGenerateNormals < 'a > {
    surround_object: &'a mut re_export::SurfaceTool, flip: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGenerateNormals < 'a > {
    fn new(surround_object: &'a mut re_export::SurfaceTool,) -> Self {
        Self {
            surround_object, flip: false,
        }
    }
    #[inline]
    pub fn flip(self, value: bool) -> Self {
        Self {
            flip: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::SurfaceTool::generate_normals_full(self.surround_object, self.flip,)
    }
}
#[doc = "Default-param extender for [`SurfaceTool::generate_lod_ex`][super::SurfaceTool::generate_lod_ex]."]
#[must_use]
pub struct ExGenerateLod < 'a > {
    surround_object: &'a mut re_export::SurfaceTool, nd_threshold: f32, target_index_count: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGenerateLod < 'a > {
    fn new(surround_object: &'a mut re_export::SurfaceTool, nd_threshold: f32,) -> Self {
        Self {
            surround_object, nd_threshold, target_index_count: 3i32,
        }
    }
    #[inline]
    pub fn target_index_count(self, value: i32) -> Self {
        Self {
            target_index_count: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> PackedInt32Array {
        re_export::SurfaceTool::generate_lod_full(self.surround_object, self.nd_threshold, self.target_index_count,)
    }
}
#[doc = "Default-param extender for [`SurfaceTool::commit_ex`][super::SurfaceTool::commit_ex]."]
#[must_use]
pub struct ExCommit < 'a > {
    surround_object: &'a mut re_export::SurfaceTool, existing: Gd < crate::engine::ArrayMesh >, flags: u64,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExCommit < 'a > {
    fn new(surround_object: &'a mut re_export::SurfaceTool,) -> Self {
        Self {
            surround_object, existing: unimplemented !("see #156"), flags: 0u64,
        }
    }
    #[inline]
    pub fn existing(self, value: Gd < crate::engine::ArrayMesh >) -> Self {
        Self {
            existing: value, .. self
        }
    }
    #[inline]
    pub fn flags(self, value: u64) -> Self {
        Self {
            flags: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::engine::ArrayMesh > > {
        re_export::SurfaceTool::commit_full(self.surround_object, self.existing, self.flags,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct CustomFormat {
    ord: i32
}
impl CustomFormat {
    pub const CUSTOM_RGBA8_UNORM: Self = Self {
        ord: 0i32
    };
    pub const CUSTOM_RGBA8_SNORM: Self = Self {
        ord: 1i32
    };
    pub const CUSTOM_RG_HALF: Self = Self {
        ord: 2i32
    };
    pub const CUSTOM_RGBA_HALF: Self = Self {
        ord: 3i32
    };
    pub const CUSTOM_R_FLOAT: Self = Self {
        ord: 4i32
    };
    pub const CUSTOM_RG_FLOAT: Self = Self {
        ord: 5i32
    };
    pub const CUSTOM_RGB_FLOAT: Self = Self {
        ord: 6i32
    };
    pub const CUSTOM_RGBA_FLOAT: Self = Self {
        ord: 7i32
    };
    pub const CUSTOM_MAX: Self = Self {
        ord: 8i32
    };
    
}
impl crate::obj::EngineEnum for CustomFormat {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 => Some(Self {
                ord
            }), _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
impl crate::obj::IndexEnum for CustomFormat {
    const ENUMERATOR_COUNT: usize = 8usize;
    
}
impl crate::builtin::meta::GodotConvert for CustomFormat {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for CustomFormat {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for CustomFormat {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct SkinWeightCount {
    ord: i32
}
impl SkinWeightCount {
    pub const SKIN_4_WEIGHTS: Self = Self {
        ord: 0i32
    };
    pub const SKIN_8_WEIGHTS: Self = Self {
        ord: 1i32
    };
    
}
impl crate::obj::EngineEnum for SkinWeightCount {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 => Some(Self {
                ord
            }), _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
impl crate::builtin::meta::GodotConvert for SkinWeightCount {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for SkinWeightCount {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for SkinWeightCount {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}