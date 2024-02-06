#![doc = "Sidecar module for class [`PhysicsDirectBodyState2D`][crate::engine::PhysicsDirectBodyState2D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `PhysicsDirectBodyState2D` enums](https://docs.godotengine.org/en/stable/classes/class_physicsdirectbodystate2d.html#enumerations).\n\n"]
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
    #[doc = "Godot class `PhysicsDirectBodyState2D.`\n\nInherits [`Object`][crate::engine::Object].\n\nRelated symbols:\n\n* [`physics_direct_body_state_2d`][crate::engine::physics_direct_body_state_2d]: sidecar module with related enum/flag types\n* [`IPhysicsDirectBodyState2D`][crate::engine::IPhysicsDirectBodyState2D]: virtual methods\n\n\nSee also [Godot docs for `PhysicsDirectBodyState2D`](https://docs.godotengine.org/en/stable/classes/class_physicsdirectbodystate2d.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct PhysicsDirectBodyState2D {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`PhysicsDirectBodyState2D`][crate::engine::PhysicsDirectBodyState2D].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `PhysicsDirectBodyState2D` methods](https://docs.godotengine.org/en/stable/classes/class_physicsdirectbodystate2d.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IPhysicsDirectBodyState2D: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
    impl PhysicsDirectBodyState2D {
        pub fn get_total_gravity(&self,) -> Vector2 {
            type RetMarshal = PtrcallReturnT < Vector2 >;
            type CallSig = (Vector2,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5681usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_total_gravity", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_total_linear_damp(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5682usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_total_linear_damp", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_total_angular_damp(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5683usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_total_angular_damp", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_center_of_mass(&self,) -> Vector2 {
            type RetMarshal = PtrcallReturnT < Vector2 >;
            type CallSig = (Vector2,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5684usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_center_of_mass", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_center_of_mass_local(&self,) -> Vector2 {
            type RetMarshal = PtrcallReturnT < Vector2 >;
            type CallSig = (Vector2,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5685usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_center_of_mass_local", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_inverse_mass(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5686usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_inverse_mass", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_inverse_inertia(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5687usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_inverse_inertia", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_linear_velocity(&mut self, velocity: Vector2,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Vector2);
            let args = (velocity,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5688usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_linear_velocity", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_linear_velocity(&self,) -> Vector2 {
            type RetMarshal = PtrcallReturnT < Vector2 >;
            type CallSig = (Vector2,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5689usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_linear_velocity", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_angular_velocity(&mut self, velocity: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (velocity,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5690usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_angular_velocity", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_angular_velocity(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5691usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_angular_velocity", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_transform(&mut self, transform: Transform2D,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Transform2D);
            let args = (transform,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5692usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_transform", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_transform(&self,) -> Transform2D {
            type RetMarshal = PtrcallReturnT < Transform2D >;
            type CallSig = (Transform2D,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5693usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_transform", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_velocity_at_local_position(&self, local_position: Vector2,) -> Vector2 {
            type RetMarshal = PtrcallReturnT < Vector2 >;
            type CallSig = (Vector2, Vector2);
            let args = (local_position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5694usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_velocity_at_local_position", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn apply_central_impulse(&mut self, impulse: Vector2,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Vector2);
            let args = (impulse,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5695usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "apply_central_impulse", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn apply_torque_impulse(&mut self, impulse: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (impulse,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5696usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "apply_torque_impulse", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn apply_impulse_full(&mut self, impulse: Vector2, position: Vector2,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Vector2, Vector2);
            let args = (impulse, position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5697usize);
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
        pub(crate) fn apply_central_force_full(&mut self, force: Vector2,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Vector2);
            let args = (force,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5698usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "apply_central_force", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn apply_central_force(&mut self,) {
            self.apply_central_force_ex() . done()
        }
        #[inline]
        pub fn apply_central_force_ex(&mut self,) -> ExApplyCentralForce < '_ > {
            ExApplyCentralForce::new(self,)
        }
        pub(crate) fn apply_force_full(&mut self, force: Vector2, position: Vector2,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Vector2, Vector2);
            let args = (force, position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5699usize);
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
                let method_bind = sys::class_scene_api() . fptr_by_index(5700usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "apply_torque", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn add_constant_central_force_full(&mut self, force: Vector2,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Vector2);
            let args = (force,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5701usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_constant_central_force", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn add_constant_central_force(&mut self,) {
            self.add_constant_central_force_ex() . done()
        }
        #[inline]
        pub fn add_constant_central_force_ex(&mut self,) -> ExAddConstantCentralForce < '_ > {
            ExAddConstantCentralForce::new(self,)
        }
        pub(crate) fn add_constant_force_full(&mut self, force: Vector2, position: Vector2,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Vector2, Vector2);
            let args = (force, position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5702usize);
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
                let method_bind = sys::class_scene_api() . fptr_by_index(5703usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_constant_torque", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_constant_force(&mut self, force: Vector2,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Vector2);
            let args = (force,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5704usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_constant_force", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_constant_force(&self,) -> Vector2 {
            type RetMarshal = PtrcallReturnT < Vector2 >;
            type CallSig = (Vector2,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5705usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_constant_force", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_constant_torque(&mut self, torque: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (torque,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5706usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_constant_torque", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_constant_torque(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5707usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_constant_torque", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_sleep_state(&mut self, enabled: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5708usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_sleep_state", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_sleeping(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5709usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_sleeping", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_contact_count(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5710usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_contact_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_contact_local_position(&self, contact_idx: i32,) -> Vector2 {
            type RetMarshal = PtrcallReturnT < Vector2 >;
            type CallSig = (Vector2, i32);
            let args = (contact_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5711usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_contact_local_position", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_contact_local_normal(&self, contact_idx: i32,) -> Vector2 {
            type RetMarshal = PtrcallReturnT < Vector2 >;
            type CallSig = (Vector2, i32);
            let args = (contact_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5712usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_contact_local_normal", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_contact_local_shape(&self, contact_idx: i32,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, i32);
            let args = (contact_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5713usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_contact_local_shape", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_contact_local_velocity_at_position(&self, contact_idx: i32,) -> Vector2 {
            type RetMarshal = PtrcallReturnT < Vector2 >;
            type CallSig = (Vector2, i32);
            let args = (contact_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5714usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_contact_local_velocity_at_position", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_contact_collider(&self, contact_idx: i32,) -> Rid {
            type RetMarshal = PtrcallReturnT < Rid >;
            type CallSig = (Rid, i32);
            let args = (contact_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5715usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_contact_collider", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_contact_collider_position(&self, contact_idx: i32,) -> Vector2 {
            type RetMarshal = PtrcallReturnT < Vector2 >;
            type CallSig = (Vector2, i32);
            let args = (contact_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5716usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_contact_collider_position", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_contact_collider_id(&self, contact_idx: i32,) -> u64 {
            type RetMarshal = PtrcallReturnT < u64 >;
            type CallSig = (u64, i32);
            let args = (contact_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5717usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_contact_collider_id", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_contact_collider_object(&self, contact_idx: i32,) -> Option < Gd < crate::engine::Object > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::Object > >;
            type CallSig = (Option < Gd < crate::engine::Object > >, i32);
            let args = (contact_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5718usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_contact_collider_object", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_contact_collider_shape(&self, contact_idx: i32,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, i32);
            let args = (contact_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5719usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_contact_collider_shape", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_contact_collider_velocity_at_position(&self, contact_idx: i32,) -> Vector2 {
            type RetMarshal = PtrcallReturnT < Vector2 >;
            type CallSig = (Vector2, i32);
            let args = (contact_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5720usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_contact_collider_velocity_at_position", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_contact_impulse(&self, contact_idx: i32,) -> Vector2 {
            type RetMarshal = PtrcallReturnT < Vector2 >;
            type CallSig = (Vector2, i32);
            let args = (contact_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5721usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_contact_impulse", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_step(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5722usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_step", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn integrate_forces(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5723usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "integrate_forces", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_space_state(&mut self,) -> Option < Gd < crate::engine::PhysicsDirectSpaceState2D > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::PhysicsDirectSpaceState2D > >;
            type CallSig = (Option < Gd < crate::engine::PhysicsDirectSpaceState2D > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5724usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_space_state", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for PhysicsDirectBodyState2D {
        type Base = crate::engine::Object;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"PhysicsDirectBodyState2D\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for PhysicsDirectBodyState2D {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for PhysicsDirectBodyState2D {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::Object > for PhysicsDirectBodyState2D {
        
    }
    impl std::ops::Deref for PhysicsDirectBodyState2D {
        type Target = crate::engine::Object;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for PhysicsDirectBodyState2D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_PhysicsDirectBodyState2D {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::PhysicsDirectBodyState2D > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`PhysicsDirectBodyState2D::apply_impulse_ex`][super::PhysicsDirectBodyState2D::apply_impulse_ex]."]
#[must_use]
pub struct ExApplyImpulse < 'a > {
    surround_object: &'a mut re_export::PhysicsDirectBodyState2D, impulse: Vector2, position: Vector2,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExApplyImpulse < 'a > {
    fn new(surround_object: &'a mut re_export::PhysicsDirectBodyState2D, impulse: Vector2,) -> Self {
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
        re_export::PhysicsDirectBodyState2D::apply_impulse_full(self.surround_object, self.impulse, self.position,)
    }
}
#[doc = "Default-param extender for [`PhysicsDirectBodyState2D::apply_central_force_ex`][super::PhysicsDirectBodyState2D::apply_central_force_ex]."]
#[must_use]
pub struct ExApplyCentralForce < 'a > {
    surround_object: &'a mut re_export::PhysicsDirectBodyState2D, force: Vector2,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExApplyCentralForce < 'a > {
    fn new(surround_object: &'a mut re_export::PhysicsDirectBodyState2D,) -> Self {
        Self {
            surround_object, force: Vector2::new(0 as _, 0 as _),
        }
    }
    #[inline]
    pub fn force(self, value: Vector2) -> Self {
        Self {
            force: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::PhysicsDirectBodyState2D::apply_central_force_full(self.surround_object, self.force,)
    }
}
#[doc = "Default-param extender for [`PhysicsDirectBodyState2D::apply_force_ex`][super::PhysicsDirectBodyState2D::apply_force_ex]."]
#[must_use]
pub struct ExApplyForce < 'a > {
    surround_object: &'a mut re_export::PhysicsDirectBodyState2D, force: Vector2, position: Vector2,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExApplyForce < 'a > {
    fn new(surround_object: &'a mut re_export::PhysicsDirectBodyState2D, force: Vector2,) -> Self {
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
        re_export::PhysicsDirectBodyState2D::apply_force_full(self.surround_object, self.force, self.position,)
    }
}
#[doc = "Default-param extender for [`PhysicsDirectBodyState2D::add_constant_central_force_ex`][super::PhysicsDirectBodyState2D::add_constant_central_force_ex]."]
#[must_use]
pub struct ExAddConstantCentralForce < 'a > {
    surround_object: &'a mut re_export::PhysicsDirectBodyState2D, force: Vector2,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddConstantCentralForce < 'a > {
    fn new(surround_object: &'a mut re_export::PhysicsDirectBodyState2D,) -> Self {
        Self {
            surround_object, force: Vector2::new(0 as _, 0 as _),
        }
    }
    #[inline]
    pub fn force(self, value: Vector2) -> Self {
        Self {
            force: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::PhysicsDirectBodyState2D::add_constant_central_force_full(self.surround_object, self.force,)
    }
}
#[doc = "Default-param extender for [`PhysicsDirectBodyState2D::add_constant_force_ex`][super::PhysicsDirectBodyState2D::add_constant_force_ex]."]
#[must_use]
pub struct ExAddConstantForce < 'a > {
    surround_object: &'a mut re_export::PhysicsDirectBodyState2D, force: Vector2, position: Vector2,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddConstantForce < 'a > {
    fn new(surround_object: &'a mut re_export::PhysicsDirectBodyState2D, force: Vector2,) -> Self {
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
        re_export::PhysicsDirectBodyState2D::add_constant_force_full(self.surround_object, self.force, self.position,)
    }
}