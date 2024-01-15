#![doc = "Sidecar module for class [`AudioEffectChorus`][crate::engine::AudioEffectChorus].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `AudioEffectChorus` enums](https://docs.godotengine.org/en/stable/classes/class_audioeffectchorus.html#enumerations).\n\n"]
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
    #[doc = "Godot class `AudioEffectChorus.`\n\nInherits [`AudioEffect`][crate::engine::AudioEffect].\n\nRelated symbols:\n\n* [`IAudioEffectChorus`][crate::engine::IAudioEffectChorus]: virtual methods\n\n\nSee also [Godot docs for `AudioEffectChorus`](https://docs.godotengine.org/en/stable/classes/class_audioeffectchorus.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct AudioEffectChorus {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`AudioEffectChorus`][crate::engine::AudioEffectChorus].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `AudioEffectChorus` methods](https://docs.godotengine.org/en/stable/classes/class_audioeffectchorus.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IAudioEffectChorus: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
    impl AudioEffectChorus {
        #[deprecated = "Replaced with `new_gd` in extension trait `NewGd`."]
        pub fn new() -> Gd < Self > {
            crate::obj::Gd::default()
        }
        pub fn set_voice_count(&mut self, voices: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (voices,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(613usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_voice_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_voice_count(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(614usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_voice_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_voice_delay_ms(&mut self, voice_idx: i32, delay_ms: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, f32);
            let args = (voice_idx, delay_ms,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(615usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_voice_delay_ms", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_voice_delay_ms(&self, voice_idx: i32,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32, i32);
            let args = (voice_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(616usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_voice_delay_ms", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_voice_rate_hz(&mut self, voice_idx: i32, rate_hz: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, f32);
            let args = (voice_idx, rate_hz,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(617usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_voice_rate_hz", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_voice_rate_hz(&self, voice_idx: i32,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32, i32);
            let args = (voice_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(618usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_voice_rate_hz", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_voice_depth_ms(&mut self, voice_idx: i32, depth_ms: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, f32);
            let args = (voice_idx, depth_ms,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(619usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_voice_depth_ms", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_voice_depth_ms(&self, voice_idx: i32,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32, i32);
            let args = (voice_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(620usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_voice_depth_ms", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_voice_level_db(&mut self, voice_idx: i32, level_db: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, f32);
            let args = (voice_idx, level_db,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(621usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_voice_level_db", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_voice_level_db(&self, voice_idx: i32,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32, i32);
            let args = (voice_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(622usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_voice_level_db", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_voice_cutoff_hz(&mut self, voice_idx: i32, cutoff_hz: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, f32);
            let args = (voice_idx, cutoff_hz,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(623usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_voice_cutoff_hz", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_voice_cutoff_hz(&self, voice_idx: i32,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32, i32);
            let args = (voice_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(624usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_voice_cutoff_hz", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_voice_pan(&mut self, voice_idx: i32, pan: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, f32);
            let args = (voice_idx, pan,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(625usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_voice_pan", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_voice_pan(&self, voice_idx: i32,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32, i32);
            let args = (voice_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(626usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_voice_pan", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_wet(&mut self, amount: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (amount,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(627usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_wet", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_wet(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(628usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_wet", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_dry(&mut self, amount: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (amount,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(629usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_dry", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_dry(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(630usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_dry", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for AudioEffectChorus {
        type Base = crate::engine::AudioEffect;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"AudioEffectChorus\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for AudioEffectChorus {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for AudioEffectChorus {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::AudioEffect > for AudioEffectChorus {
        
    }
    impl crate::obj::Inherits < crate::engine::Resource > for AudioEffectChorus {
        
    }
    impl crate::obj::Inherits < crate::engine::RefCounted > for AudioEffectChorus {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for AudioEffectChorus {
        
    }
    impl crate::obj::ExportableObject for AudioEffectChorus {
        
    }
    impl crate::obj::cap::GodotDefault for AudioEffectChorus {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for AudioEffectChorus {
        type Target = crate::engine::AudioEffect;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for AudioEffectChorus {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_AudioEffectChorus {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::AudioEffectChorus > for $Class {
                
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