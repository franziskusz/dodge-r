#![doc = "Sidecar module for class [`Geometry3D`][crate::engine::Geometry3D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `Geometry3D` enums](https://docs.godotengine.org/en/stable/classes/class_geometry3d.html#enumerations).\n\n"]
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
    #[doc = "Godot class `Geometry3D.`\n\nInherits [`Object`][crate::engine::Object].\n\nRelated symbols:\n\n* [`geometry_3d`][crate::engine::geometry_3d]: sidecar module with related enum/flag types\n* [`IGeometry3D`][crate::engine::IGeometry3D]: virtual methods\n\n\nSee also [Godot docs for `Geometry3D`](https://docs.godotengine.org/en/stable/classes/class_geometry3d.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct Geometry3D {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`Geometry3D`][crate::engine::Geometry3D].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `Geometry3D` methods](https://docs.godotengine.org/en/stable/classes/class_geometry3d.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IGeometry3D: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
    impl Geometry3D {
        pub fn singleton() -> Gd < Self > {
            unsafe {
                let __class_name = StringName::from_latin1_with_nul(b"Geometry3D\0");
                let __object_ptr = sys::interface_fn !(global_get_singleton) (__class_name.string_sys());
                Gd::from_obj_sys(__object_ptr)
            }
        }
        pub fn compute_convex_mesh_points(&mut self, planes: Array < Plane >,) -> PackedVector3Array {
            type RetMarshal = PtrcallReturnT < PackedVector3Array >;
            type CallSig = (PackedVector3Array, Array < Plane >);
            let args = (planes,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3545usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "compute_convex_mesh_points", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn build_box_planes(&mut self, extents: Vector3,) -> Array < Plane > {
            type RetMarshal = PtrcallReturnT < Array < Plane > >;
            type CallSig = (Array < Plane >, Vector3);
            let args = (extents,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3546usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "build_box_planes", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn build_cylinder_planes_full(&mut self, radius: f32, height: f32, sides: i32, axis: Vector3Axis,) -> Array < Plane > {
            type RetMarshal = PtrcallReturnT < Array < Plane > >;
            type CallSig = (Array < Plane >, f32, f32, i32, Vector3Axis);
            let args = (radius, height, sides, axis,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3547usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "build_cylinder_planes", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn build_cylinder_planes(&mut self, radius: f32, height: f32, sides: i32,) -> Array < Plane > {
            self.build_cylinder_planes_ex(radius, height, sides,) . done()
        }
        #[inline]
        pub fn build_cylinder_planes_ex(&mut self, radius: f32, height: f32, sides: i32,) -> ExBuildCylinderPlanes < '_ > {
            ExBuildCylinderPlanes::new(self, radius, height, sides,)
        }
        pub(crate) fn build_capsule_planes_full(&mut self, radius: f32, height: f32, sides: i32, lats: i32, axis: Vector3Axis,) -> Array < Plane > {
            type RetMarshal = PtrcallReturnT < Array < Plane > >;
            type CallSig = (Array < Plane >, f32, f32, i32, i32, Vector3Axis);
            let args = (radius, height, sides, lats, axis,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3548usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "build_capsule_planes", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn build_capsule_planes(&mut self, radius: f32, height: f32, sides: i32, lats: i32,) -> Array < Plane > {
            self.build_capsule_planes_ex(radius, height, sides, lats,) . done()
        }
        #[inline]
        pub fn build_capsule_planes_ex(&mut self, radius: f32, height: f32, sides: i32, lats: i32,) -> ExBuildCapsulePlanes < '_ > {
            ExBuildCapsulePlanes::new(self, radius, height, sides, lats,)
        }
        pub fn get_closest_points_between_segments(&mut self, p1: Vector3, p2: Vector3, q1: Vector3, q2: Vector3,) -> PackedVector3Array {
            type RetMarshal = PtrcallReturnT < PackedVector3Array >;
            type CallSig = (PackedVector3Array, Vector3, Vector3, Vector3, Vector3);
            let args = (p1, p2, q1, q2,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3549usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_closest_points_between_segments", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_closest_point_to_segment(&mut self, point: Vector3, s1: Vector3, s2: Vector3,) -> Vector3 {
            type RetMarshal = PtrcallReturnT < Vector3 >;
            type CallSig = (Vector3, Vector3, Vector3, Vector3);
            let args = (point, s1, s2,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3550usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_closest_point_to_segment", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_closest_point_to_segment_uncapped(&mut self, point: Vector3, s1: Vector3, s2: Vector3,) -> Vector3 {
            type RetMarshal = PtrcallReturnT < Vector3 >;
            type CallSig = (Vector3, Vector3, Vector3, Vector3);
            let args = (point, s1, s2,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3551usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_closest_point_to_segment_uncapped", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_triangle_barycentric_coords(&mut self, point: Vector3, a: Vector3, b: Vector3, c: Vector3,) -> Vector3 {
            type RetMarshal = PtrcallReturnT < Vector3 >;
            type CallSig = (Vector3, Vector3, Vector3, Vector3, Vector3);
            let args = (point, a, b, c,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3552usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_triangle_barycentric_coords", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn ray_intersects_triangle(&mut self, from: Vector3, dir: Vector3, a: Vector3, b: Vector3, c: Vector3,) -> Variant {
            type RetMarshal = PtrcallReturnT < Variant >;
            type CallSig = (Variant, Vector3, Vector3, Vector3, Vector3, Vector3);
            let args = (from, dir, a, b, c,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3553usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "ray_intersects_triangle", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn segment_intersects_triangle(&mut self, from: Vector3, to: Vector3, a: Vector3, b: Vector3, c: Vector3,) -> Variant {
            type RetMarshal = PtrcallReturnT < Variant >;
            type CallSig = (Variant, Vector3, Vector3, Vector3, Vector3, Vector3);
            let args = (from, to, a, b, c,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3554usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "segment_intersects_triangle", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn segment_intersects_sphere(&mut self, from: Vector3, to: Vector3, sphere_position: Vector3, sphere_radius: f32,) -> PackedVector3Array {
            type RetMarshal = PtrcallReturnT < PackedVector3Array >;
            type CallSig = (PackedVector3Array, Vector3, Vector3, Vector3, f32);
            let args = (from, to, sphere_position, sphere_radius,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3555usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "segment_intersects_sphere", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn segment_intersects_cylinder(&mut self, from: Vector3, to: Vector3, height: f32, radius: f32,) -> PackedVector3Array {
            type RetMarshal = PtrcallReturnT < PackedVector3Array >;
            type CallSig = (PackedVector3Array, Vector3, Vector3, f32, f32);
            let args = (from, to, height, radius,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3556usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "segment_intersects_cylinder", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn segment_intersects_convex(&mut self, from: Vector3, to: Vector3, planes: Array < Plane >,) -> PackedVector3Array {
            type RetMarshal = PtrcallReturnT < PackedVector3Array >;
            type CallSig = (PackedVector3Array, Vector3, Vector3, Array < Plane >);
            let args = (from, to, planes,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3557usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "segment_intersects_convex", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn clip_polygon(&mut self, points: PackedVector3Array, plane: Plane,) -> PackedVector3Array {
            type RetMarshal = PtrcallReturnT < PackedVector3Array >;
            type CallSig = (PackedVector3Array, PackedVector3Array, Plane);
            let args = (points, plane,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3558usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "clip_polygon", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for Geometry3D {
        type Base = crate::engine::Object;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"Geometry3D\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for Geometry3D {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for Geometry3D {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::Object > for Geometry3D {
        
    }
    impl std::ops::Deref for Geometry3D {
        type Target = crate::engine::Object;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for Geometry3D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_Geometry3D {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::Geometry3D > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`Geometry3D::build_cylinder_planes_ex`][super::Geometry3D::build_cylinder_planes_ex]."]
#[must_use]
pub struct ExBuildCylinderPlanes < 'a > {
    surround_object: &'a mut re_export::Geometry3D, radius: f32, height: f32, sides: i32, axis: Vector3Axis,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExBuildCylinderPlanes < 'a > {
    fn new(surround_object: &'a mut re_export::Geometry3D, radius: f32, height: f32, sides: i32,) -> Self {
        Self {
            surround_object, radius, height, sides, axis: crate::obj::EngineEnum::from_ord(2),
        }
    }
    #[inline]
    pub fn axis(self, value: Vector3Axis) -> Self {
        Self {
            axis: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Array < Plane > {
        re_export::Geometry3D::build_cylinder_planes_full(self.surround_object, self.radius, self.height, self.sides, self.axis,)
    }
}
#[doc = "Default-param extender for [`Geometry3D::build_capsule_planes_ex`][super::Geometry3D::build_capsule_planes_ex]."]
#[must_use]
pub struct ExBuildCapsulePlanes < 'a > {
    surround_object: &'a mut re_export::Geometry3D, radius: f32, height: f32, sides: i32, lats: i32, axis: Vector3Axis,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExBuildCapsulePlanes < 'a > {
    fn new(surround_object: &'a mut re_export::Geometry3D, radius: f32, height: f32, sides: i32, lats: i32,) -> Self {
        Self {
            surround_object, radius, height, sides, lats, axis: crate::obj::EngineEnum::from_ord(2),
        }
    }
    #[inline]
    pub fn axis(self, value: Vector3Axis) -> Self {
        Self {
            axis: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Array < Plane > {
        re_export::Geometry3D::build_capsule_planes_full(self.surround_object, self.radius, self.height, self.sides, self.lats, self.axis,)
    }
}