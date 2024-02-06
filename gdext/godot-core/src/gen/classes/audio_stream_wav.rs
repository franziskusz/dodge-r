#![doc = "Sidecar module for class [`AudioStreamWav`][crate::engine::AudioStreamWav].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `AudioStreamWAV` enums](https://docs.godotengine.org/en/stable/classes/class_audiostreamwav.html#enumerations).\n\n"]
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
    #[doc = "Godot class `AudioStreamWAV.`\n\nInherits [`AudioStream`][crate::engine::AudioStream].\n\nRelated symbols:\n\n* [`audio_stream_wav`][crate::engine::audio_stream_wav]: sidecar module with related enum/flag types\n* [`IAudioStreamWav`][crate::engine::IAudioStreamWav]: virtual methods\n\n\nSee also [Godot docs for `AudioStreamWAV`](https://docs.godotengine.org/en/stable/classes/class_audiostreamwav.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct AudioStreamWav {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`AudioStreamWav`][crate::engine::AudioStreamWav].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `AudioStreamWAV` methods](https://docs.godotengine.org/en/stable/classes/class_audiostreamwav.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IAudioStreamWav: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
        fn instantiate_playback(&self,) -> Option < Gd < crate::engine::AudioStreamPlayback > > {
            unimplemented !()
        }
        fn get_stream_name(&self,) -> GString {
            unimplemented !()
        }
        fn get_length(&self,) -> f64 {
            unimplemented !()
        }
        fn is_monophonic(&self,) -> bool {
            unimplemented !()
        }
        fn get_bpm(&self,) -> f64 {
            unimplemented !()
        }
        fn get_beat_count(&self,) -> i32 {
            unimplemented !()
        }
        fn setup_local_to_scene(&mut self,) {
            unimplemented !()
        }
    }
    impl AudioStreamWav {
        #[deprecated = "Replaced with `new_gd` in extension trait `NewGd`."]
        pub fn new() -> Gd < Self > {
            crate::obj::Gd::default()
        }
        pub fn set_data(&mut self, data: PackedByteArray,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), PackedByteArray);
            let args = (data,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(918usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_data", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_data(&self,) -> PackedByteArray {
            type RetMarshal = PtrcallReturnT < PackedByteArray >;
            type CallSig = (PackedByteArray,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(919usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_data", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_format(&mut self, format: crate::engine::audio_stream_wav::Format,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::audio_stream_wav::Format);
            let args = (format,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(920usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_format", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_format(&self,) -> crate::engine::audio_stream_wav::Format {
            type RetMarshal = PtrcallReturnT < crate::engine::audio_stream_wav::Format >;
            type CallSig = (crate::engine::audio_stream_wav::Format,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(921usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_format", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_loop_mode(&mut self, loop_mode: crate::engine::audio_stream_wav::LoopMode,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::audio_stream_wav::LoopMode);
            let args = (loop_mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(922usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_loop_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_loop_mode(&self,) -> crate::engine::audio_stream_wav::LoopMode {
            type RetMarshal = PtrcallReturnT < crate::engine::audio_stream_wav::LoopMode >;
            type CallSig = (crate::engine::audio_stream_wav::LoopMode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(923usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_loop_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_loop_begin(&mut self, loop_begin: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (loop_begin,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(924usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_loop_begin", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_loop_begin(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(925usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_loop_begin", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_loop_end(&mut self, loop_end: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (loop_end,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(926usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_loop_end", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_loop_end(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(927usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_loop_end", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_mix_rate(&mut self, mix_rate: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (mix_rate,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(928usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_mix_rate", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_mix_rate(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(929usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_mix_rate", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_stereo(&mut self, stereo: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (stereo,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(930usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_stereo", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_stereo(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(931usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_stereo", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn save_to_wav(&mut self, path: GString,) -> crate::engine::global::Error {
            type RetMarshal = PtrcallReturnT < crate::engine::global::Error >;
            type CallSig = (crate::engine::global::Error, GString);
            let args = (path,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(932usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "save_to_wav", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for AudioStreamWav {
        type Base = crate::engine::AudioStream;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"AudioStreamWAV\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for AudioStreamWav {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for AudioStreamWav {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::AudioStream > for AudioStreamWav {
        
    }
    impl crate::obj::Inherits < crate::engine::Resource > for AudioStreamWav {
        
    }
    impl crate::obj::Inherits < crate::engine::RefCounted > for AudioStreamWav {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for AudioStreamWav {
        
    }
    impl crate::obj::ExportableObject for AudioStreamWav {
        
    }
    impl crate::obj::cap::GodotDefault for AudioStreamWav {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for AudioStreamWav {
        type Target = crate::engine::AudioStream;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for AudioStreamWav {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_AudioStreamWav {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::AudioStreamWav > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::AudioStream > for $Class {
                
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
pub struct Format {
    ord: i32
}
impl Format {
    pub const FORMAT_8_BITS: Self = Self {
        ord: 0i32
    };
    pub const FORMAT_16_BITS: Self = Self {
        ord: 1i32
    };
    pub const FORMAT_IMA_ADPCM: Self = Self {
        ord: 2i32
    };
    
}
impl crate::obj::EngineEnum for Format {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 => Some(Self {
                ord
            }), _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
impl crate::builtin::meta::GodotConvert for Format {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for Format {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for Format {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct LoopMode {
    ord: i32
}
impl LoopMode {
    pub const LOOP_DISABLED: Self = Self {
        ord: 0i32
    };
    pub const LOOP_FORWARD: Self = Self {
        ord: 1i32
    };
    pub const LOOP_PINGPONG: Self = Self {
        ord: 2i32
    };
    pub const LOOP_BACKWARD: Self = Self {
        ord: 3i32
    };
    
}
impl crate::obj::EngineEnum for LoopMode {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 => Some(Self {
                ord
            }), _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
impl crate::builtin::meta::GodotConvert for LoopMode {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for LoopMode {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for LoopMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}