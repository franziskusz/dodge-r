#![doc = "Sidecar module for class [`PhysicsBody3D`][crate::engine::PhysicsBody3D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `PhysicsBody3D` enums](https://docs.godotengine.org/en/stable/classes/class_physicsbody3d.html#enumerations).\n\n"]
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
    #[doc = "Godot class `PhysicsBody3D.`\n\nInherits [`CollisionObject3D`][crate::engine::CollisionObject3D].\n\nRelated symbols:\n\n* [`physics_body_3d`][crate::engine::physics_body_3d]: sidecar module with related enum/flag types\n* [`IPhysicsBody3D`][crate::engine::IPhysicsBody3D]: virtual methods\n\n\nSee also [Godot docs for `PhysicsBody3D`](https://docs.godotengine.org/en/stable/classes/class_physicsbody3d.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct PhysicsBody3D {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`PhysicsBody3D`][crate::engine::PhysicsBody3D].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `PhysicsBody3D` methods](https://docs.godotengine.org/en/stable/classes/class_physicsbody3d.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IPhysicsBody3D: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
        fn input_event(&mut self, camera: Gd < crate::engine::Camera3D >, event: Gd < crate::engine::InputEvent >, position: Vector3, normal: Vector3, shape_idx: i32,) {
            unimplemented !()
        }
        fn mouse_enter(&mut self,) {
            unimplemented !()
        }
        fn mouse_exit(&mut self,) {
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
    impl PhysicsBody3D {
        pub(crate) fn move_and_collide_full(&mut self, motion: Vector3, test_only: bool, safe_margin: f32, recovery_as_collision: bool, max_collisions: i32,) -> Option < Gd < crate::engine::KinematicCollision3D > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::KinematicCollision3D > >;
            type CallSig = (Option < Gd < crate::engine::KinematicCollision3D > >, Vector3, bool, f32, bool, i32);
            let args = (motion, test_only, safe_margin, recovery_as_collision, max_collisions,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5674usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "move_and_collide", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn move_and_collide(&mut self, motion: Vector3,) -> Option < Gd < crate::engine::KinematicCollision3D > > {
            self.move_and_collide_ex(motion,) . done()
        }
        #[inline]
        pub fn move_and_collide_ex(&mut self, motion: Vector3,) -> ExMoveAndCollide < '_ > {
            ExMoveAndCollide::new(self, motion,)
        }
        pub(crate) fn test_move_full(&mut self, from: Transform3D, motion: Vector3, collision: Gd < crate::engine::KinematicCollision3D >, safe_margin: f32, recovery_as_collision: bool, max_collisions: i32,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, Transform3D, Vector3, Gd < crate::engine::KinematicCollision3D >, f32, bool, i32);
            let args = (from, motion, collision, safe_margin, recovery_as_collision, max_collisions,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5675usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "test_move", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn test_move(&mut self, from: Transform3D, motion: Vector3,) -> bool {
            self.test_move_ex(from, motion,) . done()
        }
        #[inline]
        pub fn test_move_ex(&mut self, from: Transform3D, motion: Vector3,) -> ExTestMove < '_ > {
            ExTestMove::new(self, from, motion,)
        }
        pub fn set_axis_lock(&mut self, axis: crate::engine::physics_server_3d::BodyAxis, lock: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::physics_server_3d::BodyAxis, bool);
            let args = (axis, lock,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5676usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_axis_lock", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_axis_lock(&self, axis: crate::engine::physics_server_3d::BodyAxis,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, crate::engine::physics_server_3d::BodyAxis);
            let args = (axis,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5677usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_axis_lock", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_collision_exceptions(&mut self,) -> Array < Gd < crate::engine::PhysicsBody3D > > {
            type RetMarshal = PtrcallReturnT < Array < Gd < crate::engine::PhysicsBody3D > > >;
            type CallSig = (Array < Gd < crate::engine::PhysicsBody3D > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5678usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_collision_exceptions", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn add_collision_exception_with(&mut self, body: Gd < crate::engine::Node >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::Node >);
            let args = (body,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5679usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_collision_exception_with", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn remove_collision_exception_with(&mut self, body: Gd < crate::engine::Node >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::Node >);
            let args = (body,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5680usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "remove_collision_exception_with", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for PhysicsBody3D {
        type Base = crate::engine::CollisionObject3D;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"PhysicsBody3D\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for PhysicsBody3D {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for PhysicsBody3D {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::CollisionObject3D > for PhysicsBody3D {
        
    }
    impl crate::obj::Inherits < crate::engine::Node3D > for PhysicsBody3D {
        
    }
    impl crate::obj::Inherits < crate::engine::Node > for PhysicsBody3D {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for PhysicsBody3D {
        
    }
    impl crate::obj::ExportableObject for PhysicsBody3D {
        
    }
    impl std::ops::Deref for PhysicsBody3D {
        type Target = crate::engine::CollisionObject3D;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for PhysicsBody3D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_PhysicsBody3D {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::PhysicsBody3D > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::CollisionObject3D > for $Class {
                
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
#[doc = "Default-param extender for [`PhysicsBody3D::move_and_collide_ex`][super::PhysicsBody3D::move_and_collide_ex]."]
#[must_use]
pub struct ExMoveAndCollide < 'a > {
    surround_object: &'a mut re_export::PhysicsBody3D, motion: Vector3, test_only: bool, safe_margin: f32, recovery_as_collision: bool, max_collisions: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExMoveAndCollide < 'a > {
    fn new(surround_object: &'a mut re_export::PhysicsBody3D, motion: Vector3,) -> Self {
        Self {
            surround_object, motion, test_only: false, safe_margin: 0.001f32, recovery_as_collision: false, max_collisions: 1i32,
        }
    }
    #[inline]
    pub fn test_only(self, value: bool) -> Self {
        Self {
            test_only: value, .. self
        }
    }
    #[inline]
    pub fn safe_margin(self, value: f32) -> Self {
        Self {
            safe_margin: value, .. self
        }
    }
    #[inline]
    pub fn recovery_as_collision(self, value: bool) -> Self {
        Self {
            recovery_as_collision: value, .. self
        }
    }
    #[inline]
    pub fn max_collisions(self, value: i32) -> Self {
        Self {
            max_collisions: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::engine::KinematicCollision3D > > {
        re_export::PhysicsBody3D::move_and_collide_full(self.surround_object, self.motion, self.test_only, self.safe_margin, self.recovery_as_collision, self.max_collisions,)
    }
}
#[doc = "Default-param extender for [`PhysicsBody3D::test_move_ex`][super::PhysicsBody3D::test_move_ex]."]
#[must_use]
pub struct ExTestMove < 'a > {
    surround_object: &'a mut re_export::PhysicsBody3D, from: Transform3D, motion: Vector3, collision: Gd < crate::engine::KinematicCollision3D >, safe_margin: f32, recovery_as_collision: bool, max_collisions: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExTestMove < 'a > {
    fn new(surround_object: &'a mut re_export::PhysicsBody3D, from: Transform3D, motion: Vector3,) -> Self {
        Self {
            surround_object, from, motion, collision: unimplemented !("see #156"), safe_margin: 0.001f32, recovery_as_collision: false, max_collisions: 1i32,
        }
    }
    #[inline]
    pub fn collision(self, value: Gd < crate::engine::KinematicCollision3D >) -> Self {
        Self {
            collision: value, .. self
        }
    }
    #[inline]
    pub fn safe_margin(self, value: f32) -> Self {
        Self {
            safe_margin: value, .. self
        }
    }
    #[inline]
    pub fn recovery_as_collision(self, value: bool) -> Self {
        Self {
            recovery_as_collision: value, .. self
        }
    }
    #[inline]
    pub fn max_collisions(self, value: i32) -> Self {
        Self {
            max_collisions: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> bool {
        re_export::PhysicsBody3D::test_move_full(self.surround_object, self.from, self.motion, self.collision, self.safe_margin, self.recovery_as_collision, self.max_collisions,)
    }
}