#![doc = "Sidecar module for class [`Geometry2D`][crate::engine::Geometry2D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `Geometry2D` enums](https://docs.godotengine.org/en/stable/classes/class_geometry2d.html#enumerations).\n\n"]
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
    #[doc = "Godot class `Geometry2D.`\n\nInherits [`Object`][crate::engine::Object].\n\nRelated symbols:\n\n* [`geometry_2d`][crate::engine::geometry_2d]: sidecar module with related enum/flag types\n* [`IGeometry2D`][crate::engine::IGeometry2D]: virtual methods\n\n\nSee also [Godot docs for `Geometry2D`](https://docs.godotengine.org/en/stable/classes/class_geometry2d.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct Geometry2D {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`Geometry2D`][crate::engine::Geometry2D].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `Geometry2D` methods](https://docs.godotengine.org/en/stable/classes/class_geometry2d.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IGeometry2D: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
    impl Geometry2D {
        pub fn singleton() -> Gd < Self > {
            unsafe {
                let __class_name = StringName::from_latin1_with_nul(b"Geometry2D\0");
                let __object_ptr = sys::interface_fn !(global_get_singleton) (__class_name.string_sys());
                Gd::from_obj_sys(__object_ptr)
            }
        }
        pub fn is_point_in_circle(&mut self, point: Vector2, circle_position: Vector2, circle_radius: f32,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, Vector2, Vector2, f32);
            let args = (point, circle_position, circle_radius,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3522usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_point_in_circle", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn segment_intersects_circle(&mut self, segment_from: Vector2, segment_to: Vector2, circle_position: Vector2, circle_radius: f32,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32, Vector2, Vector2, Vector2, f32);
            let args = (segment_from, segment_to, circle_position, circle_radius,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3523usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "segment_intersects_circle", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn segment_intersects_segment(&mut self, from_a: Vector2, to_a: Vector2, from_b: Vector2, to_b: Vector2,) -> Variant {
            type RetMarshal = PtrcallReturnT < Variant >;
            type CallSig = (Variant, Vector2, Vector2, Vector2, Vector2);
            let args = (from_a, to_a, from_b, to_b,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3524usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "segment_intersects_segment", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn line_intersects_line(&mut self, from_a: Vector2, dir_a: Vector2, from_b: Vector2, dir_b: Vector2,) -> Variant {
            type RetMarshal = PtrcallReturnT < Variant >;
            type CallSig = (Variant, Vector2, Vector2, Vector2, Vector2);
            let args = (from_a, dir_a, from_b, dir_b,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3525usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "line_intersects_line", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_closest_points_between_segments(&mut self, p1: Vector2, q1: Vector2, p2: Vector2, q2: Vector2,) -> PackedVector2Array {
            type RetMarshal = PtrcallReturnT < PackedVector2Array >;
            type CallSig = (PackedVector2Array, Vector2, Vector2, Vector2, Vector2);
            let args = (p1, q1, p2, q2,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3526usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_closest_points_between_segments", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_closest_point_to_segment(&mut self, point: Vector2, s1: Vector2, s2: Vector2,) -> Vector2 {
            type RetMarshal = PtrcallReturnT < Vector2 >;
            type CallSig = (Vector2, Vector2, Vector2, Vector2);
            let args = (point, s1, s2,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3527usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_closest_point_to_segment", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_closest_point_to_segment_uncapped(&mut self, point: Vector2, s1: Vector2, s2: Vector2,) -> Vector2 {
            type RetMarshal = PtrcallReturnT < Vector2 >;
            type CallSig = (Vector2, Vector2, Vector2, Vector2);
            let args = (point, s1, s2,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3528usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_closest_point_to_segment_uncapped", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn point_is_inside_triangle(&self, point: Vector2, a: Vector2, b: Vector2, c: Vector2,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, Vector2, Vector2, Vector2, Vector2);
            let args = (point, a, b, c,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3529usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "point_is_inside_triangle", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_polygon_clockwise(&mut self, polygon: PackedVector2Array,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, PackedVector2Array);
            let args = (polygon,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3530usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_polygon_clockwise", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_point_in_polygon(&mut self, point: Vector2, polygon: PackedVector2Array,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, Vector2, PackedVector2Array);
            let args = (point, polygon,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3531usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_point_in_polygon", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn triangulate_polygon(&mut self, polygon: PackedVector2Array,) -> PackedInt32Array {
            type RetMarshal = PtrcallReturnT < PackedInt32Array >;
            type CallSig = (PackedInt32Array, PackedVector2Array);
            let args = (polygon,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3532usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "triangulate_polygon", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn triangulate_delaunay(&mut self, points: PackedVector2Array,) -> PackedInt32Array {
            type RetMarshal = PtrcallReturnT < PackedInt32Array >;
            type CallSig = (PackedInt32Array, PackedVector2Array);
            let args = (points,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3533usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "triangulate_delaunay", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn convex_hull(&mut self, points: PackedVector2Array,) -> PackedVector2Array {
            type RetMarshal = PtrcallReturnT < PackedVector2Array >;
            type CallSig = (PackedVector2Array, PackedVector2Array);
            let args = (points,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3534usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "convex_hull", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn decompose_polygon_in_convex(&mut self, polygon: PackedVector2Array,) -> Array < PackedVector2Array > {
            type RetMarshal = PtrcallReturnT < Array < PackedVector2Array > >;
            type CallSig = (Array < PackedVector2Array >, PackedVector2Array);
            let args = (polygon,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3535usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "decompose_polygon_in_convex", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn merge_polygons(&mut self, polygon_a: PackedVector2Array, polygon_b: PackedVector2Array,) -> Array < PackedVector2Array > {
            type RetMarshal = PtrcallReturnT < Array < PackedVector2Array > >;
            type CallSig = (Array < PackedVector2Array >, PackedVector2Array, PackedVector2Array);
            let args = (polygon_a, polygon_b,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3536usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "merge_polygons", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn clip_polygons(&mut self, polygon_a: PackedVector2Array, polygon_b: PackedVector2Array,) -> Array < PackedVector2Array > {
            type RetMarshal = PtrcallReturnT < Array < PackedVector2Array > >;
            type CallSig = (Array < PackedVector2Array >, PackedVector2Array, PackedVector2Array);
            let args = (polygon_a, polygon_b,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3537usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "clip_polygons", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn intersect_polygons(&mut self, polygon_a: PackedVector2Array, polygon_b: PackedVector2Array,) -> Array < PackedVector2Array > {
            type RetMarshal = PtrcallReturnT < Array < PackedVector2Array > >;
            type CallSig = (Array < PackedVector2Array >, PackedVector2Array, PackedVector2Array);
            let args = (polygon_a, polygon_b,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3538usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "intersect_polygons", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn exclude_polygons(&mut self, polygon_a: PackedVector2Array, polygon_b: PackedVector2Array,) -> Array < PackedVector2Array > {
            type RetMarshal = PtrcallReturnT < Array < PackedVector2Array > >;
            type CallSig = (Array < PackedVector2Array >, PackedVector2Array, PackedVector2Array);
            let args = (polygon_a, polygon_b,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3539usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "exclude_polygons", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn clip_polyline_with_polygon(&mut self, polyline: PackedVector2Array, polygon: PackedVector2Array,) -> Array < PackedVector2Array > {
            type RetMarshal = PtrcallReturnT < Array < PackedVector2Array > >;
            type CallSig = (Array < PackedVector2Array >, PackedVector2Array, PackedVector2Array);
            let args = (polyline, polygon,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3540usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "clip_polyline_with_polygon", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn intersect_polyline_with_polygon(&mut self, polyline: PackedVector2Array, polygon: PackedVector2Array,) -> Array < PackedVector2Array > {
            type RetMarshal = PtrcallReturnT < Array < PackedVector2Array > >;
            type CallSig = (Array < PackedVector2Array >, PackedVector2Array, PackedVector2Array);
            let args = (polyline, polygon,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3541usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "intersect_polyline_with_polygon", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn offset_polygon_full(&mut self, polygon: PackedVector2Array, delta: f32, join_type: crate::engine::geometry_2d::PolyJoinType,) -> Array < PackedVector2Array > {
            type RetMarshal = PtrcallReturnT < Array < PackedVector2Array > >;
            type CallSig = (Array < PackedVector2Array >, PackedVector2Array, f32, crate::engine::geometry_2d::PolyJoinType);
            let args = (polygon, delta, join_type,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3542usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "offset_polygon", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn offset_polygon(&mut self, polygon: PackedVector2Array, delta: f32,) -> Array < PackedVector2Array > {
            self.offset_polygon_ex(polygon, delta,) . done()
        }
        #[inline]
        pub fn offset_polygon_ex(&mut self, polygon: PackedVector2Array, delta: f32,) -> ExOffsetPolygon < '_ > {
            ExOffsetPolygon::new(self, polygon, delta,)
        }
        pub(crate) fn offset_polyline_full(&mut self, polyline: PackedVector2Array, delta: f32, join_type: crate::engine::geometry_2d::PolyJoinType, end_type: crate::engine::geometry_2d::PolyEndType,) -> Array < PackedVector2Array > {
            type RetMarshal = PtrcallReturnT < Array < PackedVector2Array > >;
            type CallSig = (Array < PackedVector2Array >, PackedVector2Array, f32, crate::engine::geometry_2d::PolyJoinType, crate::engine::geometry_2d::PolyEndType);
            let args = (polyline, delta, join_type, end_type,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3543usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "offset_polyline", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn offset_polyline(&mut self, polyline: PackedVector2Array, delta: f32,) -> Array < PackedVector2Array > {
            self.offset_polyline_ex(polyline, delta,) . done()
        }
        #[inline]
        pub fn offset_polyline_ex(&mut self, polyline: PackedVector2Array, delta: f32,) -> ExOffsetPolyline < '_ > {
            ExOffsetPolyline::new(self, polyline, delta,)
        }
        pub fn make_atlas(&mut self, sizes: PackedVector2Array,) -> Dictionary {
            type RetMarshal = PtrcallReturnT < Dictionary >;
            type CallSig = (Dictionary, PackedVector2Array);
            let args = (sizes,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3544usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "make_atlas", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for Geometry2D {
        type Base = crate::engine::Object;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"Geometry2D\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for Geometry2D {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for Geometry2D {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::Object > for Geometry2D {
        
    }
    impl std::ops::Deref for Geometry2D {
        type Target = crate::engine::Object;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for Geometry2D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_Geometry2D {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::Geometry2D > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`Geometry2D::offset_polygon_ex`][super::Geometry2D::offset_polygon_ex]."]
#[must_use]
pub struct ExOffsetPolygon < 'a > {
    surround_object: &'a mut re_export::Geometry2D, polygon: PackedVector2Array, delta: f32, join_type: crate::engine::geometry_2d::PolyJoinType,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExOffsetPolygon < 'a > {
    fn new(surround_object: &'a mut re_export::Geometry2D, polygon: PackedVector2Array, delta: f32,) -> Self {
        Self {
            surround_object, polygon, delta, join_type: crate::obj::EngineEnum::from_ord(0),
        }
    }
    #[inline]
    pub fn join_type(self, value: crate::engine::geometry_2d::PolyJoinType) -> Self {
        Self {
            join_type: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Array < PackedVector2Array > {
        re_export::Geometry2D::offset_polygon_full(self.surround_object, self.polygon, self.delta, self.join_type,)
    }
}
#[doc = "Default-param extender for [`Geometry2D::offset_polyline_ex`][super::Geometry2D::offset_polyline_ex]."]
#[must_use]
pub struct ExOffsetPolyline < 'a > {
    surround_object: &'a mut re_export::Geometry2D, polyline: PackedVector2Array, delta: f32, join_type: crate::engine::geometry_2d::PolyJoinType, end_type: crate::engine::geometry_2d::PolyEndType,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExOffsetPolyline < 'a > {
    fn new(surround_object: &'a mut re_export::Geometry2D, polyline: PackedVector2Array, delta: f32,) -> Self {
        Self {
            surround_object, polyline, delta, join_type: crate::obj::EngineEnum::from_ord(0), end_type: crate::obj::EngineEnum::from_ord(3),
        }
    }
    #[inline]
    pub fn join_type(self, value: crate::engine::geometry_2d::PolyJoinType) -> Self {
        Self {
            join_type: value, .. self
        }
    }
    #[inline]
    pub fn end_type(self, value: crate::engine::geometry_2d::PolyEndType) -> Self {
        Self {
            end_type: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Array < PackedVector2Array > {
        re_export::Geometry2D::offset_polyline_full(self.surround_object, self.polyline, self.delta, self.join_type, self.end_type,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct PolyBooleanOperation {
    ord: i32
}
impl PolyBooleanOperation {
    pub const OPERATION_UNION: Self = Self {
        ord: 0i32
    };
    pub const OPERATION_DIFFERENCE: Self = Self {
        ord: 1i32
    };
    pub const OPERATION_INTERSECTION: Self = Self {
        ord: 2i32
    };
    pub const OPERATION_XOR: Self = Self {
        ord: 3i32
    };
    
}
impl crate::obj::EngineEnum for PolyBooleanOperation {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 => Some(Self {
                ord
            }), _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
impl crate::builtin::meta::GodotConvert for PolyBooleanOperation {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for PolyBooleanOperation {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for PolyBooleanOperation {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct PolyJoinType {
    ord: i32
}
impl PolyJoinType {
    pub const JOIN_SQUARE: Self = Self {
        ord: 0i32
    };
    pub const JOIN_ROUND: Self = Self {
        ord: 1i32
    };
    pub const JOIN_MITER: Self = Self {
        ord: 2i32
    };
    
}
impl crate::obj::EngineEnum for PolyJoinType {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 => Some(Self {
                ord
            }), _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
impl crate::builtin::meta::GodotConvert for PolyJoinType {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for PolyJoinType {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for PolyJoinType {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct PolyEndType {
    ord: i32
}
impl PolyEndType {
    pub const END_POLYGON: Self = Self {
        ord: 0i32
    };
    pub const END_JOINED: Self = Self {
        ord: 1i32
    };
    pub const END_BUTT: Self = Self {
        ord: 2i32
    };
    pub const END_SQUARE: Self = Self {
        ord: 3i32
    };
    pub const END_ROUND: Self = Self {
        ord: 4i32
    };
    
}
impl crate::obj::EngineEnum for PolyEndType {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 => Some(Self {
                ord
            }), _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
impl crate::builtin::meta::GodotConvert for PolyEndType {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for PolyEndType {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for PolyEndType {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}