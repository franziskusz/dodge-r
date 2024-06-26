#![doc = "Sidecar module for class [`PhysicsDirectBodyState2DExtension`][crate::engine::PhysicsDirectBodyState2DExtension].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `PhysicsDirectBodyState2DExtension` enums](https://docs.godotengine.org/en/stable/classes/class_physicsdirectbodystate2dextension.html#enumerations).\n\n"]
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
    #[doc = "Godot class `PhysicsDirectBodyState2DExtension.`\n\nInherits [`PhysicsDirectBodyState2D`][crate::engine::PhysicsDirectBodyState2D].\n\nRelated symbols:\n\n* [`IPhysicsDirectBodyState2DExtension`][crate::engine::IPhysicsDirectBodyState2DExtension]: virtual methods\n\n\nSee also [Godot docs for `PhysicsDirectBodyState2DExtension`](https://docs.godotengine.org/en/stable/classes/class_physicsdirectbodystate2dextension.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct PhysicsDirectBodyState2DExtension {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`PhysicsDirectBodyState2DExtension`][crate::engine::PhysicsDirectBodyState2DExtension].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `PhysicsDirectBodyState2DExtension` methods](https://docs.godotengine.org/en/stable/classes/class_physicsdirectbodystate2dextension.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IPhysicsDirectBodyState2DExtension: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
        fn get_total_gravity(&self,) -> Vector2 {
            unimplemented !()
        }
        fn get_total_linear_damp(&self,) -> f32 {
            unimplemented !()
        }
        fn get_total_angular_damp(&self,) -> f32 {
            unimplemented !()
        }
        fn get_center_of_mass(&self,) -> Vector2 {
            unimplemented !()
        }
        fn get_center_of_mass_local(&self,) -> Vector2 {
            unimplemented !()
        }
        fn get_inverse_mass(&self,) -> f32 {
            unimplemented !()
        }
        fn get_inverse_inertia(&self,) -> f32 {
            unimplemented !()
        }
        fn set_linear_velocity(&mut self, velocity: Vector2,) {
            unimplemented !()
        }
        fn get_linear_velocity(&self,) -> Vector2 {
            unimplemented !()
        }
        fn set_angular_velocity(&mut self, velocity: f32,) {
            unimplemented !()
        }
        fn get_angular_velocity(&self,) -> f32 {
            unimplemented !()
        }
        fn set_transform(&mut self, transform: Transform2D,) {
            unimplemented !()
        }
        fn get_transform(&self,) -> Transform2D {
            unimplemented !()
        }
        fn get_velocity_at_local_position(&self, local_position: Vector2,) -> Vector2 {
            unimplemented !()
        }
        fn apply_central_impulse(&mut self, impulse: Vector2,) {
            unimplemented !()
        }
        fn apply_impulse(&mut self, impulse: Vector2, position: Vector2,) {
            unimplemented !()
        }
        fn apply_torque_impulse(&mut self, impulse: f32,) {
            unimplemented !()
        }
        fn apply_central_force(&mut self, force: Vector2,) {
            unimplemented !()
        }
        fn apply_force(&mut self, force: Vector2, position: Vector2,) {
            unimplemented !()
        }
        fn apply_torque(&mut self, torque: f32,) {
            unimplemented !()
        }
        fn add_constant_central_force(&mut self, force: Vector2,) {
            unimplemented !()
        }
        fn add_constant_force(&mut self, force: Vector2, position: Vector2,) {
            unimplemented !()
        }
        fn add_constant_torque(&mut self, torque: f32,) {
            unimplemented !()
        }
        fn set_constant_force(&mut self, force: Vector2,) {
            unimplemented !()
        }
        fn get_constant_force(&self,) -> Vector2 {
            unimplemented !()
        }
        fn set_constant_torque(&mut self, torque: f32,) {
            unimplemented !()
        }
        fn get_constant_torque(&self,) -> f32 {
            unimplemented !()
        }
        fn set_sleep_state(&mut self, enabled: bool,) {
            unimplemented !()
        }
        fn is_sleeping(&self,) -> bool {
            unimplemented !()
        }
        fn get_contact_count(&self,) -> i32 {
            unimplemented !()
        }
        fn get_contact_local_position(&self, contact_idx: i32,) -> Vector2 {
            unimplemented !()
        }
        fn get_contact_local_normal(&self, contact_idx: i32,) -> Vector2 {
            unimplemented !()
        }
        fn get_contact_local_shape(&self, contact_idx: i32,) -> i32 {
            unimplemented !()
        }
        fn get_contact_local_velocity_at_position(&self, contact_idx: i32,) -> Vector2 {
            unimplemented !()
        }
        fn get_contact_collider(&self, contact_idx: i32,) -> Rid {
            unimplemented !()
        }
        fn get_contact_collider_position(&self, contact_idx: i32,) -> Vector2 {
            unimplemented !()
        }
        fn get_contact_collider_id(&self, contact_idx: i32,) -> u64 {
            unimplemented !()
        }
        fn get_contact_collider_object(&self, contact_idx: i32,) -> Option < Gd < crate::engine::Object > > {
            unimplemented !()
        }
        fn get_contact_collider_shape(&self, contact_idx: i32,) -> i32 {
            unimplemented !()
        }
        fn get_contact_collider_velocity_at_position(&self, contact_idx: i32,) -> Vector2 {
            unimplemented !()
        }
        fn get_contact_impulse(&self, contact_idx: i32,) -> Vector2 {
            unimplemented !()
        }
        fn get_step(&self,) -> f32 {
            unimplemented !()
        }
        fn integrate_forces(&mut self,) {
            unimplemented !()
        }
        fn get_space_state(&mut self,) -> Option < Gd < crate::engine::PhysicsDirectSpaceState2D > > {
            unimplemented !()
        }
    }
    impl PhysicsDirectBodyState2DExtension {
        fn __checked_id(&self) -> Option < crate::obj::InstanceId > {
            let rtti = unsafe {
                self.rtti.as_ref() . unwrap_unchecked()
            };
            let instance_id = rtti.check_type::< Self > ();
            Some(instance_id)
        }
    }
    impl crate::obj::GodotClass for PhysicsDirectBodyState2DExtension {
        type Base = crate::engine::PhysicsDirectBodyState2D;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"PhysicsDirectBodyState2DExtension\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for PhysicsDirectBodyState2DExtension {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for PhysicsDirectBodyState2DExtension {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::PhysicsDirectBodyState2D > for PhysicsDirectBodyState2DExtension {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for PhysicsDirectBodyState2DExtension {
        
    }
    impl crate::obj::cap::GodotDefault for PhysicsDirectBodyState2DExtension {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for PhysicsDirectBodyState2DExtension {
        type Target = crate::engine::PhysicsDirectBodyState2D;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for PhysicsDirectBodyState2DExtension {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_PhysicsDirectBodyState2DExtension {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::PhysicsDirectBodyState2DExtension > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::PhysicsDirectBodyState2D > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::Object > for $Class {
                
            }
        }
    }
}