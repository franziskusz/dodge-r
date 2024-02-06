#![doc = "Sidecar module for class [`AudioStreamPlaybackPolyphonic`][crate::engine::AudioStreamPlaybackPolyphonic].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `AudioStreamPlaybackPolyphonic` enums](https://docs.godotengine.org/en/stable/classes/class_audiostreamplaybackpolyphonic.html#enumerations).\n\n"]
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
    #[doc = "Godot class `AudioStreamPlaybackPolyphonic.`\n\nInherits [`AudioStreamPlayback`][crate::engine::AudioStreamPlayback].\n\nRelated symbols:\n\n* [`audio_stream_playback_polyphonic`][crate::engine::audio_stream_playback_polyphonic]: sidecar module with related enum/flag types\n* [`IAudioStreamPlaybackPolyphonic`][crate::engine::IAudioStreamPlaybackPolyphonic]: virtual methods\n\n\nSee also [Godot docs for `AudioStreamPlaybackPolyphonic`](https://docs.godotengine.org/en/stable/classes/class_audiostreamplaybackpolyphonic.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct AudioStreamPlaybackPolyphonic {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`AudioStreamPlaybackPolyphonic`][crate::engine::AudioStreamPlaybackPolyphonic].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `AudioStreamPlaybackPolyphonic` methods](https://docs.godotengine.org/en/stable/classes/class_audiostreamplaybackpolyphonic.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IAudioStreamPlaybackPolyphonic: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
        fn start(&mut self, from_pos: f64,) {
            unimplemented !()
        }
        fn stop(&mut self,) {
            unimplemented !()
        }
        fn is_playing(&self,) -> bool {
            unimplemented !()
        }
        fn get_loop_count(&self,) -> i32 {
            unimplemented !()
        }
        fn get_playback_position(&self,) -> f64 {
            unimplemented !()
        }
        fn seek(&mut self, position: f64,) {
            unimplemented !()
        }
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" Godot currently does not document safety requirements on this method. Make sure you understand the underlying semantics."]
        unsafe fn mix(&mut self, buffer: * mut AudioFrame, rate_scale: f32, frames: i32,) -> i32 {
            unimplemented !()
        }
        fn tag_used_streams(&mut self,) {
            unimplemented !()
        }
    }
    impl AudioStreamPlaybackPolyphonic {
        pub(crate) fn play_stream_full(&mut self, stream: Gd < crate::engine::AudioStream >, from_offset: f32, volume_db: f32, pitch_scale: f32,) -> i64 {
            type RetMarshal = PtrcallReturnT < i64 >;
            type CallSig = (i64, Gd < crate::engine::AudioStream >, f32, f32, f32);
            let args = (stream, from_offset, volume_db, pitch_scale,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(798usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "play_stream", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn play_stream(&mut self, stream: Gd < crate::engine::AudioStream >,) -> i64 {
            self.play_stream_ex(stream,) . done()
        }
        #[inline]
        pub fn play_stream_ex(&mut self, stream: Gd < crate::engine::AudioStream >,) -> ExPlayStream < '_ > {
            ExPlayStream::new(self, stream,)
        }
        pub fn set_stream_volume(&mut self, stream: i64, volume_db: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i64, f32);
            let args = (stream, volume_db,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(799usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_stream_volume", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_stream_pitch_scale(&mut self, stream: i64, pitch_scale: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i64, f32);
            let args = (stream, pitch_scale,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(800usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_stream_pitch_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_stream_playing(&self, stream: i64,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, i64);
            let args = (stream,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(801usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_stream_playing", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn stop_stream(&mut self, stream: i64,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i64);
            let args = (stream,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(802usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "stop_stream", self.object_ptr, self.__checked_id(), args,)
            }
        }
        fn __checked_id(&self) -> Option < crate::obj::InstanceId > {
            let rtti = unsafe {
                self.rtti.as_ref() . unwrap_unchecked()
            };
            let instance_id = rtti.check_type::< Self > ();
            Some(instance_id)
        }
        pub const INVALID_ID: i32 = - 1i32;
        
    }
    impl crate::obj::GodotClass for AudioStreamPlaybackPolyphonic {
        type Base = crate::engine::AudioStreamPlayback;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"AudioStreamPlaybackPolyphonic\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for AudioStreamPlaybackPolyphonic {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for AudioStreamPlaybackPolyphonic {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::AudioStreamPlayback > for AudioStreamPlaybackPolyphonic {
        
    }
    impl crate::obj::Inherits < crate::engine::RefCounted > for AudioStreamPlaybackPolyphonic {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for AudioStreamPlaybackPolyphonic {
        
    }
    impl std::ops::Deref for AudioStreamPlaybackPolyphonic {
        type Target = crate::engine::AudioStreamPlayback;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for AudioStreamPlaybackPolyphonic {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_AudioStreamPlaybackPolyphonic {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::AudioStreamPlaybackPolyphonic > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::AudioStreamPlayback > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::RefCounted > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`AudioStreamPlaybackPolyphonic::play_stream_ex`][super::AudioStreamPlaybackPolyphonic::play_stream_ex]."]
#[must_use]
pub struct ExPlayStream < 'a > {
    surround_object: &'a mut re_export::AudioStreamPlaybackPolyphonic, stream: Gd < crate::engine::AudioStream >, from_offset: f32, volume_db: f32, pitch_scale: f32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExPlayStream < 'a > {
    fn new(surround_object: &'a mut re_export::AudioStreamPlaybackPolyphonic, stream: Gd < crate::engine::AudioStream >,) -> Self {
        Self {
            surround_object, stream, from_offset: 0f32, volume_db: 0f32, pitch_scale: 1f32,
        }
    }
    #[inline]
    pub fn from_offset(self, value: f32) -> Self {
        Self {
            from_offset: value, .. self
        }
    }
    #[inline]
    pub fn volume_db(self, value: f32) -> Self {
        Self {
            volume_db: value, .. self
        }
    }
    #[inline]
    pub fn pitch_scale(self, value: f32) -> Self {
        Self {
            pitch_scale: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i64 {
        re_export::AudioStreamPlaybackPolyphonic::play_stream_full(self.surround_object, self.stream, self.from_offset, self.volume_db, self.pitch_scale,)
    }
}