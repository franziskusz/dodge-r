#![doc = "Sidecar module for class [`MeshInstance3D`][crate::engine::MeshInstance3D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `MeshInstance3D` enums](https://docs.godotengine.org/en/stable/classes/class_meshinstance3d.html#enumerations).\n\n"]
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
    #[doc = "Godot class `MeshInstance3D.`\n\nInherits [`GeometryInstance3D`][crate::engine::GeometryInstance3D].\n\nRelated symbols:\n\n* [`mesh_instance_3d`][crate::engine::mesh_instance_3d]: sidecar module with related enum/flag types\n* [`IMeshInstance3D`][crate::engine::IMeshInstance3D]: virtual methods\n\n\nSee also [Godot docs for `MeshInstance3D`](https://docs.godotengine.org/en/stable/classes/class_meshinstance3d.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct MeshInstance3D {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`MeshInstance3D`][crate::engine::MeshInstance3D].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `MeshInstance3D` methods](https://docs.godotengine.org/en/stable/classes/class_meshinstance3d.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IMeshInstance3D: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
        fn on_notification(&mut self, what: Node3DNotification) {
            unimplemented !()
        }
        fn get_aabb(&self,) -> Aabb {
            unimplemented !()
        }
        fn process(&mut self, delta: f64,) {
            unimplemented !()
        }
        fn physics_process(&mut self, delta: f64,) {
            unimplemented !()
        }
        fn enter_tree(&mut self,) {
            unimplemented !()
        }
        fn exit_tree(&mut self,) {
            unimplemented !()
        }
        fn ready(&mut self,) {
            unimplemented !()
        }
        fn get_configuration_warnings(&self,) -> PackedStringArray {
            unimplemented !()
        }
        fn input(&mut self, event: Gd < crate::engine::InputEvent >,) {
            unimplemented !()
        }
        fn shortcut_input(&mut self, event: Gd < crate::engine::InputEvent >,) {
            unimplemented !()
        }
        fn unhandled_input(&mut self, event: Gd < crate::engine::InputEvent >,) {
            unimplemented !()
        }
        fn unhandled_key_input(&mut self, event: Gd < crate::engine::InputEvent >,) {
            unimplemented !()
        }
    }
    impl MeshInstance3D {
        pub fn set_mesh(&mut self, mesh: Gd < crate::engine::Mesh >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::Mesh >);
            let args = (mesh,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4703usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_mesh", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_mesh(&self,) -> Option < Gd < crate::engine::Mesh > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::Mesh > >;
            type CallSig = (Option < Gd < crate::engine::Mesh > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4704usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_mesh", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_skeleton_path(&mut self, skeleton_path: NodePath,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), NodePath);
            let args = (skeleton_path,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4705usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_skeleton_path", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_skeleton_path(&mut self,) -> NodePath {
            type RetMarshal = PtrcallReturnT < NodePath >;
            type CallSig = (NodePath,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4706usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_skeleton_path", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_skin(&mut self, skin: Gd < crate::engine::Skin >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::Skin >);
            let args = (skin,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4707usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_skin", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_skin(&self,) -> Option < Gd < crate::engine::Skin > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::Skin > >;
            type CallSig = (Option < Gd < crate::engine::Skin > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4708usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_skin", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_surface_override_material_count(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4709usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_surface_override_material_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_surface_override_material(&mut self, surface: i32, material: Gd < crate::engine::Material >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, Gd < crate::engine::Material >);
            let args = (surface, material,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4710usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_surface_override_material", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_surface_override_material(&self, surface: i32,) -> Option < Gd < crate::engine::Material > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::Material > >;
            type CallSig = (Option < Gd < crate::engine::Material > >, i32);
            let args = (surface,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4711usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_surface_override_material", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_active_material(&self, surface: i32,) -> Option < Gd < crate::engine::Material > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::Material > >;
            type CallSig = (Option < Gd < crate::engine::Material > >, i32);
            let args = (surface,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4712usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_active_material", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn create_trimesh_collision(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4713usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "create_trimesh_collision", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn create_convex_collision_full(&mut self, clean: bool, simplify: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool, bool);
            let args = (clean, simplify,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4714usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "create_convex_collision", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn create_convex_collision(&mut self,) {
            self.create_convex_collision_ex() . done()
        }
        #[inline]
        pub fn create_convex_collision_ex(&mut self,) -> ExCreateConvexCollision < '_ > {
            ExCreateConvexCollision::new(self,)
        }
        pub(crate) fn create_multiple_convex_collisions_full(&mut self, settings: Gd < crate::engine::MeshConvexDecompositionSettings >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::MeshConvexDecompositionSettings >);
            let args = (settings,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4715usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "create_multiple_convex_collisions", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn create_multiple_convex_collisions(&mut self,) {
            self.create_multiple_convex_collisions_ex() . done()
        }
        #[inline]
        pub fn create_multiple_convex_collisions_ex(&mut self,) -> ExCreateMultipleConvexCollisions < '_ > {
            ExCreateMultipleConvexCollisions::new(self,)
        }
        pub fn get_blend_shape_count(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4716usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_blend_shape_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn find_blend_shape_by_name(&mut self, name: StringName,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, StringName);
            let args = (name,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4717usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "find_blend_shape_by_name", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_blend_shape_value(&self, blend_shape_idx: i32,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32, i32);
            let args = (blend_shape_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4718usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_blend_shape_value", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_blend_shape_value(&mut self, blend_shape_idx: i32, value: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, f32);
            let args = (blend_shape_idx, value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4719usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_blend_shape_value", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn create_debug_tangents(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4720usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "create_debug_tangents", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for MeshInstance3D {
        type Base = crate::engine::GeometryInstance3D;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"MeshInstance3D\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for MeshInstance3D {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for MeshInstance3D {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::GeometryInstance3D > for MeshInstance3D {
        
    }
    impl crate::obj::Inherits < crate::engine::VisualInstance3D > for MeshInstance3D {
        
    }
    impl crate::obj::Inherits < crate::engine::Node3D > for MeshInstance3D {
        
    }
    impl crate::obj::Inherits < crate::engine::Node > for MeshInstance3D {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for MeshInstance3D {
        
    }
    impl crate::obj::ExportableObject for MeshInstance3D {
        
    }
    impl crate::obj::cap::GodotDefault for MeshInstance3D {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for MeshInstance3D {
        type Target = crate::engine::GeometryInstance3D;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for MeshInstance3D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_MeshInstance3D {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::MeshInstance3D > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::GeometryInstance3D > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::VisualInstance3D > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::Node3D > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::Node > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::Object > for $Class {
                
            }
            impl::godot::obj::ExportableObject for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`MeshInstance3D::create_convex_collision_ex`][super::MeshInstance3D::create_convex_collision_ex]."]
#[must_use]
pub struct ExCreateConvexCollision < 'a > {
    surround_object: &'a mut re_export::MeshInstance3D, clean: bool, simplify: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExCreateConvexCollision < 'a > {
    fn new(surround_object: &'a mut re_export::MeshInstance3D,) -> Self {
        Self {
            surround_object, clean: true, simplify: false,
        }
    }
    #[inline]
    pub fn clean(self, value: bool) -> Self {
        Self {
            clean: value, .. self
        }
    }
    #[inline]
    pub fn simplify(self, value: bool) -> Self {
        Self {
            simplify: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::MeshInstance3D::create_convex_collision_full(self.surround_object, self.clean, self.simplify,)
    }
}
#[doc = "Default-param extender for [`MeshInstance3D::create_multiple_convex_collisions_ex`][super::MeshInstance3D::create_multiple_convex_collisions_ex]."]
#[must_use]
pub struct ExCreateMultipleConvexCollisions < 'a > {
    surround_object: &'a mut re_export::MeshInstance3D, settings: Gd < crate::engine::MeshConvexDecompositionSettings >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExCreateMultipleConvexCollisions < 'a > {
    fn new(surround_object: &'a mut re_export::MeshInstance3D,) -> Self {
        Self {
            surround_object, settings: unimplemented !("see #156"),
        }
    }
    #[inline]
    pub fn settings(self, value: Gd < crate::engine::MeshConvexDecompositionSettings >) -> Self {
        Self {
            settings: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::MeshInstance3D::create_multiple_convex_collisions_full(self.surround_object, self.settings,)
    }
}