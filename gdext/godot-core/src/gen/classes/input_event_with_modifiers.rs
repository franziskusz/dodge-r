#![doc = "Sidecar module for class [`InputEventWithModifiers`][crate::engine::InputEventWithModifiers].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `InputEventWithModifiers` enums](https://docs.godotengine.org/en/stable/classes/class_inputeventwithmodifiers.html#enumerations).\n\n"]
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
    #[doc = "Godot class `InputEventWithModifiers.`\n\nInherits [`InputEventFromWindow`][crate::engine::InputEventFromWindow].\n\nRelated symbols:\n\n* [`IInputEventWithModifiers`][crate::engine::IInputEventWithModifiers]: virtual methods\n\n\nSee also [Godot docs for `InputEventWithModifiers`](https://docs.godotengine.org/en/stable/classes/class_inputeventwithmodifiers.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct InputEventWithModifiers {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`InputEventWithModifiers`][crate::engine::InputEventWithModifiers].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `InputEventWithModifiers` methods](https://docs.godotengine.org/en/stable/classes/class_inputeventwithmodifiers.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IInputEventWithModifiers: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
        fn setup_local_to_scene(&mut self,) {
            unimplemented !()
        }
    }
    impl InputEventWithModifiers {
        pub fn set_command_or_control_autoremap(&mut self, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4060usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_command_or_control_autoremap", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_command_or_control_autoremap(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4061usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_command_or_control_autoremap", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_command_or_control_pressed(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4062usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_command_or_control_pressed", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_alt_pressed(&mut self, pressed: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (pressed,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4063usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_alt_pressed", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_alt_pressed(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4064usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_alt_pressed", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_shift_pressed(&mut self, pressed: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (pressed,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4065usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_shift_pressed", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_shift_pressed(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4066usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_shift_pressed", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_ctrl_pressed(&mut self, pressed: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (pressed,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4067usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_ctrl_pressed", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_ctrl_pressed(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4068usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_ctrl_pressed", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_meta_pressed(&mut self, pressed: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (pressed,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4069usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_meta_pressed", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_meta_pressed(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4070usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_meta_pressed", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_modifiers_mask(&self,) -> crate::engine::global::KeyModifierMask {
            type RetMarshal = PtrcallReturnT < crate::engine::global::KeyModifierMask >;
            type CallSig = (crate::engine::global::KeyModifierMask,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4071usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_modifiers_mask", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for InputEventWithModifiers {
        type Base = crate::engine::InputEventFromWindow;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"InputEventWithModifiers\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for InputEventWithModifiers {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for InputEventWithModifiers {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::InputEventFromWindow > for InputEventWithModifiers {
        
    }
    impl crate::obj::Inherits < crate::engine::InputEvent > for InputEventWithModifiers {
        
    }
    impl crate::obj::Inherits < crate::engine::Resource > for InputEventWithModifiers {
        
    }
    impl crate::obj::Inherits < crate::engine::RefCounted > for InputEventWithModifiers {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for InputEventWithModifiers {
        
    }
    impl crate::obj::ExportableObject for InputEventWithModifiers {
        
    }
    impl std::ops::Deref for InputEventWithModifiers {
        type Target = crate::engine::InputEventFromWindow;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for InputEventWithModifiers {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_InputEventWithModifiers {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::InputEventWithModifiers > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::InputEventFromWindow > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::InputEvent > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::Resource > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::RefCounted > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::Object > for $Class {
                
            }
            impl::godot::obj::ExportableObject for $Class {
                
            }
        }
    }
}