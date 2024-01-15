#![doc = "Sidecar module for class [`PhysicsTestMotionResult3D`][crate::engine::PhysicsTestMotionResult3D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `PhysicsTestMotionResult3D` enums](https://docs.godotengine.org/en/stable/classes/class_physicstestmotionresult3d.html#enumerations).\n\n"]
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
    #[doc = "Godot class `PhysicsTestMotionResult3D.`\n\nInherits [`RefCounted`][crate::engine::RefCounted].\n\nRelated symbols:\n\n* [`physics_test_motion_result_3d`][crate::engine::physics_test_motion_result_3d]: sidecar module with related enum/flag types\n* [`IPhysicsTestMotionResult3D`][crate::engine::IPhysicsTestMotionResult3D]: virtual methods\n\n\nSee also [Godot docs for `PhysicsTestMotionResult3D`](https://docs.godotengine.org/en/stable/classes/class_physicstestmotionresult3d.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct PhysicsTestMotionResult3D {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`PhysicsTestMotionResult3D`][crate::engine::PhysicsTestMotionResult3D].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `PhysicsTestMotionResult3D` methods](https://docs.godotengine.org/en/stable/classes/class_physicstestmotionresult3d.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IPhysicsTestMotionResult3D: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
    impl PhysicsTestMotionResult3D {
        #[deprecated = "Replaced with `new_gd` in extension trait `NewGd`."]
        pub fn new() -> Gd < Self > {
            crate::obj::Gd::default()
        }
        pub fn get_travel(&self,) -> Vector3 {
            type RetMarshal = PtrcallReturnT < Vector3 >;
            type CallSig = (Vector3,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6189usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_travel", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_remainder(&self,) -> Vector3 {
            type RetMarshal = PtrcallReturnT < Vector3 >;
            type CallSig = (Vector3,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6190usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_remainder", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_collision_safe_fraction(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6191usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_collision_safe_fraction", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_collision_unsafe_fraction(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6192usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_collision_unsafe_fraction", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_collision_count(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6193usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_collision_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn get_collision_point_full(&self, collision_index: i32,) -> Vector3 {
            type RetMarshal = PtrcallReturnT < Vector3 >;
            type CallSig = (Vector3, i32);
            let args = (collision_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6194usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_collision_point", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn get_collision_point(&self,) -> Vector3 {
            self.get_collision_point_ex() . done()
        }
        #[inline]
        pub fn get_collision_point_ex(&self,) -> ExGetCollisionPoint < '_ > {
            ExGetCollisionPoint::new(self,)
        }
        pub(crate) fn get_collision_normal_full(&self, collision_index: i32,) -> Vector3 {
            type RetMarshal = PtrcallReturnT < Vector3 >;
            type CallSig = (Vector3, i32);
            let args = (collision_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6195usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_collision_normal", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn get_collision_normal(&self,) -> Vector3 {
            self.get_collision_normal_ex() . done()
        }
        #[inline]
        pub fn get_collision_normal_ex(&self,) -> ExGetCollisionNormal < '_ > {
            ExGetCollisionNormal::new(self,)
        }
        pub(crate) fn get_collider_velocity_full(&self, collision_index: i32,) -> Vector3 {
            type RetMarshal = PtrcallReturnT < Vector3 >;
            type CallSig = (Vector3, i32);
            let args = (collision_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6196usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_collider_velocity", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn get_collider_velocity(&self,) -> Vector3 {
            self.get_collider_velocity_ex() . done()
        }
        #[inline]
        pub fn get_collider_velocity_ex(&self,) -> ExGetColliderVelocity < '_ > {
            ExGetColliderVelocity::new(self,)
        }
        pub(crate) fn get_collider_id_full(&self, collision_index: i32,) -> u64 {
            type RetMarshal = PtrcallReturnT < u64 >;
            type CallSig = (u64, i32);
            let args = (collision_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6197usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_collider_id", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn get_collider_id(&self,) -> u64 {
            self.get_collider_id_ex() . done()
        }
        #[inline]
        pub fn get_collider_id_ex(&self,) -> ExGetColliderId < '_ > {
            ExGetColliderId::new(self,)
        }
        pub(crate) fn get_collider_rid_full(&self, collision_index: i32,) -> Rid {
            type RetMarshal = PtrcallReturnT < Rid >;
            type CallSig = (Rid, i32);
            let args = (collision_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6198usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_collider_rid", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn get_collider_rid(&self,) -> Rid {
            self.get_collider_rid_ex() . done()
        }
        #[inline]
        pub fn get_collider_rid_ex(&self,) -> ExGetColliderRid < '_ > {
            ExGetColliderRid::new(self,)
        }
        pub(crate) fn get_collider_full(&self, collision_index: i32,) -> Option < Gd < crate::engine::Object > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::Object > >;
            type CallSig = (Option < Gd < crate::engine::Object > >, i32);
            let args = (collision_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6199usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_collider", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn get_collider(&self,) -> Option < Gd < crate::engine::Object > > {
            self.get_collider_ex() . done()
        }
        #[inline]
        pub fn get_collider_ex(&self,) -> ExGetCollider < '_ > {
            ExGetCollider::new(self,)
        }
        pub(crate) fn get_collider_shape_full(&self, collision_index: i32,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, i32);
            let args = (collision_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6200usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_collider_shape", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn get_collider_shape(&self,) -> i32 {
            self.get_collider_shape_ex() . done()
        }
        #[inline]
        pub fn get_collider_shape_ex(&self,) -> ExGetColliderShape < '_ > {
            ExGetColliderShape::new(self,)
        }
        pub(crate) fn get_collision_local_shape_full(&self, collision_index: i32,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, i32);
            let args = (collision_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6201usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_collision_local_shape", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn get_collision_local_shape(&self,) -> i32 {
            self.get_collision_local_shape_ex() . done()
        }
        #[inline]
        pub fn get_collision_local_shape_ex(&self,) -> ExGetCollisionLocalShape < '_ > {
            ExGetCollisionLocalShape::new(self,)
        }
        pub(crate) fn get_collision_depth_full(&self, collision_index: i32,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32, i32);
            let args = (collision_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6202usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_collision_depth", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn get_collision_depth(&self,) -> f32 {
            self.get_collision_depth_ex() . done()
        }
        #[inline]
        pub fn get_collision_depth_ex(&self,) -> ExGetCollisionDepth < '_ > {
            ExGetCollisionDepth::new(self,)
        }
        fn __checked_id(&self) -> Option < crate::obj::InstanceId > {
            let rtti = unsafe {
                self.rtti.as_ref() . unwrap_unchecked()
            };
            let instance_id = rtti.check_type::< Self > ();
            Some(instance_id)
        }
    }
    impl crate::obj::GodotClass for PhysicsTestMotionResult3D {
        type Base = crate::engine::RefCounted;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"PhysicsTestMotionResult3D\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for PhysicsTestMotionResult3D {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for PhysicsTestMotionResult3D {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::RefCounted > for PhysicsTestMotionResult3D {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for PhysicsTestMotionResult3D {
        
    }
    impl crate::obj::cap::GodotDefault for PhysicsTestMotionResult3D {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for PhysicsTestMotionResult3D {
        type Target = crate::engine::RefCounted;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for PhysicsTestMotionResult3D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_PhysicsTestMotionResult3D {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::PhysicsTestMotionResult3D > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::RefCounted > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`PhysicsTestMotionResult3D::get_collision_point_ex`][super::PhysicsTestMotionResult3D::get_collision_point_ex]."]
#[must_use]
pub struct ExGetCollisionPoint < 'a > {
    surround_object: &'a re_export::PhysicsTestMotionResult3D, collision_index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetCollisionPoint < 'a > {
    fn new(surround_object: &'a re_export::PhysicsTestMotionResult3D,) -> Self {
        Self {
            surround_object, collision_index: 0i32,
        }
    }
    #[inline]
    pub fn collision_index(self, value: i32) -> Self {
        Self {
            collision_index: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Vector3 {
        re_export::PhysicsTestMotionResult3D::get_collision_point_full(self.surround_object, self.collision_index,)
    }
}
#[doc = "Default-param extender for [`PhysicsTestMotionResult3D::get_collision_normal_ex`][super::PhysicsTestMotionResult3D::get_collision_normal_ex]."]
#[must_use]
pub struct ExGetCollisionNormal < 'a > {
    surround_object: &'a re_export::PhysicsTestMotionResult3D, collision_index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetCollisionNormal < 'a > {
    fn new(surround_object: &'a re_export::PhysicsTestMotionResult3D,) -> Self {
        Self {
            surround_object, collision_index: 0i32,
        }
    }
    #[inline]
    pub fn collision_index(self, value: i32) -> Self {
        Self {
            collision_index: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Vector3 {
        re_export::PhysicsTestMotionResult3D::get_collision_normal_full(self.surround_object, self.collision_index,)
    }
}
#[doc = "Default-param extender for [`PhysicsTestMotionResult3D::get_collider_velocity_ex`][super::PhysicsTestMotionResult3D::get_collider_velocity_ex]."]
#[must_use]
pub struct ExGetColliderVelocity < 'a > {
    surround_object: &'a re_export::PhysicsTestMotionResult3D, collision_index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetColliderVelocity < 'a > {
    fn new(surround_object: &'a re_export::PhysicsTestMotionResult3D,) -> Self {
        Self {
            surround_object, collision_index: 0i32,
        }
    }
    #[inline]
    pub fn collision_index(self, value: i32) -> Self {
        Self {
            collision_index: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Vector3 {
        re_export::PhysicsTestMotionResult3D::get_collider_velocity_full(self.surround_object, self.collision_index,)
    }
}
#[doc = "Default-param extender for [`PhysicsTestMotionResult3D::get_collider_id_ex`][super::PhysicsTestMotionResult3D::get_collider_id_ex]."]
#[must_use]
pub struct ExGetColliderId < 'a > {
    surround_object: &'a re_export::PhysicsTestMotionResult3D, collision_index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetColliderId < 'a > {
    fn new(surround_object: &'a re_export::PhysicsTestMotionResult3D,) -> Self {
        Self {
            surround_object, collision_index: 0i32,
        }
    }
    #[inline]
    pub fn collision_index(self, value: i32) -> Self {
        Self {
            collision_index: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> u64 {
        re_export::PhysicsTestMotionResult3D::get_collider_id_full(self.surround_object, self.collision_index,)
    }
}
#[doc = "Default-param extender for [`PhysicsTestMotionResult3D::get_collider_rid_ex`][super::PhysicsTestMotionResult3D::get_collider_rid_ex]."]
#[must_use]
pub struct ExGetColliderRid < 'a > {
    surround_object: &'a re_export::PhysicsTestMotionResult3D, collision_index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetColliderRid < 'a > {
    fn new(surround_object: &'a re_export::PhysicsTestMotionResult3D,) -> Self {
        Self {
            surround_object, collision_index: 0i32,
        }
    }
    #[inline]
    pub fn collision_index(self, value: i32) -> Self {
        Self {
            collision_index: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Rid {
        re_export::PhysicsTestMotionResult3D::get_collider_rid_full(self.surround_object, self.collision_index,)
    }
}
#[doc = "Default-param extender for [`PhysicsTestMotionResult3D::get_collider_ex`][super::PhysicsTestMotionResult3D::get_collider_ex]."]
#[must_use]
pub struct ExGetCollider < 'a > {
    surround_object: &'a re_export::PhysicsTestMotionResult3D, collision_index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetCollider < 'a > {
    fn new(surround_object: &'a re_export::PhysicsTestMotionResult3D,) -> Self {
        Self {
            surround_object, collision_index: 0i32,
        }
    }
    #[inline]
    pub fn collision_index(self, value: i32) -> Self {
        Self {
            collision_index: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::engine::Object > > {
        re_export::PhysicsTestMotionResult3D::get_collider_full(self.surround_object, self.collision_index,)
    }
}
#[doc = "Default-param extender for [`PhysicsTestMotionResult3D::get_collider_shape_ex`][super::PhysicsTestMotionResult3D::get_collider_shape_ex]."]
#[must_use]
pub struct ExGetColliderShape < 'a > {
    surround_object: &'a re_export::PhysicsTestMotionResult3D, collision_index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetColliderShape < 'a > {
    fn new(surround_object: &'a re_export::PhysicsTestMotionResult3D,) -> Self {
        Self {
            surround_object, collision_index: 0i32,
        }
    }
    #[inline]
    pub fn collision_index(self, value: i32) -> Self {
        Self {
            collision_index: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        re_export::PhysicsTestMotionResult3D::get_collider_shape_full(self.surround_object, self.collision_index,)
    }
}
#[doc = "Default-param extender for [`PhysicsTestMotionResult3D::get_collision_local_shape_ex`][super::PhysicsTestMotionResult3D::get_collision_local_shape_ex]."]
#[must_use]
pub struct ExGetCollisionLocalShape < 'a > {
    surround_object: &'a re_export::PhysicsTestMotionResult3D, collision_index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetCollisionLocalShape < 'a > {
    fn new(surround_object: &'a re_export::PhysicsTestMotionResult3D,) -> Self {
        Self {
            surround_object, collision_index: 0i32,
        }
    }
    #[inline]
    pub fn collision_index(self, value: i32) -> Self {
        Self {
            collision_index: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        re_export::PhysicsTestMotionResult3D::get_collision_local_shape_full(self.surround_object, self.collision_index,)
    }
}
#[doc = "Default-param extender for [`PhysicsTestMotionResult3D::get_collision_depth_ex`][super::PhysicsTestMotionResult3D::get_collision_depth_ex]."]
#[must_use]
pub struct ExGetCollisionDepth < 'a > {
    surround_object: &'a re_export::PhysicsTestMotionResult3D, collision_index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetCollisionDepth < 'a > {
    fn new(surround_object: &'a re_export::PhysicsTestMotionResult3D,) -> Self {
        Self {
            surround_object, collision_index: 0i32,
        }
    }
    #[inline]
    pub fn collision_index(self, value: i32) -> Self {
        Self {
            collision_index: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> f32 {
        re_export::PhysicsTestMotionResult3D::get_collision_depth_full(self.surround_object, self.collision_index,)
    }
}