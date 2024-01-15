#![doc = "Sidecar module for class [`RayCast2D`][crate::engine::RayCast2D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `RayCast2D` enums](https://docs.godotengine.org/en/stable/classes/class_raycast2d.html#enumerations).\n\n"]
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
    #[doc = "Godot class `RayCast2D.`\n\nInherits [`Node2D`][crate::engine::Node2D].\n\nRelated symbols:\n\n* [`IRayCast2D`][crate::engine::IRayCast2D]: virtual methods\n\n\nSee also [Godot docs for `RayCast2D`](https://docs.godotengine.org/en/stable/classes/class_raycast2d.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct RayCast2D {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`RayCast2D`][crate::engine::RayCast2D].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `RayCast2D` methods](https://docs.godotengine.org/en/stable/classes/class_raycast2d.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IRayCast2D: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
    impl RayCast2D {
        pub fn set_enabled(&mut self, enabled: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6699usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_enabled(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6700usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_target_position(&mut self, local_point: Vector2,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Vector2);
            let args = (local_point,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6701usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_target_position", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_target_position(&self,) -> Vector2 {
            type RetMarshal = PtrcallReturnT < Vector2 >;
            type CallSig = (Vector2,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6702usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_target_position", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_colliding(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6703usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_colliding", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn force_raycast_update(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6704usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "force_raycast_update", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_collider(&self,) -> Option < Gd < crate::engine::Object > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::Object > >;
            type CallSig = (Option < Gd < crate::engine::Object > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6705usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_collider", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_collider_rid(&self,) -> Rid {
            type RetMarshal = PtrcallReturnT < Rid >;
            type CallSig = (Rid,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6706usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_collider_rid", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_collider_shape(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6707usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_collider_shape", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_collision_point(&self,) -> Vector2 {
            type RetMarshal = PtrcallReturnT < Vector2 >;
            type CallSig = (Vector2,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6708usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_collision_point", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_collision_normal(&self,) -> Vector2 {
            type RetMarshal = PtrcallReturnT < Vector2 >;
            type CallSig = (Vector2,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6709usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_collision_normal", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn add_exception_rid(&mut self, rid: Rid,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid);
            let args = (rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6710usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_exception_rid", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn add_exception(&mut self, node: Gd < crate::engine::CollisionObject2D >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::CollisionObject2D >);
            let args = (node,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6711usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_exception", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn remove_exception_rid(&mut self, rid: Rid,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Rid);
            let args = (rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6712usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "remove_exception_rid", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn remove_exception(&mut self, node: Gd < crate::engine::CollisionObject2D >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::CollisionObject2D >);
            let args = (node,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6713usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "remove_exception", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn clear_exceptions(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6714usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "clear_exceptions", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_collision_mask(&mut self, mask: u32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), u32);
            let args = (mask,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6715usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_collision_mask", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_collision_mask(&self,) -> u32 {
            type RetMarshal = PtrcallReturnT < u32 >;
            type CallSig = (u32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6716usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_collision_mask", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_collision_mask_value(&mut self, layer_number: i32, value: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, bool);
            let args = (layer_number, value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6717usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_collision_mask_value", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_collision_mask_value(&self, layer_number: i32,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, i32);
            let args = (layer_number,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6718usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_collision_mask_value", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_exclude_parent_body(&mut self, mask: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (mask,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6719usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_exclude_parent_body", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_exclude_parent_body(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6720usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_exclude_parent_body", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_collide_with_areas(&mut self, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6721usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_collide_with_areas", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_collide_with_areas_enabled(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6722usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_collide_with_areas_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_collide_with_bodies(&mut self, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6723usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_collide_with_bodies", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_collide_with_bodies_enabled(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6724usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_collide_with_bodies_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_hit_from_inside(&mut self, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6725usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_hit_from_inside", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_hit_from_inside_enabled(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6726usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_hit_from_inside_enabled", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for RayCast2D {
        type Base = crate::engine::Node2D;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"RayCast2D\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for RayCast2D {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for RayCast2D {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::Node2D > for RayCast2D {
        
    }
    impl crate::obj::Inherits < crate::engine::CanvasItem > for RayCast2D {
        
    }
    impl crate::obj::Inherits < crate::engine::Node > for RayCast2D {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for RayCast2D {
        
    }
    impl crate::obj::ExportableObject for RayCast2D {
        
    }
    impl crate::obj::cap::GodotDefault for RayCast2D {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for RayCast2D {
        type Target = crate::engine::Node2D;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for RayCast2D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_RayCast2D {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::RayCast2D > for $Class {
                
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