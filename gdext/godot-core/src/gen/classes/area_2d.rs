#![doc = "Sidecar module for class [`Area2D`][crate::engine::Area2D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `Area2D` enums](https://docs.godotengine.org/en/stable/classes/class_area2d.html#enumerations).\n\n"]
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
    #[doc = "Godot class `Area2D.`\n\nInherits [`CollisionObject2D`][crate::engine::CollisionObject2D].\n\nRelated symbols:\n\n* [`area_2d`][crate::engine::area_2d]: sidecar module with related enum/flag types\n* [`IArea2D`][crate::engine::IArea2D]: virtual methods\n\n\nSee also [Godot docs for `Area2D`](https://docs.godotengine.org/en/stable/classes/class_area2d.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct Area2D {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`Area2D`][crate::engine::Area2D].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `Area2D` methods](https://docs.godotengine.org/en/stable/classes/class_area2d.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IArea2D: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
        fn on_notification(&mut self, what: CanvasItemNotification) {
            unimplemented !()
        }
        fn input_event(&mut self, viewport: Gd < crate::engine::Viewport >, event: Gd < crate::engine::InputEvent >, shape_idx: i32,) {
            unimplemented !()
        }
        fn mouse_enter(&mut self,) {
            unimplemented !()
        }
        fn mouse_exit(&mut self,) {
            unimplemented !()
        }
        fn mouse_shape_enter(&mut self, shape_idx: i32,) {
            unimplemented !()
        }
        fn mouse_shape_exit(&mut self, shape_idx: i32,) {
            unimplemented !()
        }
        fn draw(&mut self,) {
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
    impl Area2D {
        pub fn set_gravity_space_override_mode(&mut self, space_override_mode: crate::engine::area_2d::SpaceOverride,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::area_2d::SpaceOverride);
            let args = (space_override_mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(472usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_gravity_space_override_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_gravity_space_override_mode(&self,) -> crate::engine::area_2d::SpaceOverride {
            type RetMarshal = PtrcallReturnT < crate::engine::area_2d::SpaceOverride >;
            type CallSig = (crate::engine::area_2d::SpaceOverride,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(473usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_gravity_space_override_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_gravity_is_point(&mut self, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(474usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_gravity_is_point", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_gravity_a_point(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(475usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_gravity_a_point", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_gravity_point_unit_distance(&mut self, distance_scale: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (distance_scale,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(476usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_gravity_point_unit_distance", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_gravity_point_unit_distance(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(477usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_gravity_point_unit_distance", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_gravity_point_center(&mut self, center: Vector2,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Vector2);
            let args = (center,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(478usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_gravity_point_center", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_gravity_point_center(&self,) -> Vector2 {
            type RetMarshal = PtrcallReturnT < Vector2 >;
            type CallSig = (Vector2,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(479usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_gravity_point_center", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_gravity_direction(&mut self, direction: Vector2,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Vector2);
            let args = (direction,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(480usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_gravity_direction", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_gravity_direction(&self,) -> Vector2 {
            type RetMarshal = PtrcallReturnT < Vector2 >;
            type CallSig = (Vector2,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(481usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_gravity_direction", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_gravity(&mut self, gravity: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (gravity,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(482usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_gravity", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_gravity(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(483usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_gravity", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_linear_damp_space_override_mode(&mut self, space_override_mode: crate::engine::area_2d::SpaceOverride,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::area_2d::SpaceOverride);
            let args = (space_override_mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(484usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_linear_damp_space_override_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_linear_damp_space_override_mode(&self,) -> crate::engine::area_2d::SpaceOverride {
            type RetMarshal = PtrcallReturnT < crate::engine::area_2d::SpaceOverride >;
            type CallSig = (crate::engine::area_2d::SpaceOverride,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(485usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_linear_damp_space_override_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_angular_damp_space_override_mode(&mut self, space_override_mode: crate::engine::area_2d::SpaceOverride,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::area_2d::SpaceOverride);
            let args = (space_override_mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(486usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_angular_damp_space_override_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_angular_damp_space_override_mode(&self,) -> crate::engine::area_2d::SpaceOverride {
            type RetMarshal = PtrcallReturnT < crate::engine::area_2d::SpaceOverride >;
            type CallSig = (crate::engine::area_2d::SpaceOverride,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(487usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_angular_damp_space_override_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_linear_damp(&mut self, linear_damp: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (linear_damp,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(488usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_linear_damp", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_linear_damp(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(489usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_linear_damp", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_angular_damp(&mut self, angular_damp: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (angular_damp,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(490usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_angular_damp", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_angular_damp(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(491usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_angular_damp", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_priority(&mut self, priority: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (priority,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(492usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_priority", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_priority(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(493usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_priority", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_monitoring(&mut self, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(494usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_monitoring", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_monitoring(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(495usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_monitoring", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_monitorable(&mut self, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(496usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_monitorable", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_monitorable(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(497usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_monitorable", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_overlapping_bodies(&self,) -> Array < Gd < crate::engine::Node2D > > {
            type RetMarshal = PtrcallReturnT < Array < Gd < crate::engine::Node2D > > >;
            type CallSig = (Array < Gd < crate::engine::Node2D > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(498usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_overlapping_bodies", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_overlapping_areas(&self,) -> Array < Gd < crate::engine::Area2D > > {
            type RetMarshal = PtrcallReturnT < Array < Gd < crate::engine::Area2D > > >;
            type CallSig = (Array < Gd < crate::engine::Area2D > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(499usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_overlapping_areas", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn has_overlapping_bodies(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(500usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "has_overlapping_bodies", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn has_overlapping_areas(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(501usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "has_overlapping_areas", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn overlaps_body(&self, body: Gd < crate::engine::Node >,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, Gd < crate::engine::Node >);
            let args = (body,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(502usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "overlaps_body", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn overlaps_area(&self, area: Gd < crate::engine::Node >,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, Gd < crate::engine::Node >);
            let args = (area,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(503usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "overlaps_area", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_audio_bus_name(&mut self, name: StringName,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), StringName);
            let args = (name,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(504usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_audio_bus_name", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_audio_bus_name(&self,) -> StringName {
            type RetMarshal = PtrcallReturnT < StringName >;
            type CallSig = (StringName,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(505usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_audio_bus_name", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_audio_bus_override(&mut self, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(506usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_audio_bus_override", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_overriding_audio_bus(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(507usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_overriding_audio_bus", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for Area2D {
        type Base = crate::engine::CollisionObject2D;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"Area2D\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for Area2D {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for Area2D {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::CollisionObject2D > for Area2D {
        
    }
    impl crate::obj::Inherits < crate::engine::Node2D > for Area2D {
        
    }
    impl crate::obj::Inherits < crate::engine::CanvasItem > for Area2D {
        
    }
    impl crate::obj::Inherits < crate::engine::Node > for Area2D {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for Area2D {
        
    }
    impl crate::obj::ExportableObject for Area2D {
        
    }
    impl crate::obj::cap::GodotDefault for Area2D {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for Area2D {
        type Target = crate::engine::CollisionObject2D;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for Area2D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_Area2D {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::Area2D > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::CollisionObject2D > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::Node2D > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::CanvasItem > for $Class {
                
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
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct SpaceOverride {
    ord: i32
}
impl SpaceOverride {
    pub const SPACE_OVERRIDE_DISABLED: Self = Self {
        ord: 0i32
    };
    pub const SPACE_OVERRIDE_COMBINE: Self = Self {
        ord: 1i32
    };
    pub const SPACE_OVERRIDE_COMBINE_REPLACE: Self = Self {
        ord: 2i32
    };
    pub const SPACE_OVERRIDE_REPLACE: Self = Self {
        ord: 3i32
    };
    pub const SPACE_OVERRIDE_REPLACE_COMBINE: Self = Self {
        ord: 4i32
    };
    
}
impl crate::obj::EngineEnum for SpaceOverride {
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
impl crate::builtin::meta::GodotConvert for SpaceOverride {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for SpaceOverride {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for SpaceOverride {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}