#![doc = "Sidecar module for class [`RigidBody2D`][crate::engine::RigidBody2D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `RigidBody2D` enums](https://docs.godotengine.org/en/stable/classes/class_rigidbody2d.html#enumerations).\n\n"]
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
    #[doc = "Godot class `RigidBody2D.`\n\nInherits [`PhysicsBody2D`][crate::engine::PhysicsBody2D].\n\nRelated symbols:\n\n* [`rigid_body_2d`][crate::engine::rigid_body_2d]: sidecar module with related enum/flag types\n* [`IRigidBody2D`][crate::engine::IRigidBody2D]: virtual methods\n\n\nSee also [Godot docs for `RigidBody2D`](https://docs.godotengine.org/en/stable/classes/class_rigidbody2d.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct RigidBody2D {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`RigidBody2D`][crate::engine::RigidBody2D].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `RigidBody2D` methods](https://docs.godotengine.org/en/stable/classes/class_rigidbody2d.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IRigidBody2D: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
        fn integrate_forces(&mut self, state: Gd < crate::engine::PhysicsDirectBodyState2D >,) {
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
    impl RigidBody2D {
        pub fn set_mass(&mut self, mass: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (mass,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7141usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_mass", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_mass(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7142usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_mass", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_inertia(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7143usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_inertia", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_inertia(&mut self, inertia: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (inertia,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7144usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_inertia", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_center_of_mass_mode(&mut self, mode: crate::engine::rigid_body_2d::CenterOfMassMode,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::rigid_body_2d::CenterOfMassMode);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7145usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_center_of_mass_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_center_of_mass_mode(&self,) -> crate::engine::rigid_body_2d::CenterOfMassMode {
            type RetMarshal = PtrcallReturnT < crate::engine::rigid_body_2d::CenterOfMassMode >;
            type CallSig = (crate::engine::rigid_body_2d::CenterOfMassMode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7146usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_center_of_mass_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_center_of_mass(&mut self, center_of_mass: Vector2,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Vector2);
            let args = (center_of_mass,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7147usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_center_of_mass", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_center_of_mass(&self,) -> Vector2 {
            type RetMarshal = PtrcallReturnT < Vector2 >;
            type CallSig = (Vector2,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7148usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_center_of_mass", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_physics_material_override(&mut self, physics_material_override: Gd < crate::engine::PhysicsMaterial >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::PhysicsMaterial >);
            let args = (physics_material_override,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7149usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_physics_material_override", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_physics_material_override(&self,) -> Option < Gd < crate::engine::PhysicsMaterial > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::PhysicsMaterial > >;
            type CallSig = (Option < Gd < crate::engine::PhysicsMaterial > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7150usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_physics_material_override", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_gravity_scale(&mut self, gravity_scale: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (gravity_scale,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7151usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_gravity_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_gravity_scale(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7152usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_gravity_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_linear_damp_mode(&mut self, linear_damp_mode: crate::engine::rigid_body_2d::DampMode,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::rigid_body_2d::DampMode);
            let args = (linear_damp_mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7153usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_linear_damp_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_linear_damp_mode(&self,) -> crate::engine::rigid_body_2d::DampMode {
            type RetMarshal = PtrcallReturnT < crate::engine::rigid_body_2d::DampMode >;
            type CallSig = (crate::engine::rigid_body_2d::DampMode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7154usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_linear_damp_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_angular_damp_mode(&mut self, angular_damp_mode: crate::engine::rigid_body_2d::DampMode,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::rigid_body_2d::DampMode);
            let args = (angular_damp_mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7155usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_angular_damp_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_angular_damp_mode(&self,) -> crate::engine::rigid_body_2d::DampMode {
            type RetMarshal = PtrcallReturnT < crate::engine::rigid_body_2d::DampMode >;
            type CallSig = (crate::engine::rigid_body_2d::DampMode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7156usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_angular_damp_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_linear_damp(&mut self, linear_damp: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (linear_damp,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7157usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_linear_damp", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_linear_damp(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7158usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_linear_damp", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_angular_damp(&mut self, angular_damp: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (angular_damp,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7159usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_angular_damp", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_angular_damp(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7160usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_angular_damp", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_linear_velocity(&mut self, linear_velocity: Vector2,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Vector2);
            let args = (linear_velocity,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7161usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_linear_velocity", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_linear_velocity(&self,) -> Vector2 {
            type RetMarshal = PtrcallReturnT < Vector2 >;
            type CallSig = (Vector2,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7162usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_linear_velocity", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_angular_velocity(&mut self, angular_velocity: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (angular_velocity,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7163usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_angular_velocity", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_angular_velocity(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7164usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_angular_velocity", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_max_contacts_reported(&mut self, amount: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (amount,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7165usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_max_contacts_reported", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_max_contacts_reported(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7166usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_max_contacts_reported", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_contact_count(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7167usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_contact_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_use_custom_integrator(&mut self, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7168usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_use_custom_integrator", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_using_custom_integrator(&mut self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7169usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_using_custom_integrator", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_contact_monitor(&mut self, enabled: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7170usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_contact_monitor", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_contact_monitor_enabled(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7171usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_contact_monitor_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_continuous_collision_detection_mode(&mut self, mode: crate::engine::rigid_body_2d::CCDMode,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::rigid_body_2d::CCDMode);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7172usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_continuous_collision_detection_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_continuous_collision_detection_mode(&self,) -> crate::engine::rigid_body_2d::CCDMode {
            type RetMarshal = PtrcallReturnT < crate::engine::rigid_body_2d::CCDMode >;
            type CallSig = (crate::engine::rigid_body_2d::CCDMode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7173usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_continuous_collision_detection_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_axis_velocity(&mut self, axis_velocity: Vector2,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Vector2);
            let args = (axis_velocity,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7174usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_axis_velocity", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn apply_central_impulse_full(&mut self, impulse: Vector2,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Vector2);
            let args = (impulse,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7175usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "apply_central_impulse", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn apply_central_impulse(&mut self,) {
            self.apply_central_impulse_ex() . done()
        }
        #[inline]
        pub fn apply_central_impulse_ex(&mut self,) -> ExApplyCentralImpulse < '_ > {
            ExApplyCentralImpulse::new(self,)
        }
        pub(crate) fn apply_impulse_full(&mut self, impulse: Vector2, position: Vector2,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Vector2, Vector2);
            let args = (impulse, position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7176usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "apply_impulse", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn apply_impulse(&mut self, impulse: Vector2,) {
            self.apply_impulse_ex(impulse,) . done()
        }
        #[inline]
        pub fn apply_impulse_ex(&mut self, impulse: Vector2,) -> ExApplyImpulse < '_ > {
            ExApplyImpulse::new(self, impulse,)
        }
        pub fn apply_torque_impulse(&mut self, torque: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (torque,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7177usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "apply_torque_impulse", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn apply_central_force(&mut self, force: Vector2,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Vector2);
            let args = (force,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7178usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "apply_central_force", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn apply_force_full(&mut self, force: Vector2, position: Vector2,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Vector2, Vector2);
            let args = (force, position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7179usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "apply_force", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn apply_force(&mut self, force: Vector2,) {
            self.apply_force_ex(force,) . done()
        }
        #[inline]
        pub fn apply_force_ex(&mut self, force: Vector2,) -> ExApplyForce < '_ > {
            ExApplyForce::new(self, force,)
        }
        pub fn apply_torque(&mut self, torque: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (torque,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7180usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "apply_torque", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn add_constant_central_force(&mut self, force: Vector2,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Vector2);
            let args = (force,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7181usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_constant_central_force", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn add_constant_force_full(&mut self, force: Vector2, position: Vector2,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Vector2, Vector2);
            let args = (force, position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7182usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_constant_force", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn add_constant_force(&mut self, force: Vector2,) {
            self.add_constant_force_ex(force,) . done()
        }
        #[inline]
        pub fn add_constant_force_ex(&mut self, force: Vector2,) -> ExAddConstantForce < '_ > {
            ExAddConstantForce::new(self, force,)
        }
        pub fn add_constant_torque(&mut self, torque: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (torque,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7183usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_constant_torque", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_constant_force(&mut self, force: Vector2,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Vector2);
            let args = (force,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7184usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_constant_force", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_constant_force(&self,) -> Vector2 {
            type RetMarshal = PtrcallReturnT < Vector2 >;
            type CallSig = (Vector2,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7185usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_constant_force", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_constant_torque(&mut self, torque: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (torque,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7186usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_constant_torque", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_constant_torque(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7187usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_constant_torque", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_sleeping(&mut self, sleeping: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (sleeping,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7188usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_sleeping", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_sleeping(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7189usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_sleeping", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_can_sleep(&mut self, able_to_sleep: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (able_to_sleep,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7190usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_can_sleep", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_able_to_sleep(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7191usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_able_to_sleep", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_lock_rotation_enabled(&mut self, lock_rotation: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (lock_rotation,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7192usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_lock_rotation_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_lock_rotation_enabled(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7193usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_lock_rotation_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_freeze_enabled(&mut self, freeze_mode: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (freeze_mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7194usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_freeze_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_freeze_enabled(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7195usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_freeze_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_freeze_mode(&mut self, freeze_mode: crate::engine::rigid_body_2d::FreezeMode,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::rigid_body_2d::FreezeMode);
            let args = (freeze_mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7196usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_freeze_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_freeze_mode(&self,) -> crate::engine::rigid_body_2d::FreezeMode {
            type RetMarshal = PtrcallReturnT < crate::engine::rigid_body_2d::FreezeMode >;
            type CallSig = (crate::engine::rigid_body_2d::FreezeMode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7197usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_freeze_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_colliding_bodies(&self,) -> Array < Gd < crate::engine::Node2D > > {
            type RetMarshal = PtrcallReturnT < Array < Gd < crate::engine::Node2D > > >;
            type CallSig = (Array < Gd < crate::engine::Node2D > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7198usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_colliding_bodies", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for RigidBody2D {
        type Base = crate::engine::PhysicsBody2D;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"RigidBody2D\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for RigidBody2D {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for RigidBody2D {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::PhysicsBody2D > for RigidBody2D {
        
    }
    impl crate::obj::Inherits < crate::engine::CollisionObject2D > for RigidBody2D {
        
    }
    impl crate::obj::Inherits < crate::engine::Node2D > for RigidBody2D {
        
    }
    impl crate::obj::Inherits < crate::engine::CanvasItem > for RigidBody2D {
        
    }
    impl crate::obj::Inherits < crate::engine::Node > for RigidBody2D {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for RigidBody2D {
        
    }
    impl crate::obj::ExportableObject for RigidBody2D {
        
    }
    impl crate::obj::cap::GodotDefault for RigidBody2D {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for RigidBody2D {
        type Target = crate::engine::PhysicsBody2D;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for RigidBody2D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_RigidBody2D {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::RigidBody2D > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::PhysicsBody2D > for $Class {
                
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
#[doc = "Default-param extender for [`RigidBody2D::apply_central_impulse_ex`][super::RigidBody2D::apply_central_impulse_ex]."]
#[must_use]
pub struct ExApplyCentralImpulse < 'a > {
    surround_object: &'a mut re_export::RigidBody2D, impulse: Vector2,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExApplyCentralImpulse < 'a > {
    fn new(surround_object: &'a mut re_export::RigidBody2D,) -> Self {
        Self {
            surround_object, impulse: Vector2::new(0 as _, 0 as _),
        }
    }
    #[inline]
    pub fn impulse(self, value: Vector2) -> Self {
        Self {
            impulse: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::RigidBody2D::apply_central_impulse_full(self.surround_object, self.impulse,)
    }
}
#[doc = "Default-param extender for [`RigidBody2D::apply_impulse_ex`][super::RigidBody2D::apply_impulse_ex]."]
#[must_use]
pub struct ExApplyImpulse < 'a > {
    surround_object: &'a mut re_export::RigidBody2D, impulse: Vector2, position: Vector2,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExApplyImpulse < 'a > {
    fn new(surround_object: &'a mut re_export::RigidBody2D, impulse: Vector2,) -> Self {
        Self {
            surround_object, impulse, position: Vector2::new(0 as _, 0 as _),
        }
    }
    #[inline]
    pub fn position(self, value: Vector2) -> Self {
        Self {
            position: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::RigidBody2D::apply_impulse_full(self.surround_object, self.impulse, self.position,)
    }
}
#[doc = "Default-param extender for [`RigidBody2D::apply_force_ex`][super::RigidBody2D::apply_force_ex]."]
#[must_use]
pub struct ExApplyForce < 'a > {
    surround_object: &'a mut re_export::RigidBody2D, force: Vector2, position: Vector2,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExApplyForce < 'a > {
    fn new(surround_object: &'a mut re_export::RigidBody2D, force: Vector2,) -> Self {
        Self {
            surround_object, force, position: Vector2::new(0 as _, 0 as _),
        }
    }
    #[inline]
    pub fn position(self, value: Vector2) -> Self {
        Self {
            position: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::RigidBody2D::apply_force_full(self.surround_object, self.force, self.position,)
    }
}
#[doc = "Default-param extender for [`RigidBody2D::add_constant_force_ex`][super::RigidBody2D::add_constant_force_ex]."]
#[must_use]
pub struct ExAddConstantForce < 'a > {
    surround_object: &'a mut re_export::RigidBody2D, force: Vector2, position: Vector2,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddConstantForce < 'a > {
    fn new(surround_object: &'a mut re_export::RigidBody2D, force: Vector2,) -> Self {
        Self {
            surround_object, force, position: Vector2::new(0 as _, 0 as _),
        }
    }
    #[inline]
    pub fn position(self, value: Vector2) -> Self {
        Self {
            position: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::RigidBody2D::add_constant_force_full(self.surround_object, self.force, self.position,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct FreezeMode {
    ord: i32
}
impl FreezeMode {
    pub const FREEZE_MODE_STATIC: Self = Self {
        ord: 0i32
    };
    pub const FREEZE_MODE_KINEMATIC: Self = Self {
        ord: 1i32
    };
    
}
impl crate::obj::EngineEnum for FreezeMode {
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
impl crate::builtin::meta::GodotConvert for FreezeMode {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for FreezeMode {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for FreezeMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct CenterOfMassMode {
    ord: i32
}
impl CenterOfMassMode {
    pub const CENTER_OF_MASS_MODE_AUTO: Self = Self {
        ord: 0i32
    };
    pub const CENTER_OF_MASS_MODE_CUSTOM: Self = Self {
        ord: 1i32
    };
    
}
impl crate::obj::EngineEnum for CenterOfMassMode {
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
impl crate::builtin::meta::GodotConvert for CenterOfMassMode {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for CenterOfMassMode {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for CenterOfMassMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct DampMode {
    ord: i32
}
impl DampMode {
    pub const DAMP_MODE_COMBINE: Self = Self {
        ord: 0i32
    };
    pub const DAMP_MODE_REPLACE: Self = Self {
        ord: 1i32
    };
    
}
impl crate::obj::EngineEnum for DampMode {
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
impl crate::builtin::meta::GodotConvert for DampMode {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for DampMode {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for DampMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct CCDMode {
    ord: i32
}
impl CCDMode {
    pub const CCD_MODE_DISABLED: Self = Self {
        ord: 0i32
    };
    pub const CCD_MODE_CAST_RAY: Self = Self {
        ord: 1i32
    };
    pub const CCD_MODE_CAST_SHAPE: Self = Self {
        ord: 2i32
    };
    
}
impl crate::obj::EngineEnum for CCDMode {
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
impl crate::builtin::meta::GodotConvert for CCDMode {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for CCDMode {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for CCDMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}