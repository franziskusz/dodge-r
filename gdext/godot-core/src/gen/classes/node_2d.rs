#![doc = "Sidecar module for class [`Node2D`][crate::engine::Node2D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `Node2D` enums](https://docs.godotengine.org/en/stable/classes/class_node2d.html#enumerations).\n\n"]
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
    #[doc = "Godot class `Node2D.`\n\nInherits [`CanvasItem`][crate::engine::CanvasItem].\n\nRelated symbols:\n\n* [`node_2d`][crate::engine::node_2d]: sidecar module with related enum/flag types\n* [`INode2D`][crate::engine::INode2D]: virtual methods\n\n\nSee also [Godot docs for `Node2D`](https://docs.godotengine.org/en/stable/classes/class_node2d.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct Node2D {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`Node2D`][crate::engine::Node2D].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `Node2D` methods](https://docs.godotengine.org/en/stable/classes/class_node2d.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait INode2D: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
    impl Node2D {
        pub fn set_position(&mut self, position: Vector2,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Vector2);
            let args = (position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4986usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_position", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_rotation(&mut self, radians: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (radians,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4987usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_rotation", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_rotation_degrees(&mut self, degrees: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (degrees,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4988usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_rotation_degrees", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_skew(&mut self, radians: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (radians,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4989usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_skew", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_scale(&mut self, scale: Vector2,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Vector2);
            let args = (scale,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4990usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_position(&self,) -> Vector2 {
            type RetMarshal = PtrcallReturnT < Vector2 >;
            type CallSig = (Vector2,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4991usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_position", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_rotation(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4992usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_rotation", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_rotation_degrees(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4993usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_rotation_degrees", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_skew(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4994usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_skew", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_scale(&self,) -> Vector2 {
            type RetMarshal = PtrcallReturnT < Vector2 >;
            type CallSig = (Vector2,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4995usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn rotate(&mut self, radians: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (radians,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4996usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "rotate", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn move_local_x_full(&mut self, delta: f32, scaled: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32, bool);
            let args = (delta, scaled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4997usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "move_local_x", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn move_local_x(&mut self, delta: f32,) {
            self.move_local_x_ex(delta,) . done()
        }
        #[inline]
        pub fn move_local_x_ex(&mut self, delta: f32,) -> ExMoveLocalX < '_ > {
            ExMoveLocalX::new(self, delta,)
        }
        pub(crate) fn move_local_y_full(&mut self, delta: f32, scaled: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32, bool);
            let args = (delta, scaled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4998usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "move_local_y", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn move_local_y(&mut self, delta: f32,) {
            self.move_local_y_ex(delta,) . done()
        }
        #[inline]
        pub fn move_local_y_ex(&mut self, delta: f32,) -> ExMoveLocalY < '_ > {
            ExMoveLocalY::new(self, delta,)
        }
        pub fn translate(&mut self, offset: Vector2,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Vector2);
            let args = (offset,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4999usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "translate", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn global_translate(&mut self, offset: Vector2,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Vector2);
            let args = (offset,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5000usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "global_translate", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn apply_scale(&mut self, ratio: Vector2,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Vector2);
            let args = (ratio,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5001usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "apply_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_global_position(&mut self, position: Vector2,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Vector2);
            let args = (position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5002usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_global_position", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_global_position(&self,) -> Vector2 {
            type RetMarshal = PtrcallReturnT < Vector2 >;
            type CallSig = (Vector2,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5003usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_global_position", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_global_rotation(&mut self, radians: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (radians,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5004usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_global_rotation", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_global_rotation_degrees(&mut self, degrees: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (degrees,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5005usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_global_rotation_degrees", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_global_rotation(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5006usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_global_rotation", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_global_rotation_degrees(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5007usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_global_rotation_degrees", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_global_skew(&mut self, radians: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (radians,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5008usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_global_skew", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_global_skew(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5009usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_global_skew", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_global_scale(&mut self, scale: Vector2,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Vector2);
            let args = (scale,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5010usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_global_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_global_scale(&self,) -> Vector2 {
            type RetMarshal = PtrcallReturnT < Vector2 >;
            type CallSig = (Vector2,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5011usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_global_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_transform(&mut self, xform: Transform2D,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Transform2D);
            let args = (xform,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5012usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_transform", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_global_transform(&mut self, xform: Transform2D,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Transform2D);
            let args = (xform,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5013usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_global_transform", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn look_at(&mut self, point: Vector2,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Vector2);
            let args = (point,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5014usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "look_at", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_angle_to(&self, point: Vector2,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32, Vector2);
            let args = (point,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5015usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_angle_to", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn to_local(&self, global_point: Vector2,) -> Vector2 {
            type RetMarshal = PtrcallReturnT < Vector2 >;
            type CallSig = (Vector2, Vector2);
            let args = (global_point,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5016usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "to_local", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn to_global(&self, local_point: Vector2,) -> Vector2 {
            type RetMarshal = PtrcallReturnT < Vector2 >;
            type CallSig = (Vector2, Vector2);
            let args = (local_point,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5017usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "to_global", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_relative_transform_to_parent(&self, parent: Gd < crate::engine::Node >,) -> Transform2D {
            type RetMarshal = PtrcallReturnT < Transform2D >;
            type CallSig = (Transform2D, Gd < crate::engine::Node >);
            let args = (parent,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5018usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_relative_transform_to_parent", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for Node2D {
        type Base = crate::engine::CanvasItem;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"Node2D\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for Node2D {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for Node2D {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::CanvasItem > for Node2D {
        
    }
    impl crate::obj::Inherits < crate::engine::Node > for Node2D {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for Node2D {
        
    }
    impl crate::obj::ExportableObject for Node2D {
        
    }
    impl crate::obj::cap::GodotDefault for Node2D {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for Node2D {
        type Target = crate::engine::CanvasItem;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for Node2D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_Node2D {
        ($Class: ident) => {
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
#[doc = "Default-param extender for [`Node2D::move_local_x_ex`][super::Node2D::move_local_x_ex]."]
#[must_use]
pub struct ExMoveLocalX < 'a > {
    surround_object: &'a mut re_export::Node2D, delta: f32, scaled: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExMoveLocalX < 'a > {
    fn new(surround_object: &'a mut re_export::Node2D, delta: f32,) -> Self {
        Self {
            surround_object, delta, scaled: false,
        }
    }
    #[inline]
    pub fn scaled(self, value: bool) -> Self {
        Self {
            scaled: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::Node2D::move_local_x_full(self.surround_object, self.delta, self.scaled,)
    }
}
#[doc = "Default-param extender for [`Node2D::move_local_y_ex`][super::Node2D::move_local_y_ex]."]
#[must_use]
pub struct ExMoveLocalY < 'a > {
    surround_object: &'a mut re_export::Node2D, delta: f32, scaled: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExMoveLocalY < 'a > {
    fn new(surround_object: &'a mut re_export::Node2D, delta: f32,) -> Self {
        Self {
            surround_object, delta, scaled: false,
        }
    }
    #[inline]
    pub fn scaled(self, value: bool) -> Self {
        Self {
            scaled: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::Node2D::move_local_y_full(self.surround_object, self.delta, self.scaled,)
    }
}