#![doc = "Sidecar module for class [`AStar3D`][crate::engine::AStar3D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `AStar3D` enums](https://docs.godotengine.org/en/stable/classes/class_astar3d.html#enumerations).\n\n"]
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
    #[doc = "Godot class `AStar3D.`\n\nInherits [`RefCounted`][crate::engine::RefCounted].\n\nRelated symbols:\n\n* [`a_star_3d`][crate::engine::a_star_3d]: sidecar module with related enum/flag types\n* [`IAStar3D`][crate::engine::IAStar3D]: virtual methods\n\n\nSee also [Godot docs for `AStar3D`](https://docs.godotengine.org/en/stable/classes/class_astar3d.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct AStar3D {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`AStar3D`][crate::engine::AStar3D].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `AStar3D` methods](https://docs.godotengine.org/en/stable/classes/class_astar3d.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IAStar3D: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
        fn estimate_cost(&self, from_id: i64, to_id: i64,) -> f32 {
            unimplemented !()
        }
        fn compute_cost(&self, from_id: i64, to_id: i64,) -> f32 {
            unimplemented !()
        }
    }
    impl AStar3D {
        #[deprecated = "Replaced with `new_gd` in extension trait `NewGd`."]
        pub fn new() -> Gd < Self > {
            crate::obj::Gd::default()
        }
        pub fn get_available_point_id(&self,) -> i64 {
            type RetMarshal = PtrcallReturnT < i64 >;
            type CallSig = (i64,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(27usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_available_point_id", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn add_point_full(&mut self, id: i64, position: Vector3, weight_scale: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i64, Vector3, f32);
            let args = (id, position, weight_scale,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(28usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_point", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn add_point(&mut self, id: i64, position: Vector3,) {
            self.add_point_ex(id, position,) . done()
        }
        #[inline]
        pub fn add_point_ex(&mut self, id: i64, position: Vector3,) -> ExAddPoint < '_ > {
            ExAddPoint::new(self, id, position,)
        }
        pub fn get_point_position(&self, id: i64,) -> Vector3 {
            type RetMarshal = PtrcallReturnT < Vector3 >;
            type CallSig = (Vector3, i64);
            let args = (id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(29usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_point_position", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_point_position(&mut self, id: i64, position: Vector3,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i64, Vector3);
            let args = (id, position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(30usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_point_position", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_point_weight_scale(&self, id: i64,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32, i64);
            let args = (id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(31usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_point_weight_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_point_weight_scale(&mut self, id: i64, weight_scale: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i64, f32);
            let args = (id, weight_scale,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(32usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_point_weight_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn remove_point(&mut self, id: i64,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i64);
            let args = (id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(33usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "remove_point", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn has_point(&self, id: i64,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, i64);
            let args = (id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(34usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "has_point", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_point_connections(&mut self, id: i64,) -> PackedInt64Array {
            type RetMarshal = PtrcallReturnT < PackedInt64Array >;
            type CallSig = (PackedInt64Array, i64);
            let args = (id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(35usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_point_connections", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_point_ids(&mut self,) -> PackedInt64Array {
            type RetMarshal = PtrcallReturnT < PackedInt64Array >;
            type CallSig = (PackedInt64Array,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(36usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_point_ids", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn set_point_disabled_full(&mut self, id: i64, disabled: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i64, bool);
            let args = (id, disabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(37usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_point_disabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn set_point_disabled(&mut self, id: i64,) {
            self.set_point_disabled_ex(id,) . done()
        }
        #[inline]
        pub fn set_point_disabled_ex(&mut self, id: i64,) -> ExSetPointDisabled < '_ > {
            ExSetPointDisabled::new(self, id,)
        }
        pub fn is_point_disabled(&self, id: i64,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, i64);
            let args = (id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(38usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_point_disabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn connect_points_full(&mut self, id: i64, to_id: i64, bidirectional: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i64, i64, bool);
            let args = (id, to_id, bidirectional,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(39usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "connect_points", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn connect_points(&mut self, id: i64, to_id: i64,) {
            self.connect_points_ex(id, to_id,) . done()
        }
        #[inline]
        pub fn connect_points_ex(&mut self, id: i64, to_id: i64,) -> ExConnectPoints < '_ > {
            ExConnectPoints::new(self, id, to_id,)
        }
        pub(crate) fn disconnect_points_full(&mut self, id: i64, to_id: i64, bidirectional: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i64, i64, bool);
            let args = (id, to_id, bidirectional,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(40usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "disconnect_points", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn disconnect_points(&mut self, id: i64, to_id: i64,) {
            self.disconnect_points_ex(id, to_id,) . done()
        }
        #[inline]
        pub fn disconnect_points_ex(&mut self, id: i64, to_id: i64,) -> ExDisconnectPoints < '_ > {
            ExDisconnectPoints::new(self, id, to_id,)
        }
        pub(crate) fn are_points_connected_full(&self, id: i64, to_id: i64, bidirectional: bool,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, i64, i64, bool);
            let args = (id, to_id, bidirectional,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(41usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "are_points_connected", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn are_points_connected(&self, id: i64, to_id: i64,) -> bool {
            self.are_points_connected_ex(id, to_id,) . done()
        }
        #[inline]
        pub fn are_points_connected_ex(&self, id: i64, to_id: i64,) -> ExArePointsConnected < '_ > {
            ExArePointsConnected::new(self, id, to_id,)
        }
        pub fn get_point_count(&self,) -> i64 {
            type RetMarshal = PtrcallReturnT < i64 >;
            type CallSig = (i64,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(42usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_point_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_point_capacity(&self,) -> i64 {
            type RetMarshal = PtrcallReturnT < i64 >;
            type CallSig = (i64,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(43usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_point_capacity", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn reserve_space(&mut self, num_nodes: i64,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i64);
            let args = (num_nodes,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(44usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "reserve_space", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn clear(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(45usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "clear", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn get_closest_point_full(&self, to_position: Vector3, include_disabled: bool,) -> i64 {
            type RetMarshal = PtrcallReturnT < i64 >;
            type CallSig = (i64, Vector3, bool);
            let args = (to_position, include_disabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(46usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_closest_point", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn get_closest_point(&self, to_position: Vector3,) -> i64 {
            self.get_closest_point_ex(to_position,) . done()
        }
        #[inline]
        pub fn get_closest_point_ex(&self, to_position: Vector3,) -> ExGetClosestPoint < '_ > {
            ExGetClosestPoint::new(self, to_position,)
        }
        pub fn get_closest_position_in_segment(&self, to_position: Vector3,) -> Vector3 {
            type RetMarshal = PtrcallReturnT < Vector3 >;
            type CallSig = (Vector3, Vector3);
            let args = (to_position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(47usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_closest_position_in_segment", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_point_path(&mut self, from_id: i64, to_id: i64,) -> PackedVector3Array {
            type RetMarshal = PtrcallReturnT < PackedVector3Array >;
            type CallSig = (PackedVector3Array, i64, i64);
            let args = (from_id, to_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(48usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_point_path", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_id_path(&mut self, from_id: i64, to_id: i64,) -> PackedInt64Array {
            type RetMarshal = PtrcallReturnT < PackedInt64Array >;
            type CallSig = (PackedInt64Array, i64, i64);
            let args = (from_id, to_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(49usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_id_path", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for AStar3D {
        type Base = crate::engine::RefCounted;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"AStar3D\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for AStar3D {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for AStar3D {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::RefCounted > for AStar3D {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for AStar3D {
        
    }
    impl crate::obj::cap::GodotDefault for AStar3D {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for AStar3D {
        type Target = crate::engine::RefCounted;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for AStar3D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_AStar3D {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::AStar3D > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::RefCounted > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`AStar3D::add_point_ex`][super::AStar3D::add_point_ex]."]
#[must_use]
pub struct ExAddPoint < 'a > {
    surround_object: &'a mut re_export::AStar3D, id: i64, position: Vector3, weight_scale: f32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddPoint < 'a > {
    fn new(surround_object: &'a mut re_export::AStar3D, id: i64, position: Vector3,) -> Self {
        Self {
            surround_object, id, position, weight_scale: 1f32,
        }
    }
    #[inline]
    pub fn weight_scale(self, value: f32) -> Self {
        Self {
            weight_scale: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::AStar3D::add_point_full(self.surround_object, self.id, self.position, self.weight_scale,)
    }
}
#[doc = "Default-param extender for [`AStar3D::set_point_disabled_ex`][super::AStar3D::set_point_disabled_ex]."]
#[must_use]
pub struct ExSetPointDisabled < 'a > {
    surround_object: &'a mut re_export::AStar3D, id: i64, disabled: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExSetPointDisabled < 'a > {
    fn new(surround_object: &'a mut re_export::AStar3D, id: i64,) -> Self {
        Self {
            surround_object, id, disabled: true,
        }
    }
    #[inline]
    pub fn disabled(self, value: bool) -> Self {
        Self {
            disabled: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::AStar3D::set_point_disabled_full(self.surround_object, self.id, self.disabled,)
    }
}
#[doc = "Default-param extender for [`AStar3D::connect_points_ex`][super::AStar3D::connect_points_ex]."]
#[must_use]
pub struct ExConnectPoints < 'a > {
    surround_object: &'a mut re_export::AStar3D, id: i64, to_id: i64, bidirectional: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExConnectPoints < 'a > {
    fn new(surround_object: &'a mut re_export::AStar3D, id: i64, to_id: i64,) -> Self {
        Self {
            surround_object, id, to_id, bidirectional: true,
        }
    }
    #[inline]
    pub fn bidirectional(self, value: bool) -> Self {
        Self {
            bidirectional: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::AStar3D::connect_points_full(self.surround_object, self.id, self.to_id, self.bidirectional,)
    }
}
#[doc = "Default-param extender for [`AStar3D::disconnect_points_ex`][super::AStar3D::disconnect_points_ex]."]
#[must_use]
pub struct ExDisconnectPoints < 'a > {
    surround_object: &'a mut re_export::AStar3D, id: i64, to_id: i64, bidirectional: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExDisconnectPoints < 'a > {
    fn new(surround_object: &'a mut re_export::AStar3D, id: i64, to_id: i64,) -> Self {
        Self {
            surround_object, id, to_id, bidirectional: true,
        }
    }
    #[inline]
    pub fn bidirectional(self, value: bool) -> Self {
        Self {
            bidirectional: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::AStar3D::disconnect_points_full(self.surround_object, self.id, self.to_id, self.bidirectional,)
    }
}
#[doc = "Default-param extender for [`AStar3D::are_points_connected_ex`][super::AStar3D::are_points_connected_ex]."]
#[must_use]
pub struct ExArePointsConnected < 'a > {
    surround_object: &'a re_export::AStar3D, id: i64, to_id: i64, bidirectional: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExArePointsConnected < 'a > {
    fn new(surround_object: &'a re_export::AStar3D, id: i64, to_id: i64,) -> Self {
        Self {
            surround_object, id, to_id, bidirectional: true,
        }
    }
    #[inline]
    pub fn bidirectional(self, value: bool) -> Self {
        Self {
            bidirectional: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> bool {
        re_export::AStar3D::are_points_connected_full(self.surround_object, self.id, self.to_id, self.bidirectional,)
    }
}
#[doc = "Default-param extender for [`AStar3D::get_closest_point_ex`][super::AStar3D::get_closest_point_ex]."]
#[must_use]
pub struct ExGetClosestPoint < 'a > {
    surround_object: &'a re_export::AStar3D, to_position: Vector3, include_disabled: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetClosestPoint < 'a > {
    fn new(surround_object: &'a re_export::AStar3D, to_position: Vector3,) -> Self {
        Self {
            surround_object, to_position, include_disabled: false,
        }
    }
    #[inline]
    pub fn include_disabled(self, value: bool) -> Self {
        Self {
            include_disabled: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i64 {
        re_export::AStar3D::get_closest_point_full(self.surround_object, self.to_position, self.include_disabled,)
    }
}