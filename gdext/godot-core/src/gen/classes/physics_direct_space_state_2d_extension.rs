#![doc = "Sidecar module for class [`PhysicsDirectSpaceState2DExtension`][crate::engine::PhysicsDirectSpaceState2DExtension].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `PhysicsDirectSpaceState2DExtension` enums](https://docs.godotengine.org/en/stable/classes/class_physicsdirectspacestate2dextension.html#enumerations).\n\n"]
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
    #[doc = "Godot class `PhysicsDirectSpaceState2DExtension.`\n\nInherits [`PhysicsDirectSpaceState2D`][crate::engine::PhysicsDirectSpaceState2D].\n\nRelated symbols:\n\n* [`IPhysicsDirectSpaceState2DExtension`][crate::engine::IPhysicsDirectSpaceState2DExtension]: virtual methods\n\n\nSee also [Godot docs for `PhysicsDirectSpaceState2DExtension`](https://docs.godotengine.org/en/stable/classes/class_physicsdirectspacestate2dextension.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct PhysicsDirectSpaceState2DExtension {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`PhysicsDirectSpaceState2DExtension`][crate::engine::PhysicsDirectSpaceState2DExtension].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `PhysicsDirectSpaceState2DExtension` methods](https://docs.godotengine.org/en/stable/classes/class_physicsdirectspacestate2dextension.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IPhysicsDirectSpaceState2DExtension: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" Godot currently does not document safety requirements on this method. Make sure you understand the underlying semantics."]
        unsafe fn intersect_ray(&mut self, from: Vector2, to: Vector2, collision_mask: u32, collide_with_bodies: bool, collide_with_areas: bool, hit_from_inside: bool, result: * mut PhysicsServer2DExtensionRayResult,) -> bool {
            unimplemented !()
        }
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" Godot currently does not document safety requirements on this method. Make sure you understand the underlying semantics."]
        unsafe fn intersect_point(&mut self, position: Vector2, canvas_instance_id: u64, collision_mask: u32, collide_with_bodies: bool, collide_with_areas: bool, results: * mut PhysicsServer2DExtensionShapeResult, max_results: i32,) -> i32 {
            unimplemented !()
        }
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" Godot currently does not document safety requirements on this method. Make sure you understand the underlying semantics."]
        unsafe fn intersect_shape(&mut self, shape_rid: Rid, transform: Transform2D, motion: Vector2, margin: f32, collision_mask: u32, collide_with_bodies: bool, collide_with_areas: bool, result: * mut PhysicsServer2DExtensionShapeResult, max_results: i32,) -> i32 {
            unimplemented !()
        }
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" Godot currently does not document safety requirements on this method. Make sure you understand the underlying semantics."]
        unsafe fn cast_motion(&mut self, shape_rid: Rid, transform: Transform2D, motion: Vector2, margin: f32, collision_mask: u32, collide_with_bodies: bool, collide_with_areas: bool, closest_safe: * mut f64, closest_unsafe: * mut f64,) -> bool {
            unimplemented !()
        }
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" Godot currently does not document safety requirements on this method. Make sure you understand the underlying semantics."]
        unsafe fn collide_shape(&mut self, shape_rid: Rid, transform: Transform2D, motion: Vector2, margin: f32, collision_mask: u32, collide_with_bodies: bool, collide_with_areas: bool, results: * mut c_void, max_results: i32, result_count: * mut i32,) -> bool {
            unimplemented !()
        }
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" Godot currently does not document safety requirements on this method. Make sure you understand the underlying semantics."]
        unsafe fn rest_info(&mut self, shape_rid: Rid, transform: Transform2D, motion: Vector2, margin: f32, collision_mask: u32, collide_with_bodies: bool, collide_with_areas: bool, rest_info: * mut PhysicsServer2DExtensionShapeRestInfo,) -> bool {
            unimplemented !()
        }
    }
    impl PhysicsDirectSpaceState2DExtension {
        pub fn is_body_excluded_from_query(&self, body: Rid,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, Rid);
            let args = (body,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5777usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_body_excluded_from_query", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for PhysicsDirectSpaceState2DExtension {
        type Base = crate::engine::PhysicsDirectSpaceState2D;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"PhysicsDirectSpaceState2DExtension\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for PhysicsDirectSpaceState2DExtension {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for PhysicsDirectSpaceState2DExtension {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::PhysicsDirectSpaceState2D > for PhysicsDirectSpaceState2DExtension {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for PhysicsDirectSpaceState2DExtension {
        
    }
    impl crate::obj::cap::GodotDefault for PhysicsDirectSpaceState2DExtension {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for PhysicsDirectSpaceState2DExtension {
        type Target = crate::engine::PhysicsDirectSpaceState2D;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for PhysicsDirectSpaceState2DExtension {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_PhysicsDirectSpaceState2DExtension {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::PhysicsDirectSpaceState2DExtension > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::PhysicsDirectSpaceState2D > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::Object > for $Class {
                
            }
        }
    }
}