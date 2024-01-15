#![doc = "Sidecar module for class [`AudioEffectSpectrumAnalyzer`][crate::engine::AudioEffectSpectrumAnalyzer].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `AudioEffectSpectrumAnalyzer` enums](https://docs.godotengine.org/en/stable/classes/class_audioeffectspectrumanalyzer.html#enumerations).\n\n"]
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
    #[doc = "Godot class `AudioEffectSpectrumAnalyzer.`\n\nInherits [`AudioEffect`][crate::engine::AudioEffect].\n\nRelated symbols:\n\n* [`audio_effect_spectrum_analyzer`][crate::engine::audio_effect_spectrum_analyzer]: sidecar module with related enum/flag types\n* [`IAudioEffectSpectrumAnalyzer`][crate::engine::IAudioEffectSpectrumAnalyzer]: virtual methods\n\n\nSee also [Godot docs for `AudioEffectSpectrumAnalyzer`](https://docs.godotengine.org/en/stable/classes/class_audioeffectspectrumanalyzer.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct AudioEffectSpectrumAnalyzer {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`AudioEffectSpectrumAnalyzer`][crate::engine::AudioEffectSpectrumAnalyzer].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `AudioEffectSpectrumAnalyzer` methods](https://docs.godotengine.org/en/stable/classes/class_audioeffectspectrumanalyzer.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IAudioEffectSpectrumAnalyzer: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
    impl AudioEffectSpectrumAnalyzer {
        #[deprecated = "Replaced with `new_gd` in extension trait `NewGd`."]
        pub fn new() -> Gd < Self > {
            crate::obj::Gd::default()
        }
        pub fn set_buffer_length(&mut self, seconds: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (seconds,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(739usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_buffer_length", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_buffer_length(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(740usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_buffer_length", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_tap_back_pos(&mut self, seconds: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (seconds,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(741usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_tap_back_pos", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_tap_back_pos(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(742usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_tap_back_pos", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_fft_size(&mut self, size: crate::engine::audio_effect_spectrum_analyzer::FFTSize,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::audio_effect_spectrum_analyzer::FFTSize);
            let args = (size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(743usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_fft_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_fft_size(&self,) -> crate::engine::audio_effect_spectrum_analyzer::FFTSize {
            type RetMarshal = PtrcallReturnT < crate::engine::audio_effect_spectrum_analyzer::FFTSize >;
            type CallSig = (crate::engine::audio_effect_spectrum_analyzer::FFTSize,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(744usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_fft_size", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for AudioEffectSpectrumAnalyzer {
        type Base = crate::engine::AudioEffect;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"AudioEffectSpectrumAnalyzer\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for AudioEffectSpectrumAnalyzer {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for AudioEffectSpectrumAnalyzer {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::AudioEffect > for AudioEffectSpectrumAnalyzer {
        
    }
    impl crate::obj::Inherits < crate::engine::Resource > for AudioEffectSpectrumAnalyzer {
        
    }
    impl crate::obj::Inherits < crate::engine::RefCounted > for AudioEffectSpectrumAnalyzer {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for AudioEffectSpectrumAnalyzer {
        
    }
    impl crate::obj::ExportableObject for AudioEffectSpectrumAnalyzer {
        
    }
    impl crate::obj::cap::GodotDefault for AudioEffectSpectrumAnalyzer {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for AudioEffectSpectrumAnalyzer {
        type Target = crate::engine::AudioEffect;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for AudioEffectSpectrumAnalyzer {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_AudioEffectSpectrumAnalyzer {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::AudioEffectSpectrumAnalyzer > for $Class {
                
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
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct FFTSize {
    ord: i32
}
impl FFTSize {
    pub const FFT_SIZE_256: Self = Self {
        ord: 0i32
    };
    pub const FFT_SIZE_512: Self = Self {
        ord: 1i32
    };
    pub const FFT_SIZE_1024: Self = Self {
        ord: 2i32
    };
    pub const FFT_SIZE_2048: Self = Self {
        ord: 3i32
    };
    pub const FFT_SIZE_4096: Self = Self {
        ord: 4i32
    };
    pub const FFT_SIZE_MAX: Self = Self {
        ord: 5i32
    };
    
}
impl crate::obj::EngineEnum for FFTSize {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 => Some(Self {
                ord
            }), _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
impl crate::obj::IndexEnum for FFTSize {
    const ENUMERATOR_COUNT: usize = 5usize;
    
}
impl crate::builtin::meta::GodotConvert for FFTSize {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for FFTSize {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for FFTSize {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}