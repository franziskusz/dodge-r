#![doc = "Sidecar module for class [`PhysicsDirectSpaceState2D`][crate::engine::PhysicsDirectSpaceState2D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `PhysicsDirectSpaceState2D` enums](https://docs.godotengine.org/en/stable/classes/class_physicsdirectspacestate2d.html#enumerations).\n\n"]
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
    #[doc = "Godot class `PhysicsDirectSpaceState2D.`\n\nInherits [`Object`][crate::engine::Object].\n\nRelated symbols:\n\n* [`physics_direct_space_state_2d`][crate::engine::physics_direct_space_state_2d]: sidecar module with related enum/flag types\n* [`IPhysicsDirectSpaceState2D`][crate::engine::IPhysicsDirectSpaceState2D]: virtual methods\n\n\nSee also [Godot docs for `PhysicsDirectSpaceState2D`](https://docs.godotengine.org/en/stable/classes/class_physicsdirectspacestate2d.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct PhysicsDirectSpaceState2D {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`PhysicsDirectSpaceState2D`][crate::engine::PhysicsDirectSpaceState2D].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `PhysicsDirectSpaceState2D` methods](https://docs.godotengine.org/en/stable/classes/class_physicsdirectspacestate2d.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IPhysicsDirectSpaceState2D: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
    impl PhysicsDirectSpaceState2D {
        pub(crate) fn intersect_point_full(&mut self, parameters: Gd < crate::engine::PhysicsPointQueryParameters2D >, max_results: i32,) -> Array < Dictionary > {
            type RetMarshal = PtrcallReturnT < Array < Dictionary > >;
            type CallSig = (Array < Dictionary >, Gd < crate::engine::PhysicsPointQueryParameters2D >, i32);
            let args = (parameters, max_results,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5771usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "intersect_point", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn intersect_point(&mut self, parameters: Gd < crate::engine::PhysicsPointQueryParameters2D >,) -> Array < Dictionary > {
            self.intersect_point_ex(parameters,) . done()
        }
        #[inline]
        pub fn intersect_point_ex(&mut self, parameters: Gd < crate::engine::PhysicsPointQueryParameters2D >,) -> ExIntersectPoint < '_ > {
            ExIntersectPoint::new(self, parameters,)
        }
        pub fn intersect_ray(&mut self, parameters: Gd < crate::engine::PhysicsRayQueryParameters2D >,) -> Dictionary {
            type RetMarshal = PtrcallReturnT < Dictionary >;
            type CallSig = (Dictionary, Gd < crate::engine::PhysicsRayQueryParameters2D >);
            let args = (parameters,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5772usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "intersect_ray", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn intersect_shape_full(&mut self, parameters: Gd < crate::engine::PhysicsShapeQueryParameters2D >, max_results: i32,) -> Array < Dictionary > {
            type RetMarshal = PtrcallReturnT < Array < Dictionary > >;
            type CallSig = (Array < Dictionary >, Gd < crate::engine::PhysicsShapeQueryParameters2D >, i32);
            let args = (parameters, max_results,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5773usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "intersect_shape", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn intersect_shape(&mut self, parameters: Gd < crate::engine::PhysicsShapeQueryParameters2D >,) -> Array < Dictionary > {
            self.intersect_shape_ex(parameters,) . done()
        }
        #[inline]
        pub fn intersect_shape_ex(&mut self, parameters: Gd < crate::engine::PhysicsShapeQueryParameters2D >,) -> ExIntersectShape < '_ > {
            ExIntersectShape::new(self, parameters,)
        }
        pub fn cast_motion(&mut self, parameters: Gd < crate::engine::PhysicsShapeQueryParameters2D >,) -> PackedFloat32Array {
            type RetMarshal = PtrcallReturnT < PackedFloat32Array >;
            type CallSig = (PackedFloat32Array, Gd < crate::engine::PhysicsShapeQueryParameters2D >);
            let args = (parameters,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5774usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "cast_motion", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn collide_shape_full(&mut self, parameters: Gd < crate::engine::PhysicsShapeQueryParameters2D >, max_results: i32,) -> Array < Vector2 > {
            type RetMarshal = PtrcallReturnT < Array < Vector2 > >;
            type CallSig = (Array < Vector2 >, Gd < crate::engine::PhysicsShapeQueryParameters2D >, i32);
            let args = (parameters, max_results,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5775usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "collide_shape", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn collide_shape(&mut self, parameters: Gd < crate::engine::PhysicsShapeQueryParameters2D >,) -> Array < Vector2 > {
            self.collide_shape_ex(parameters,) . done()
        }
        #[inline]
        pub fn collide_shape_ex(&mut self, parameters: Gd < crate::engine::PhysicsShapeQueryParameters2D >,) -> ExCollideShape < '_ > {
            ExCollideShape::new(self, parameters,)
        }
        pub fn get_rest_info(&mut self, parameters: Gd < crate::engine::PhysicsShapeQueryParameters2D >,) -> Dictionary {
            type RetMarshal = PtrcallReturnT < Dictionary >;
            type CallSig = (Dictionary, Gd < crate::engine::PhysicsShapeQueryParameters2D >);
            let args = (parameters,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5776usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_rest_info", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for PhysicsDirectSpaceState2D {
        type Base = crate::engine::Object;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"PhysicsDirectSpaceState2D\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for PhysicsDirectSpaceState2D {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for PhysicsDirectSpaceState2D {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::Object > for PhysicsDirectSpaceState2D {
        
    }
    impl std::ops::Deref for PhysicsDirectSpaceState2D {
        type Target = crate::engine::Object;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for PhysicsDirectSpaceState2D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_PhysicsDirectSpaceState2D {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::PhysicsDirectSpaceState2D > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`PhysicsDirectSpaceState2D::intersect_point_ex`][super::PhysicsDirectSpaceState2D::intersect_point_ex]."]
#[must_use]
pub struct ExIntersectPoint < 'a > {
    surround_object: &'a mut re_export::PhysicsDirectSpaceState2D, parameters: Gd < crate::engine::PhysicsPointQueryParameters2D >, max_results: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExIntersectPoint < 'a > {
    fn new(surround_object: &'a mut re_export::PhysicsDirectSpaceState2D, parameters: Gd < crate::engine::PhysicsPointQueryParameters2D >,) -> Self {
        Self {
            surround_object, parameters, max_results: 32i32,
        }
    }
    #[inline]
    pub fn max_results(self, value: i32) -> Self {
        Self {
            max_results: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Array < Dictionary > {
        re_export::PhysicsDirectSpaceState2D::intersect_point_full(self.surround_object, self.parameters, self.max_results,)
    }
}
#[doc = "Default-param extender for [`PhysicsDirectSpaceState2D::intersect_shape_ex`][super::PhysicsDirectSpaceState2D::intersect_shape_ex]."]
#[must_use]
pub struct ExIntersectShape < 'a > {
    surround_object: &'a mut re_export::PhysicsDirectSpaceState2D, parameters: Gd < crate::engine::PhysicsShapeQueryParameters2D >, max_results: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExIntersectShape < 'a > {
    fn new(surround_object: &'a mut re_export::PhysicsDirectSpaceState2D, parameters: Gd < crate::engine::PhysicsShapeQueryParameters2D >,) -> Self {
        Self {
            surround_object, parameters, max_results: 32i32,
        }
    }
    #[inline]
    pub fn max_results(self, value: i32) -> Self {
        Self {
            max_results: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Array < Dictionary > {
        re_export::PhysicsDirectSpaceState2D::intersect_shape_full(self.surround_object, self.parameters, self.max_results,)
    }
}
#[doc = "Default-param extender for [`PhysicsDirectSpaceState2D::collide_shape_ex`][super::PhysicsDirectSpaceState2D::collide_shape_ex]."]
#[must_use]
pub struct ExCollideShape < 'a > {
    surround_object: &'a mut re_export::PhysicsDirectSpaceState2D, parameters: Gd < crate::engine::PhysicsShapeQueryParameters2D >, max_results: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExCollideShape < 'a > {
    fn new(surround_object: &'a mut re_export::PhysicsDirectSpaceState2D, parameters: Gd < crate::engine::PhysicsShapeQueryParameters2D >,) -> Self {
        Self {
            surround_object, parameters, max_results: 32i32,
        }
    }
    #[inline]
    pub fn max_results(self, value: i32) -> Self {
        Self {
            max_results: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Array < Vector2 > {
        re_export::PhysicsDirectSpaceState2D::collide_shape_full(self.surround_object, self.parameters, self.max_results,)
    }
}