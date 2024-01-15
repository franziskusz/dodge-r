#![doc = "Sidecar module for class [`AudioStreamRandomizer`][crate::engine::AudioStreamRandomizer].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `AudioStreamRandomizer` enums](https://docs.godotengine.org/en/stable/classes/class_audiostreamrandomizer.html#enumerations).\n\n"]
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
    #[doc = "Godot class `AudioStreamRandomizer.`\n\nInherits [`AudioStream`][crate::engine::AudioStream].\n\nRelated symbols:\n\n* [`audio_stream_randomizer`][crate::engine::audio_stream_randomizer]: sidecar module with related enum/flag types\n* [`IAudioStreamRandomizer`][crate::engine::IAudioStreamRandomizer]: virtual methods\n\n\nSee also [Godot docs for `AudioStreamRandomizer`](https://docs.godotengine.org/en/stable/classes/class_audiostreamrandomizer.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct AudioStreamRandomizer {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`AudioStreamRandomizer`][crate::engine::AudioStreamRandomizer].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `AudioStreamRandomizer` methods](https://docs.godotengine.org/en/stable/classes/class_audiostreamrandomizer.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IAudioStreamRandomizer: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
    impl AudioStreamRandomizer {
        #[deprecated = "Replaced with `new_gd` in extension trait `NewGd`."]
        pub fn new() -> Gd < Self > {
            crate::obj::Gd::default()
        }
        pub(crate) fn add_stream_full(&mut self, index: i32, stream: Gd < crate::engine::AudioStream >, weight: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, Gd < crate::engine::AudioStream >, f32);
            let args = (index, stream, weight,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(903usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_stream", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn add_stream(&mut self, index: i32, stream: Gd < crate::engine::AudioStream >,) {
            self.add_stream_ex(index, stream,) . done()
        }
        #[inline]
        pub fn add_stream_ex(&mut self, index: i32, stream: Gd < crate::engine::AudioStream >,) -> ExAddStream < '_ > {
            ExAddStream::new(self, index, stream,)
        }
        pub fn move_stream(&mut self, index_from: i32, index_to: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, i32);
            let args = (index_from, index_to,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(904usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "move_stream", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn remove_stream(&mut self, index: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(905usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "remove_stream", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_stream(&mut self, index: i32, stream: Gd < crate::engine::AudioStream >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, Gd < crate::engine::AudioStream >);
            let args = (index, stream,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(906usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_stream", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_stream(&self, index: i32,) -> Option < Gd < crate::engine::AudioStream > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::AudioStream > >;
            type CallSig = (Option < Gd < crate::engine::AudioStream > >, i32);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(907usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_stream", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_stream_probability_weight(&mut self, index: i32, weight: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, f32);
            let args = (index, weight,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(908usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_stream_probability_weight", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_stream_probability_weight(&self, index: i32,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32, i32);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(909usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_stream_probability_weight", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_streams_count(&mut self, count: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (count,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(910usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_streams_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_streams_count(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(911usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_streams_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_random_pitch(&mut self, scale: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (scale,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(912usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_random_pitch", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_random_pitch(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(913usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_random_pitch", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_random_volume_offset_db(&mut self, db_offset: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (db_offset,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(914usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_random_volume_offset_db", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_random_volume_offset_db(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(915usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_random_volume_offset_db", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_playback_mode(&mut self, mode: crate::engine::audio_stream_randomizer::PlaybackMode,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::audio_stream_randomizer::PlaybackMode);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(916usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_playback_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_playback_mode(&self,) -> crate::engine::audio_stream_randomizer::PlaybackMode {
            type RetMarshal = PtrcallReturnT < crate::engine::audio_stream_randomizer::PlaybackMode >;
            type CallSig = (crate::engine::audio_stream_randomizer::PlaybackMode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(917usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_playback_mode", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for AudioStreamRandomizer {
        type Base = crate::engine::AudioStream;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"AudioStreamRandomizer\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for AudioStreamRandomizer {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for AudioStreamRandomizer {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::AudioStream > for AudioStreamRandomizer {
        
    }
    impl crate::obj::Inherits < crate::engine::Resource > for AudioStreamRandomizer {
        
    }
    impl crate::obj::Inherits < crate::engine::RefCounted > for AudioStreamRandomizer {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for AudioStreamRandomizer {
        
    }
    impl crate::obj::ExportableObject for AudioStreamRandomizer {
        
    }
    impl crate::obj::cap::GodotDefault for AudioStreamRandomizer {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for AudioStreamRandomizer {
        type Target = crate::engine::AudioStream;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for AudioStreamRandomizer {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_AudioStreamRandomizer {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::AudioStreamRandomizer > for $Class {
                
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
#[doc = "Default-param extender for [`AudioStreamRandomizer::add_stream_ex`][super::AudioStreamRandomizer::add_stream_ex]."]
#[must_use]
pub struct ExAddStream < 'a > {
    surround_object: &'a mut re_export::AudioStreamRandomizer, index: i32, stream: Gd < crate::engine::AudioStream >, weight: f32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddStream < 'a > {
    fn new(surround_object: &'a mut re_export::AudioStreamRandomizer, index: i32, stream: Gd < crate::engine::AudioStream >,) -> Self {
        Self {
            surround_object, index, stream, weight: 1f32,
        }
    }
    #[inline]
    pub fn weight(self, value: f32) -> Self {
        Self {
            weight: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::AudioStreamRandomizer::add_stream_full(self.surround_object, self.index, self.stream, self.weight,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct PlaybackMode {
    ord: i32
}
impl PlaybackMode {
    pub const PLAYBACK_RANDOM_NO_REPEATS: Self = Self {
        ord: 0i32
    };
    pub const PLAYBACK_RANDOM: Self = Self {
        ord: 1i32
    };
    pub const PLAYBACK_SEQUENTIAL: Self = Self {
        ord: 2i32
    };
    
}
impl crate::obj::EngineEnum for PlaybackMode {
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
impl crate::builtin::meta::GodotConvert for PlaybackMode {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for PlaybackMode {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for PlaybackMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}