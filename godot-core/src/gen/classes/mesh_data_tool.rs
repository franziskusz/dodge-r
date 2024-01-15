#![doc = "Sidecar module for class [`MeshDataTool`][crate::engine::MeshDataTool].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `MeshDataTool` enums](https://docs.godotengine.org/en/stable/classes/class_meshdatatool.html#enumerations).\n\n"]
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
    #[doc = "Godot class `MeshDataTool.`\n\nInherits [`RefCounted`][crate::engine::RefCounted].\n\nRelated symbols:\n\n* [`mesh_data_tool`][crate::engine::mesh_data_tool]: sidecar module with related enum/flag types\n* [`IMeshDataTool`][crate::engine::IMeshDataTool]: virtual methods\n\n\nSee also [Godot docs for `MeshDataTool`](https://docs.godotengine.org/en/stable/classes/class_meshdatatool.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct MeshDataTool {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`MeshDataTool`][crate::engine::MeshDataTool].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `MeshDataTool` methods](https://docs.godotengine.org/en/stable/classes/class_meshdatatool.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IMeshDataTool: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
    impl MeshDataTool {
        #[deprecated = "Replaced with `new_gd` in extension trait `NewGd`."]
        pub fn new() -> Gd < Self > {
            crate::obj::Gd::default()
        }
        pub fn clear(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4661usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "clear", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn create_from_surface(&mut self, mesh: Gd < crate::engine::ArrayMesh >, surface: i32,) -> crate::engine::global::Error {
            type RetMarshal = PtrcallReturnT < crate::engine::global::Error >;
            type CallSig = (crate::engine::global::Error, Gd < crate::engine::ArrayMesh >, i32);
            let args = (mesh, surface,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4662usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "create_from_surface", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn commit_to_surface_full(&mut self, mesh: Gd < crate::engine::ArrayMesh >, compression_flags: u64,) -> crate::engine::global::Error {
            type RetMarshal = PtrcallReturnT < crate::engine::global::Error >;
            type CallSig = (crate::engine::global::Error, Gd < crate::engine::ArrayMesh >, u64);
            let args = (mesh, compression_flags,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4663usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "commit_to_surface", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn commit_to_surface(&mut self, mesh: Gd < crate::engine::ArrayMesh >,) -> crate::engine::global::Error {
            self.commit_to_surface_ex(mesh,) . done()
        }
        #[inline]
        pub fn commit_to_surface_ex(&mut self, mesh: Gd < crate::engine::ArrayMesh >,) -> ExCommitToSurface < '_ > {
            ExCommitToSurface::new(self, mesh,)
        }
        pub fn get_format(&self,) -> u64 {
            type RetMarshal = PtrcallReturnT < u64 >;
            type CallSig = (u64,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4664usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_format", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_vertex_count(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4665usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_vertex_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_edge_count(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4666usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_edge_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_face_count(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4667usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_face_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_vertex(&mut self, idx: i32, vertex: Vector3,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, Vector3);
            let args = (idx, vertex,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4668usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_vertex", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_vertex(&self, idx: i32,) -> Vector3 {
            type RetMarshal = PtrcallReturnT < Vector3 >;
            type CallSig = (Vector3, i32);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4669usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_vertex", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_vertex_normal(&mut self, idx: i32, normal: Vector3,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, Vector3);
            let args = (idx, normal,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4670usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_vertex_normal", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_vertex_normal(&self, idx: i32,) -> Vector3 {
            type RetMarshal = PtrcallReturnT < Vector3 >;
            type CallSig = (Vector3, i32);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4671usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_vertex_normal", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_vertex_tangent(&mut self, idx: i32, tangent: Plane,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, Plane);
            let args = (idx, tangent,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4672usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_vertex_tangent", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_vertex_tangent(&self, idx: i32,) -> Plane {
            type RetMarshal = PtrcallReturnT < Plane >;
            type CallSig = (Plane, i32);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4673usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_vertex_tangent", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_vertex_uv(&mut self, idx: i32, uv: Vector2,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, Vector2);
            let args = (idx, uv,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4674usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_vertex_uv", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_vertex_uv(&self, idx: i32,) -> Vector2 {
            type RetMarshal = PtrcallReturnT < Vector2 >;
            type CallSig = (Vector2, i32);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4675usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_vertex_uv", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_vertex_uv2(&mut self, idx: i32, uv2: Vector2,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, Vector2);
            let args = (idx, uv2,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4676usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_vertex_uv2", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_vertex_uv2(&self, idx: i32,) -> Vector2 {
            type RetMarshal = PtrcallReturnT < Vector2 >;
            type CallSig = (Vector2, i32);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4677usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_vertex_uv2", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_vertex_color(&mut self, idx: i32, color: Color,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, Color);
            let args = (idx, color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4678usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_vertex_color", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_vertex_color(&self, idx: i32,) -> Color {
            type RetMarshal = PtrcallReturnT < Color >;
            type CallSig = (Color, i32);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4679usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_vertex_color", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_vertex_bones(&mut self, idx: i32, bones: PackedInt32Array,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, PackedInt32Array);
            let args = (idx, bones,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4680usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_vertex_bones", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_vertex_bones(&self, idx: i32,) -> PackedInt32Array {
            type RetMarshal = PtrcallReturnT < PackedInt32Array >;
            type CallSig = (PackedInt32Array, i32);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4681usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_vertex_bones", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_vertex_weights(&mut self, idx: i32, weights: PackedFloat32Array,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, PackedFloat32Array);
            let args = (idx, weights,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4682usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_vertex_weights", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_vertex_weights(&self, idx: i32,) -> PackedFloat32Array {
            type RetMarshal = PtrcallReturnT < PackedFloat32Array >;
            type CallSig = (PackedFloat32Array, i32);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4683usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_vertex_weights", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_vertex_meta(&mut self, idx: i32, meta: Variant,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, Variant);
            let args = (idx, meta,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4684usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_vertex_meta", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_vertex_meta(&self, idx: i32,) -> Variant {
            type RetMarshal = PtrcallReturnT < Variant >;
            type CallSig = (Variant, i32);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4685usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_vertex_meta", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_vertex_edges(&self, idx: i32,) -> PackedInt32Array {
            type RetMarshal = PtrcallReturnT < PackedInt32Array >;
            type CallSig = (PackedInt32Array, i32);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4686usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_vertex_edges", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_vertex_faces(&self, idx: i32,) -> PackedInt32Array {
            type RetMarshal = PtrcallReturnT < PackedInt32Array >;
            type CallSig = (PackedInt32Array, i32);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4687usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_vertex_faces", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_edge_vertex(&self, idx: i32, vertex: i32,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, i32, i32);
            let args = (idx, vertex,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4688usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_edge_vertex", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_edge_faces(&self, idx: i32,) -> PackedInt32Array {
            type RetMarshal = PtrcallReturnT < PackedInt32Array >;
            type CallSig = (PackedInt32Array, i32);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4689usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_edge_faces", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_edge_meta(&mut self, idx: i32, meta: Variant,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, Variant);
            let args = (idx, meta,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4690usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_edge_meta", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_edge_meta(&self, idx: i32,) -> Variant {
            type RetMarshal = PtrcallReturnT < Variant >;
            type CallSig = (Variant, i32);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4691usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_edge_meta", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_face_vertex(&self, idx: i32, vertex: i32,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, i32, i32);
            let args = (idx, vertex,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4692usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_face_vertex", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_face_edge(&self, idx: i32, edge: i32,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, i32, i32);
            let args = (idx, edge,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4693usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_face_edge", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_face_meta(&mut self, idx: i32, meta: Variant,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, Variant);
            let args = (idx, meta,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4694usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_face_meta", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_face_meta(&self, idx: i32,) -> Variant {
            type RetMarshal = PtrcallReturnT < Variant >;
            type CallSig = (Variant, i32);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4695usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_face_meta", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_face_normal(&self, idx: i32,) -> Vector3 {
            type RetMarshal = PtrcallReturnT < Vector3 >;
            type CallSig = (Vector3, i32);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4696usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_face_normal", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_material(&mut self, material: Gd < crate::engine::Material >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::Material >);
            let args = (material,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4697usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_material", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_material(&self,) -> Option < Gd < crate::engine::Material > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::Material > >;
            type CallSig = (Option < Gd < crate::engine::Material > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4698usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_material", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for MeshDataTool {
        type Base = crate::engine::RefCounted;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"MeshDataTool\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for MeshDataTool {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for MeshDataTool {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::RefCounted > for MeshDataTool {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for MeshDataTool {
        
    }
    impl crate::obj::cap::GodotDefault for MeshDataTool {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for MeshDataTool {
        type Target = crate::engine::RefCounted;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for MeshDataTool {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_MeshDataTool {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::MeshDataTool > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::RefCounted > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`MeshDataTool::commit_to_surface_ex`][super::MeshDataTool::commit_to_surface_ex]."]
#[must_use]
pub struct ExCommitToSurface < 'a > {
    surround_object: &'a mut re_export::MeshDataTool, mesh: Gd < crate::engine::ArrayMesh >, compression_flags: u64,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExCommitToSurface < 'a > {
    fn new(surround_object: &'a mut re_export::MeshDataTool, mesh: Gd < crate::engine::ArrayMesh >,) -> Self {
        Self {
            surround_object, mesh, compression_flags: 0u64,
        }
    }
    #[inline]
    pub fn compression_flags(self, value: u64) -> Self {
        Self {
            compression_flags: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::engine::global::Error {
        re_export::MeshDataTool::commit_to_surface_full(self.surround_object, self.mesh, self.compression_flags,)
    }
}