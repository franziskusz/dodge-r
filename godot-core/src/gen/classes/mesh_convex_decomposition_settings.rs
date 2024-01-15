#![doc = "Sidecar module for class [`MeshConvexDecompositionSettings`][crate::engine::MeshConvexDecompositionSettings].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `MeshConvexDecompositionSettings` enums](https://docs.godotengine.org/en/stable/classes/class_meshconvexdecompositionsettings.html#enumerations).\n\n"]
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
    #[doc = "Godot class `MeshConvexDecompositionSettings.`\n\nInherits [`RefCounted`][crate::engine::RefCounted].\n\nRelated symbols:\n\n* [`mesh_convex_decomposition_settings`][crate::engine::mesh_convex_decomposition_settings]: sidecar module with related enum/flag types\n* [`IMeshConvexDecompositionSettings`][crate::engine::IMeshConvexDecompositionSettings]: virtual methods\n\n\nSee also [Godot docs for `MeshConvexDecompositionSettings`](https://docs.godotengine.org/en/stable/classes/class_meshconvexdecompositionsettings.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct MeshConvexDecompositionSettings {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`MeshConvexDecompositionSettings`][crate::engine::MeshConvexDecompositionSettings].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `MeshConvexDecompositionSettings` methods](https://docs.godotengine.org/en/stable/classes/class_meshconvexdecompositionsettings.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IMeshConvexDecompositionSettings: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
    impl MeshConvexDecompositionSettings {
        #[deprecated = "Replaced with `new_gd` in extension trait `NewGd`."]
        pub fn new() -> Gd < Self > {
            crate::obj::Gd::default()
        }
        pub fn set_max_concavity(&mut self, max_concavity: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (max_concavity,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4635usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_max_concavity", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_max_concavity(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4636usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_max_concavity", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_symmetry_planes_clipping_bias(&mut self, symmetry_planes_clipping_bias: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (symmetry_planes_clipping_bias,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4637usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_symmetry_planes_clipping_bias", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_symmetry_planes_clipping_bias(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4638usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_symmetry_planes_clipping_bias", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_revolution_axes_clipping_bias(&mut self, revolution_axes_clipping_bias: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (revolution_axes_clipping_bias,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4639usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_revolution_axes_clipping_bias", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_revolution_axes_clipping_bias(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4640usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_revolution_axes_clipping_bias", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_min_volume_per_convex_hull(&mut self, min_volume_per_convex_hull: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (min_volume_per_convex_hull,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4641usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_min_volume_per_convex_hull", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_min_volume_per_convex_hull(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4642usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_min_volume_per_convex_hull", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_resolution(&mut self, min_volume_per_convex_hull: u32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), u32);
            let args = (min_volume_per_convex_hull,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4643usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_resolution", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_resolution(&self,) -> u32 {
            type RetMarshal = PtrcallReturnT < u32 >;
            type CallSig = (u32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4644usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_resolution", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_max_num_vertices_per_convex_hull(&mut self, max_num_vertices_per_convex_hull: u32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), u32);
            let args = (max_num_vertices_per_convex_hull,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4645usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_max_num_vertices_per_convex_hull", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_max_num_vertices_per_convex_hull(&self,) -> u32 {
            type RetMarshal = PtrcallReturnT < u32 >;
            type CallSig = (u32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4646usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_max_num_vertices_per_convex_hull", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_plane_downsampling(&mut self, plane_downsampling: u32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), u32);
            let args = (plane_downsampling,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4647usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_plane_downsampling", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_plane_downsampling(&self,) -> u32 {
            type RetMarshal = PtrcallReturnT < u32 >;
            type CallSig = (u32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4648usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_plane_downsampling", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_convex_hull_downsampling(&mut self, convex_hull_downsampling: u32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), u32);
            let args = (convex_hull_downsampling,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4649usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_convex_hull_downsampling", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_convex_hull_downsampling(&self,) -> u32 {
            type RetMarshal = PtrcallReturnT < u32 >;
            type CallSig = (u32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4650usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_convex_hull_downsampling", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_normalize_mesh(&mut self, normalize_mesh: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (normalize_mesh,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4651usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_normalize_mesh", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_normalize_mesh(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4652usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_normalize_mesh", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_mode(&mut self, mode: crate::engine::mesh_convex_decomposition_settings::Mode,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::mesh_convex_decomposition_settings::Mode);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4653usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_mode(&self,) -> crate::engine::mesh_convex_decomposition_settings::Mode {
            type RetMarshal = PtrcallReturnT < crate::engine::mesh_convex_decomposition_settings::Mode >;
            type CallSig = (crate::engine::mesh_convex_decomposition_settings::Mode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4654usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_convex_hull_approximation(&mut self, convex_hull_approximation: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (convex_hull_approximation,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4655usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_convex_hull_approximation", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_convex_hull_approximation(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4656usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_convex_hull_approximation", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_max_convex_hulls(&mut self, max_convex_hulls: u32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), u32);
            let args = (max_convex_hulls,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4657usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_max_convex_hulls", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_max_convex_hulls(&self,) -> u32 {
            type RetMarshal = PtrcallReturnT < u32 >;
            type CallSig = (u32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4658usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_max_convex_hulls", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_project_hull_vertices(&mut self, project_hull_vertices: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (project_hull_vertices,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4659usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_project_hull_vertices", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_project_hull_vertices(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4660usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_project_hull_vertices", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for MeshConvexDecompositionSettings {
        type Base = crate::engine::RefCounted;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"MeshConvexDecompositionSettings\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for MeshConvexDecompositionSettings {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for MeshConvexDecompositionSettings {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::RefCounted > for MeshConvexDecompositionSettings {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for MeshConvexDecompositionSettings {
        
    }
    impl crate::obj::cap::GodotDefault for MeshConvexDecompositionSettings {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for MeshConvexDecompositionSettings {
        type Target = crate::engine::RefCounted;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for MeshConvexDecompositionSettings {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_MeshConvexDecompositionSettings {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::MeshConvexDecompositionSettings > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::RefCounted > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::Object > for $Class {
                
            }
        }
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Mode {
    ord: i32
}
impl Mode {
    pub const CONVEX_DECOMPOSITION_MODE_VOXEL: Self = Self {
        ord: 0i32
    };
    pub const CONVEX_DECOMPOSITION_MODE_TETRAHEDRON: Self = Self {
        ord: 1i32
    };
    
}
impl crate::obj::EngineEnum for Mode {
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
impl crate::builtin::meta::GodotConvert for Mode {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for Mode {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for Mode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}