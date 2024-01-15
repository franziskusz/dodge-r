#![doc = "Sidecar module for class [`AudioEffectSpectrumAnalyzerInstance`][crate::engine::AudioEffectSpectrumAnalyzerInstance].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `AudioEffectSpectrumAnalyzerInstance` enums](https://docs.godotengine.org/en/stable/classes/class_audioeffectspectrumanalyzerinstance.html#enumerations).\n\n"]
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
    #[doc = "Godot class `AudioEffectSpectrumAnalyzerInstance.`\n\nInherits [`AudioEffectInstance`][crate::engine::AudioEffectInstance].\n\nRelated symbols:\n\n* [`audio_effect_spectrum_analyzer_instance`][crate::engine::audio_effect_spectrum_analyzer_instance]: sidecar module with related enum/flag types\n* [`IAudioEffectSpectrumAnalyzerInstance`][crate::engine::IAudioEffectSpectrumAnalyzerInstance]: virtual methods\n\n\nSee also [Godot docs for `AudioEffectSpectrumAnalyzerInstance`](https://docs.godotengine.org/en/stable/classes/class_audioeffectspectrumanalyzerinstance.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct AudioEffectSpectrumAnalyzerInstance {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`AudioEffectSpectrumAnalyzerInstance`][crate::engine::AudioEffectSpectrumAnalyzerInstance].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `AudioEffectSpectrumAnalyzerInstance` methods](https://docs.godotengine.org/en/stable/classes/class_audioeffectspectrumanalyzerinstance.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IAudioEffectSpectrumAnalyzerInstance: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" Godot currently does not document safety requirements on this method. Make sure you understand the underlying semantics."]
        unsafe fn process(&mut self, src_buffer: * const c_void, dst_buffer: * mut AudioFrame, frame_count: i32,) {
            unimplemented !()
        }
        fn process_silence(&self,) -> bool {
            unimplemented !()
        }
    }
    impl AudioEffectSpectrumAnalyzerInstance {
        pub(crate) fn get_magnitude_for_frequency_range_full(&self, from_hz: f32, to_hz: f32, mode: crate::engine::audio_effect_spectrum_analyzer_instance::MagnitudeMode,) -> Vector2 {
            type RetMarshal = PtrcallReturnT < Vector2 >;
            type CallSig = (Vector2, f32, f32, crate::engine::audio_effect_spectrum_analyzer_instance::MagnitudeMode);
            let args = (from_hz, to_hz, mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(745usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_magnitude_for_frequency_range", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn get_magnitude_for_frequency_range(&self, from_hz: f32, to_hz: f32,) -> Vector2 {
            self.get_magnitude_for_frequency_range_ex(from_hz, to_hz,) . done()
        }
        #[inline]
        pub fn get_magnitude_for_frequency_range_ex(&self, from_hz: f32, to_hz: f32,) -> ExGetMagnitudeForFrequencyRange < '_ > {
            ExGetMagnitudeForFrequencyRange::new(self, from_hz, to_hz,)
        }
        fn __checked_id(&self) -> Option < crate::obj::InstanceId > {
            let rtti = unsafe {
                self.rtti.as_ref() . unwrap_unchecked()
            };
            let instance_id = rtti.check_type::< Self > ();
            Some(instance_id)
        }
    }
    impl crate::obj::GodotClass for AudioEffectSpectrumAnalyzerInstance {
        type Base = crate::engine::AudioEffectInstance;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"AudioEffectSpectrumAnalyzerInstance\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for AudioEffectSpectrumAnalyzerInstance {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for AudioEffectSpectrumAnalyzerInstance {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::AudioEffectInstance > for AudioEffectSpectrumAnalyzerInstance {
        
    }
    impl crate::obj::Inherits < crate::engine::RefCounted > for AudioEffectSpectrumAnalyzerInstance {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for AudioEffectSpectrumAnalyzerInstance {
        
    }
    impl std::ops::Deref for AudioEffectSpectrumAnalyzerInstance {
        type Target = crate::engine::AudioEffectInstance;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for AudioEffectSpectrumAnalyzerInstance {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_AudioEffectSpectrumAnalyzerInstance {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::AudioEffectSpectrumAnalyzerInstance > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::AudioEffectInstance > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::RefCounted > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`AudioEffectSpectrumAnalyzerInstance::get_magnitude_for_frequency_range_ex`][super::AudioEffectSpectrumAnalyzerInstance::get_magnitude_for_frequency_range_ex]."]
#[must_use]
pub struct ExGetMagnitudeForFrequencyRange < 'a > {
    surround_object: &'a re_export::AudioEffectSpectrumAnalyzerInstance, from_hz: f32, to_hz: f32, mode: crate::engine::audio_effect_spectrum_analyzer_instance::MagnitudeMode,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetMagnitudeForFrequencyRange < 'a > {
    fn new(surround_object: &'a re_export::AudioEffectSpectrumAnalyzerInstance, from_hz: f32, to_hz: f32,) -> Self {
        Self {
            surround_object, from_hz, to_hz, mode: crate::obj::EngineEnum::from_ord(1),
        }
    }
    #[inline]
    pub fn mode(self, value: crate::engine::audio_effect_spectrum_analyzer_instance::MagnitudeMode) -> Self {
        Self {
            mode: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Vector2 {
        re_export::AudioEffectSpectrumAnalyzerInstance::get_magnitude_for_frequency_range_full(self.surround_object, self.from_hz, self.to_hz, self.mode,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct MagnitudeMode {
    ord: i32
}
impl MagnitudeMode {
    pub const MAGNITUDE_AVERAGE: Self = Self {
        ord: 0i32
    };
    pub const MAGNITUDE_MAX: Self = Self {
        ord: 1i32
    };
    
}
impl crate::obj::EngineEnum for MagnitudeMode {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 => Some(Self {
                ord
            }), _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
impl crate::obj::IndexEnum for MagnitudeMode {
    const ENUMERATOR_COUNT: usize = 1usize;
    
}
impl crate::builtin::meta::GodotConvert for MagnitudeMode {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for MagnitudeMode {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for MagnitudeMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}