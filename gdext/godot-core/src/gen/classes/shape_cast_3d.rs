#![doc = "Sidecar module for class [`ShapeCast3D`][crate::engine::ShapeCast3D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `ShapeCast3D` enums](https://docs.godotengine.org/en/stable/classes/class_shapecast3d.html#enumerations).\n\n"]
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
    #[doc = "Godot class `ShapeCast3D.`\n\nInherits [`Node3D`][crate::engine::Node3D].\n\nRelated symbols:\n\n* [`IShapeCast3D`][crate::engine::IShapeCast3D]: virtual methods\n\n\nSee also [Godot docs for `ShapeCast3D`](https://docs.godotengine.org/en/stable/classes/class_shapecast3d.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct ShapeCast3D {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`ShapeCast3D`][crate::engine::ShapeCast3D].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `ShapeCast3D` methods](https://docs.godotengine.org/en/stable/classes/class_shapecast3d.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IShapeCast3D: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
    impl ShapeCast3D {
        pub fn resource_changed(&mut self, resource: Gd < crate::engine::Resource >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::Resource >);
            let args = (resource,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7476usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "resource_changed", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_enabled(&mut self, enabled: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7477usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_enabled(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7478usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_shape(&mut self, shape: Gd < crate::engine::Shape3D >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::Shape3D >);
            let args = (shape,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7479usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_shape", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_shape(&self,) -> Option < Gd < crate::engine::Shape3D > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::Shape3D > >;
            type CallSig = (Option < Gd < crate::engine::Shape3D > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7480usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_shape", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_target_position(&mut self, local_point: Vector3,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Vector3);
            let args = (local_point,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7481usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_target_position", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_target_position(&self,) -> Vector3 {
            type RetMarshal = PtrcallReturnT < Vector3 >;
            type CallSig = (Vector3,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7482usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_target_position", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_margin(&mut self, margin: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (margin,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7483usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_margin", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_margin(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7484usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_margin", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_max_results(&mut self, max_results: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (max_results,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7485usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_max_results", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_max_results(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7486usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_max_results", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_colliding(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7487usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_colliding", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_collision_count(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7488usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_collision_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn force_shapecast_update(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7489usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "force_shapecast_update", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_collider(&self, index: i32,) -> Option < Gd < crate::engine::Object > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::Object > >;
            type CallSig = (Option < Gd < crate::engine::Object > >, i32);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7490usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_collider", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_collider_rid(&self, index: i32,) -> Rid {
            type RetMarshal = PtrcallReturnT < Rid >;
            type CallSig = (Rid, i32);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7491usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_collider_rid", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_collider_shape(&self, index: i32,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, i32);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7492usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_collider_shape", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_collision_point(&self, index: i32,) -> Vector3 {
            type RetMarshal = PtrcallReturnT < Vector3 >;
            type CallSig = (Vector3, i32);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7493usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_collision_point", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_collision_normal(&self, index: i32,) -> Vector3 {
            type RetMarshal = PtrcallReturnT < Vector3 >;
            type CallSig = (Vector3, i32);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7494usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_collision_normal", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_closest_collision_safe_fraction(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7495usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_closest_collision_safe_fraction", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_closest_collision_unsafe_fraction(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7496usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_closest_collision_unsafe_fraction", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn add_exception_rid(&mut self, rid: Rid,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid);
            let args = (rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7497usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_exception_rid", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn add_exception(&mut self, node: Gd < crate::engine::CollisionObject3D >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::CollisionObject3D >);
            let args = (node,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7498usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_exception", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn remove_exception_rid(&mut self, rid: Rid,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid);
            let args = (rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7499usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "remove_exception_rid", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn remove_exception(&mut self, node: Gd < crate::engine::CollisionObject3D >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::CollisionObject3D >);
            let args = (node,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7500usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "remove_exception", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn clear_exceptions(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7501usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "clear_exceptions", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_collision_mask(&mut self, mask: u32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), u32);
            let args = (mask,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7502usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_collision_mask", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_collision_mask(&self,) -> u32 {
            type RetMarshal = PtrcallReturnT < u32 >;
            type CallSig = (u32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7503usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_collision_mask", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_collision_mask_value(&mut self, layer_number: i32, value: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, bool);
            let args = (layer_number, value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7504usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_collision_mask_value", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_collision_mask_value(&self, layer_number: i32,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, i32);
            let args = (layer_number,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7505usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_collision_mask_value", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_exclude_parent_body(&mut self, mask: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (mask,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7506usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_exclude_parent_body", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_exclude_parent_body(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7507usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_exclude_parent_body", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_collide_with_areas(&mut self, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7508usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_collide_with_areas", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_collide_with_areas_enabled(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7509usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_collide_with_areas_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_collide_with_bodies(&mut self, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7510usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_collide_with_bodies", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_collide_with_bodies_enabled(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7511usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_collide_with_bodies_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_debug_shape_custom_color(&mut self, debug_shape_custom_color: Color,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Color);
            let args = (debug_shape_custom_color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7512usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_debug_shape_custom_color", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_debug_shape_custom_color(&self,) -> Color {
            type RetMarshal = PtrcallReturnT < Color >;
            type CallSig = (Color,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7513usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_debug_shape_custom_color", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for ShapeCast3D {
        type Base = crate::engine::Node3D;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"ShapeCast3D\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for ShapeCast3D {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for ShapeCast3D {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::Node3D > for ShapeCast3D {
        
    }
    impl crate::obj::Inherits < crate::engine::Node > for ShapeCast3D {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for ShapeCast3D {
        
    }
    impl crate::obj::ExportableObject for ShapeCast3D {
        
    }
    impl crate::obj::cap::GodotDefault for ShapeCast3D {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for ShapeCast3D {
        type Target = crate::engine::Node3D;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for ShapeCast3D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_ShapeCast3D {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::ShapeCast3D > for $Class {
                
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