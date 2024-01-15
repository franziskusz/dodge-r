#![doc = "Sidecar module for class [`AudioEffectPhaser`][crate::engine::AudioEffectPhaser].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `AudioEffectPhaser` enums](https://docs.godotengine.org/en/stable/classes/class_audioeffectphaser.html#enumerations).\n\n"]
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
    #[doc = "Godot class `AudioEffectPhaser.`\n\nInherits [`AudioEffect`][crate::engine::AudioEffect].\n\nRelated symbols:\n\n* [`IAudioEffectPhaser`][crate::engine::IAudioEffectPhaser]: virtual methods\n\n\nSee also [Godot docs for `AudioEffectPhaser`](https://docs.godotengine.org/en/stable/classes/class_audioeffectphaser.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct AudioEffectPhaser {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`AudioEffectPhaser`][crate::engine::AudioEffectPhaser].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `AudioEffectPhaser` methods](https://docs.godotengine.org/en/stable/classes/class_audioeffectphaser.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IAudioEffectPhaser: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
        fn instantiate(&mut self,) -> Option < Gd < crate::engine::AudioEffectInstance > > {
            unimplemented !()
        }
        fn setup_local_to_scene(&mut self,) {
            unimplemented !()
        }
    }
    impl AudioEffectPhaser {
        #[deprecated = "Replaced with `new_gd` in extension trait `NewGd`."]
        pub fn new() -> Gd < Self > {
            crate::obj::Gd::default()
        }
        pub fn set_range_min_hz(&mut self, hz: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (hz,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(702usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_range_min_hz", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_range_min_hz(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(703usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_range_min_hz", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_range_max_hz(&mut self, hz: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (hz,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(704usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_range_max_hz", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_range_max_hz(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(705usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_range_max_hz", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_rate_hz(&mut self, hz: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (hz,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(706usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_rate_hz", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_rate_hz(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(707usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_rate_hz", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_feedback(&mut self, fbk: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (fbk,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(708usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_feedback", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_feedback(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(709usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_feedback", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_depth(&mut self, depth: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (depth,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(710usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_depth", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_depth(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(711usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_depth", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for AudioEffectPhaser {
        type Base = crate::engine::AudioEffect;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"AudioEffectPhaser\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for AudioEffectPhaser {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for AudioEffectPhaser {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::AudioEffect > for AudioEffectPhaser {
        
    }
    impl crate::obj::Inherits < crate::engine::Resource > for AudioEffectPhaser {
        
    }
    impl crate::obj::Inherits < crate::engine::RefCounted > for AudioEffectPhaser {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for AudioEffectPhaser {
        
    }
    impl crate::obj::ExportableObject for AudioEffectPhaser {
        
    }
    impl crate::obj::cap::GodotDefault for AudioEffectPhaser {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for AudioEffectPhaser {
        type Target = crate::engine::AudioEffect;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for AudioEffectPhaser {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_AudioEffectPhaser {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::AudioEffectPhaser > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::AudioEffect > for $Class {
                
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