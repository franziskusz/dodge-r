#![doc = "Sidecar module for class [`PhysicalBone2D`][crate::engine::PhysicalBone2D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `PhysicalBone2D` enums](https://docs.godotengine.org/en/stable/classes/class_physicalbone2d.html#enumerations).\n\n"]
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
    #[doc = "Godot class `PhysicalBone2D.`\n\nInherits [`RigidBody2D`][crate::engine::RigidBody2D].\n\nRelated symbols:\n\n* [`IPhysicalBone2D`][crate::engine::IPhysicalBone2D]: virtual methods\n\n\nSee also [Godot docs for `PhysicalBone2D`](https://docs.godotengine.org/en/stable/classes/class_physicalbone2d.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct PhysicalBone2D {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`PhysicalBone2D`][crate::engine::PhysicalBone2D].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `PhysicalBone2D` methods](https://docs.godotengine.org/en/stable/classes/class_physicalbone2d.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IPhysicalBone2D: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
        fn integrate_forces(&mut self, state: Gd < crate::engine::PhysicsDirectBodyState2D >,) {
            unimplemented !()
        }
        fn input_event(&mut self, viewport: Gd < crate::engine::Viewport >, event: Gd < crate::engine::InputEvent >, shape_idx: i32,) {
            unimplemented !()
        }
        fn mouse_enter(&mut self,) {
            unimplemented !()
        }
        fn mouse_exit(&mut self,) {
            unimplemented !()
        }
        fn mouse_shape_enter(&mut self, shape_idx: i32,) {
            unimplemented !()
        }
        fn mouse_shape_exit(&mut self, shape_idx: i32,) {
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
    impl PhysicalBone2D {
        pub fn get_joint(&self,) -> Option < Gd < crate::engine::Joint2D > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::Joint2D > >;
            type CallSig = (Option < Gd < crate::engine::Joint2D > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5598usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_joint", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_auto_configure_joint(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5599usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_auto_configure_joint", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_auto_configure_joint(&mut self, auto_configure_joint: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (auto_configure_joint,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5600usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_auto_configure_joint", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_simulate_physics(&mut self, simulate_physics: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (simulate_physics,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5601usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_simulate_physics", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_simulate_physics(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5602usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_simulate_physics", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_simulating_physics(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5603usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_simulating_physics", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_bone2d_nodepath(&mut self, nodepath: NodePath,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), NodePath);
            let args = (nodepath,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5604usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_bone2d_nodepath", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_bone2d_nodepath(&self,) -> NodePath {
            type RetMarshal = PtrcallReturnT < NodePath >;
            type CallSig = (NodePath,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5605usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_bone2d_nodepath", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_bone2d_index(&mut self, bone_index: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (bone_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5606usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_bone2d_index", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_bone2d_index(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5607usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_bone2d_index", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_follow_bone_when_simulating(&mut self, follow_bone: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (follow_bone,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5608usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_follow_bone_when_simulating", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_follow_bone_when_simulating(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5609usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_follow_bone_when_simulating", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for PhysicalBone2D {
        type Base = crate::engine::RigidBody2D;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"PhysicalBone2D\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for PhysicalBone2D {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for PhysicalBone2D {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::RigidBody2D > for PhysicalBone2D {
        
    }
    impl crate::obj::Inherits < crate::engine::PhysicsBody2D > for PhysicalBone2D {
        
    }
    impl crate::obj::Inherits < crate::engine::CollisionObject2D > for PhysicalBone2D {
        
    }
    impl crate::obj::Inherits < crate::engine::Node2D > for PhysicalBone2D {
        
    }
    impl crate::obj::Inherits < crate::engine::CanvasItem > for PhysicalBone2D {
        
    }
    impl crate::obj::Inherits < crate::engine::Node > for PhysicalBone2D {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for PhysicalBone2D {
        
    }
    impl crate::obj::ExportableObject for PhysicalBone2D {
        
    }
    impl crate::obj::cap::GodotDefault for PhysicalBone2D {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for PhysicalBone2D {
        type Target = crate::engine::RigidBody2D;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for PhysicalBone2D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_PhysicalBone2D {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::PhysicalBone2D > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::RigidBody2D > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::PhysicsBody2D > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::CollisionObject2D > for $Class {
                
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