#![doc = "Sidecar module for class [`PhysicsTestMotionParameters2D`][crate::engine::PhysicsTestMotionParameters2D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `PhysicsTestMotionParameters2D` enums](https://docs.godotengine.org/en/stable/classes/class_physicstestmotionparameters2d.html#enumerations).\n\n"]
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
    #[doc = "Godot class `PhysicsTestMotionParameters2D.`\n\nInherits [`RefCounted`][crate::engine::RefCounted].\n\nRelated symbols:\n\n* [`IPhysicsTestMotionParameters2D`][crate::engine::IPhysicsTestMotionParameters2D]: virtual methods\n\n\nSee also [Godot docs for `PhysicsTestMotionParameters2D`](https://docs.godotengine.org/en/stable/classes/class_physicstestmotionparameters2d.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct PhysicsTestMotionParameters2D {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`PhysicsTestMotionParameters2D`][crate::engine::PhysicsTestMotionParameters2D].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `PhysicsTestMotionParameters2D` methods](https://docs.godotengine.org/en/stable/classes/class_physicstestmotionparameters2d.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IPhysicsTestMotionParameters2D: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
    impl PhysicsTestMotionParameters2D {
        #[deprecated = "Replaced with `new_gd` in extension trait `NewGd`."]
        pub fn new() -> Gd < Self > {
            crate::obj::Gd::default()
        }
        pub fn get_from(&self,) -> Transform2D {
            type RetMarshal = PtrcallReturnT < Transform2D >;
            type CallSig = (Transform2D,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6146usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_from", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_from(&mut self, from: Transform2D,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Transform2D);
            let args = (from,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6147usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_from", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_motion(&self,) -> Vector2 {
            type RetMarshal = PtrcallReturnT < Vector2 >;
            type CallSig = (Vector2,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6148usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_motion", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_motion(&mut self, motion: Vector2,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Vector2);
            let args = (motion,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6149usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_motion", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_margin(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6150usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_margin", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_margin(&mut self, margin: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (margin,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6151usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_margin", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_collide_separation_ray_enabled(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6152usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_collide_separation_ray_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_collide_separation_ray_enabled(&mut self, enabled: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6153usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_collide_separation_ray_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_exclude_bodies(&self,) -> Array < Rid > {
            type RetMarshal = PtrcallReturnT < Array < Rid > >;
            type CallSig = (Array < Rid >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6154usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_exclude_bodies", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_exclude_bodies(&mut self, exclude_list: Array < Rid >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Array < Rid >);
            let args = (exclude_list,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6155usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_exclude_bodies", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_exclude_objects(&self,) -> Array < i64 > {
            type RetMarshal = PtrcallReturnT < Array < i64 > >;
            type CallSig = (Array < i64 >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6156usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_exclude_objects", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_exclude_objects(&mut self, exclude_list: Array < i64 >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Array < i64 >);
            let args = (exclude_list,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6157usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_exclude_objects", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_recovery_as_collision_enabled(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6158usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_recovery_as_collision_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_recovery_as_collision_enabled(&mut self, enabled: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6159usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_recovery_as_collision_enabled", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for PhysicsTestMotionParameters2D {
        type Base = crate::engine::RefCounted;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"PhysicsTestMotionParameters2D\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for PhysicsTestMotionParameters2D {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for PhysicsTestMotionParameters2D {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::RefCounted > for PhysicsTestMotionParameters2D {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for PhysicsTestMotionParameters2D {
        
    }
    impl crate::obj::cap::GodotDefault for PhysicsTestMotionParameters2D {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for PhysicsTestMotionParameters2D {
        type Target = crate::engine::RefCounted;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for PhysicsTestMotionParameters2D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_PhysicsTestMotionParameters2D {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::PhysicsTestMotionParameters2D > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::RefCounted > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::Object > for $Class {
                
            }
        }
    }
}