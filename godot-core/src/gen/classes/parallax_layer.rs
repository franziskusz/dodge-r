#![doc = "Sidecar module for class [`ParallaxLayer`][crate::engine::ParallaxLayer].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `ParallaxLayer` enums](https://docs.godotengine.org/en/stable/classes/class_parallaxlayer.html#enumerations).\n\n"]
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
    #[doc = "Godot class `ParallaxLayer.`\n\nInherits [`Node2D`][crate::engine::Node2D].\n\nRelated symbols:\n\n* [`IParallaxLayer`][crate::engine::IParallaxLayer]: virtual methods\n\n\nSee also [Godot docs for `ParallaxLayer`](https://docs.godotengine.org/en/stable/classes/class_parallaxlayer.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct ParallaxLayer {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`ParallaxLayer`][crate::engine::ParallaxLayer].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `ParallaxLayer` methods](https://docs.godotengine.org/en/stable/classes/class_parallaxlayer.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IParallaxLayer: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
    impl ParallaxLayer {
        pub fn set_motion_scale(&mut self, scale: Vector2,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Vector2);
            let args = (scale,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5458usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_motion_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_motion_scale(&self,) -> Vector2 {
            type RetMarshal = PtrcallReturnT < Vector2 >;
            type CallSig = (Vector2,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5459usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_motion_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_motion_offset(&mut self, offset: Vector2,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Vector2);
            let args = (offset,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5460usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_motion_offset", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_motion_offset(&self,) -> Vector2 {
            type RetMarshal = PtrcallReturnT < Vector2 >;
            type CallSig = (Vector2,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5461usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_motion_offset", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_mirroring(&mut self, mirror: Vector2,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Vector2);
            let args = (mirror,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5462usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_mirroring", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_mirroring(&self,) -> Vector2 {
            type RetMarshal = PtrcallReturnT < Vector2 >;
            type CallSig = (Vector2,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5463usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_mirroring", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for ParallaxLayer {
        type Base = crate::engine::Node2D;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"ParallaxLayer\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for ParallaxLayer {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for ParallaxLayer {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::Node2D > for ParallaxLayer {
        
    }
    impl crate::obj::Inherits < crate::engine::CanvasItem > for ParallaxLayer {
        
    }
    impl crate::obj::Inherits < crate::engine::Node > for ParallaxLayer {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for ParallaxLayer {
        
    }
    impl crate::obj::ExportableObject for ParallaxLayer {
        
    }
    impl crate::obj::cap::GodotDefault for ParallaxLayer {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for ParallaxLayer {
        type Target = crate::engine::Node2D;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for ParallaxLayer {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_ParallaxLayer {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::ParallaxLayer > for $Class {
                
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