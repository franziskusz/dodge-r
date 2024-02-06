#![doc = "Sidecar module for class [`AudioEffectCompressor`][crate::engine::AudioEffectCompressor].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `AudioEffectCompressor` enums](https://docs.godotengine.org/en/stable/classes/class_audioeffectcompressor.html#enumerations).\n\n"]
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
    #[doc = "Godot class `AudioEffectCompressor.`\n\nInherits [`AudioEffect`][crate::engine::AudioEffect].\n\nRelated symbols:\n\n* [`IAudioEffectCompressor`][crate::engine::IAudioEffectCompressor]: virtual methods\n\n\nSee also [Godot docs for `AudioEffectCompressor`](https://docs.godotengine.org/en/stable/classes/class_audioeffectcompressor.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct AudioEffectCompressor {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`AudioEffectCompressor`][crate::engine::AudioEffectCompressor].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `AudioEffectCompressor` methods](https://docs.godotengine.org/en/stable/classes/class_audioeffectcompressor.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IAudioEffectCompressor: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
    impl AudioEffectCompressor {
        #[deprecated = "Replaced with `new_gd` in extension trait `NewGd`."]
        pub fn new() -> Gd < Self > {
            crate::obj::Gd::default()
        }
        pub fn set_threshold(&mut self, threshold: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (threshold,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(631usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_threshold", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_threshold(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(632usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_threshold", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_ratio(&mut self, ratio: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (ratio,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(633usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_ratio", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_ratio(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(634usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_ratio", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_gain(&mut self, gain: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (gain,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(635usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_gain", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_gain(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(636usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_gain", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_attack_us(&mut self, attack_us: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (attack_us,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(637usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_attack_us", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_attack_us(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(638usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_attack_us", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_release_ms(&mut self, release_ms: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (release_ms,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(639usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_release_ms", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_release_ms(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(640usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_release_ms", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_mix(&mut self, mix: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (mix,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(641usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_mix", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_mix(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(642usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_mix", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_sidechain(&mut self, sidechain: StringName,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), StringName);
            let args = (sidechain,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(643usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_sidechain", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_sidechain(&self,) -> StringName {
            type RetMarshal = PtrcallReturnT < StringName >;
            type CallSig = (StringName,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(644usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_sidechain", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for AudioEffectCompressor {
        type Base = crate::engine::AudioEffect;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"AudioEffectCompressor\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for AudioEffectCompressor {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for AudioEffectCompressor {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::AudioEffect > for AudioEffectCompressor {
        
    }
    impl crate::obj::Inherits < crate::engine::Resource > for AudioEffectCompressor {
        
    }
    impl crate::obj::Inherits < crate::engine::RefCounted > for AudioEffectCompressor {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for AudioEffectCompressor {
        
    }
    impl crate::obj::ExportableObject for AudioEffectCompressor {
        
    }
    impl crate::obj::cap::GodotDefault for AudioEffectCompressor {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for AudioEffectCompressor {
        type Target = crate::engine::AudioEffect;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for AudioEffectCompressor {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_AudioEffectCompressor {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::AudioEffectCompressor > for $Class {
                
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