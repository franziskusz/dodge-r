#![doc = "Sidecar module for class [`PhysicsRayQueryParameters3D`][crate::engine::PhysicsRayQueryParameters3D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `PhysicsRayQueryParameters3D` enums](https://docs.godotengine.org/en/stable/classes/class_physicsrayqueryparameters3d.html#enumerations).\n\n"]
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
    #[doc = "Godot class `PhysicsRayQueryParameters3D.`\n\nInherits [`RefCounted`][crate::engine::RefCounted].\n\nRelated symbols:\n\n* [`physics_ray_query_parameters_3d`][crate::engine::physics_ray_query_parameters_3d]: sidecar module with related enum/flag types\n* [`IPhysicsRayQueryParameters3D`][crate::engine::IPhysicsRayQueryParameters3D]: virtual methods\n\n\nSee also [Godot docs for `PhysicsRayQueryParameters3D`](https://docs.godotengine.org/en/stable/classes/class_physicsrayqueryparameters3d.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct PhysicsRayQueryParameters3D {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`PhysicsRayQueryParameters3D`][crate::engine::PhysicsRayQueryParameters3D].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `PhysicsRayQueryParameters3D` methods](https://docs.godotengine.org/en/stable/classes/class_physicsrayqueryparameters3d.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IPhysicsRayQueryParameters3D: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
    impl PhysicsRayQueryParameters3D {
        #[deprecated = "Replaced with `new_gd` in extension trait `NewGd`."]
        pub fn new() -> Gd < Self > {
            crate::obj::Gd::default()
        }
        pub(crate) fn create_full(from: Vector3, to: Vector3, collision_mask: u32, exclude: Array < Rid >,) -> Option < Gd < crate::engine::PhysicsRayQueryParameters3D > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::PhysicsRayQueryParameters3D > >;
            type CallSig = (Option < Gd < crate::engine::PhysicsRayQueryParameters3D > >, Vector3, Vector3, u32, Array < Rid >);
            let args = (from, to, collision_mask, exclude,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5830usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "create", std::ptr::null_mut(), None, args,)
            }
        }
        #[inline]
        pub fn create(from: Vector3, to: Vector3,) -> Option < Gd < crate::engine::PhysicsRayQueryParameters3D > > {
            Self::create_ex(from, to,) . done()
        }
        #[inline]
        pub fn create_ex(from: Vector3, to: Vector3,) -> ExCreate {
            ExCreate::new(from, to,)
        }
        pub fn set_from(&mut self, from: Vector3,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Vector3);
            let args = (from,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5831usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_from", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_from(&self,) -> Vector3 {
            type RetMarshal = PtrcallReturnT < Vector3 >;
            type CallSig = (Vector3,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5832usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_from", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_to(&mut self, to: Vector3,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Vector3);
            let args = (to,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5833usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_to", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_to(&self,) -> Vector3 {
            type RetMarshal = PtrcallReturnT < Vector3 >;
            type CallSig = (Vector3,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5834usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_to", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_collision_mask(&mut self, collision_mask: u32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), u32);
            let args = (collision_mask,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5835usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_collision_mask", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_collision_mask(&self,) -> u32 {
            type RetMarshal = PtrcallReturnT < u32 >;
            type CallSig = (u32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5836usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_collision_mask", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_exclude(&mut self, exclude: Array < Rid >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Array < Rid >);
            let args = (exclude,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5837usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_exclude", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_exclude(&self,) -> Array < Rid > {
            type RetMarshal = PtrcallReturnT < Array < Rid > >;
            type CallSig = (Array < Rid >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5838usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_exclude", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_collide_with_bodies(&mut self, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5839usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_collide_with_bodies", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_collide_with_bodies_enabled(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5840usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_collide_with_bodies_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_collide_with_areas(&mut self, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5841usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_collide_with_areas", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_collide_with_areas_enabled(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5842usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_collide_with_areas_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_hit_from_inside(&mut self, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5843usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_hit_from_inside", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_hit_from_inside_enabled(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5844usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_hit_from_inside_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_hit_back_faces(&mut self, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5845usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_hit_back_faces", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_hit_back_faces_enabled(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5846usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_hit_back_faces_enabled", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for PhysicsRayQueryParameters3D {
        type Base = crate::engine::RefCounted;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"PhysicsRayQueryParameters3D\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for PhysicsRayQueryParameters3D {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for PhysicsRayQueryParameters3D {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::RefCounted > for PhysicsRayQueryParameters3D {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for PhysicsRayQueryParameters3D {
        
    }
    impl crate::obj::cap::GodotDefault for PhysicsRayQueryParameters3D {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for PhysicsRayQueryParameters3D {
        type Target = crate::engine::RefCounted;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for PhysicsRayQueryParameters3D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_PhysicsRayQueryParameters3D {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::PhysicsRayQueryParameters3D > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::RefCounted > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`PhysicsRayQueryParameters3D::create_ex`][super::PhysicsRayQueryParameters3D::create_ex]."]
#[must_use]
pub struct ExCreate {
    from: Vector3, to: Vector3, collision_mask: u32, exclude: Array < Rid >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl ExCreate {
    fn new(from: Vector3, to: Vector3,) -> Self {
        Self {
            from, to, collision_mask: 4294967295u32, exclude: Array::new(),
        }
    }
    #[inline]
    pub fn collision_mask(self, value: u32) -> Self {
        Self {
            collision_mask: value, .. self
        }
    }
    #[inline]
    pub fn exclude(self, value: Array < Rid >) -> Self {
        Self {
            exclude: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::engine::PhysicsRayQueryParameters3D > > {
        re_export::PhysicsRayQueryParameters3D::create_full(self.from, self.to, self.collision_mask, self.exclude,)
    }
}