#![doc = "Sidecar module for class [`InputEventKey`][crate::engine::InputEventKey].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `InputEventKey` enums](https://docs.godotengine.org/en/stable/classes/class_inputeventkey.html#enumerations).\n\n"]
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
    #[doc = "Godot class `InputEventKey.`\n\nInherits [`InputEventWithModifiers`][crate::engine::InputEventWithModifiers].\n\nRelated symbols:\n\n* [`IInputEventKey`][crate::engine::IInputEventKey]: virtual methods\n\n\nSee also [Godot docs for `InputEventKey`](https://docs.godotengine.org/en/stable/classes/class_inputeventkey.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct InputEventKey {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`InputEventKey`][crate::engine::InputEventKey].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `InputEventKey` methods](https://docs.godotengine.org/en/stable/classes/class_inputeventkey.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IInputEventKey: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
    impl InputEventKey {
        #[deprecated = "Replaced with `new_gd` in extension trait `NewGd`."]
        pub fn new() -> Gd < Self > {
            crate::obj::Gd::default()
        }
        pub fn set_pressed(&mut self, pressed: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (pressed,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3976usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_pressed", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_keycode(&mut self, keycode: crate::engine::global::Key,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::global::Key);
            let args = (keycode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3977usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_keycode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_keycode(&self,) -> crate::engine::global::Key {
            type RetMarshal = PtrcallReturnT < crate::engine::global::Key >;
            type CallSig = (crate::engine::global::Key,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3978usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_keycode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_physical_keycode(&mut self, physical_keycode: crate::engine::global::Key,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::global::Key);
            let args = (physical_keycode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3979usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_physical_keycode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_physical_keycode(&self,) -> crate::engine::global::Key {
            type RetMarshal = PtrcallReturnT < crate::engine::global::Key >;
            type CallSig = (crate::engine::global::Key,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3980usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_physical_keycode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_key_label(&mut self, key_label: crate::engine::global::Key,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::global::Key);
            let args = (key_label,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3981usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_key_label", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_key_label(&self,) -> crate::engine::global::Key {
            type RetMarshal = PtrcallReturnT < crate::engine::global::Key >;
            type CallSig = (crate::engine::global::Key,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3982usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_key_label", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_unicode(&mut self, unicode: i64,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i64);
            let args = (unicode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3983usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_unicode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_unicode(&self,) -> i64 {
            type RetMarshal = PtrcallReturnT < i64 >;
            type CallSig = (i64,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3984usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_unicode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_echo(&mut self, echo: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (echo,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3985usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_echo", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_keycode_with_modifiers(&self,) -> crate::engine::global::Key {
            type RetMarshal = PtrcallReturnT < crate::engine::global::Key >;
            type CallSig = (crate::engine::global::Key,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3986usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_keycode_with_modifiers", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_physical_keycode_with_modifiers(&self,) -> crate::engine::global::Key {
            type RetMarshal = PtrcallReturnT < crate::engine::global::Key >;
            type CallSig = (crate::engine::global::Key,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3987usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_physical_keycode_with_modifiers", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_key_label_with_modifiers(&self,) -> crate::engine::global::Key {
            type RetMarshal = PtrcallReturnT < crate::engine::global::Key >;
            type CallSig = (crate::engine::global::Key,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3988usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_key_label_with_modifiers", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn as_text_keycode(&self,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3989usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "as_text_keycode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn as_text_physical_keycode(&self,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3990usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "as_text_physical_keycode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn as_text_key_label(&self,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3991usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "as_text_key_label", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for InputEventKey {
        type Base = crate::engine::InputEventWithModifiers;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"InputEventKey\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for InputEventKey {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for InputEventKey {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::InputEventWithModifiers > for InputEventKey {
        
    }
    impl crate::obj::Inherits < crate::engine::InputEventFromWindow > for InputEventKey {
        
    }
    impl crate::obj::Inherits < crate::engine::InputEvent > for InputEventKey {
        
    }
    impl crate::obj::Inherits < crate::engine::Resource > for InputEventKey {
        
    }
    impl crate::obj::Inherits < crate::engine::RefCounted > for InputEventKey {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for InputEventKey {
        
    }
    impl crate::obj::ExportableObject for InputEventKey {
        
    }
    impl crate::obj::cap::GodotDefault for InputEventKey {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for InputEventKey {
        type Target = crate::engine::InputEventWithModifiers;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for InputEventKey {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_InputEventKey {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::InputEventKey > for $Class {
                
            }
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