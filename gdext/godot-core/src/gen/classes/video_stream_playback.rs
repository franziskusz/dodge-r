#![doc = "Sidecar module for class [`VideoStreamPlayback`][crate::engine::VideoStreamPlayback].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `VideoStreamPlayback` enums](https://docs.godotengine.org/en/stable/classes/class_videostreamplayback.html#enumerations).\n\n"]
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
    #[doc = "Godot class `VideoStreamPlayback.`\n\nInherits [`Resource`][crate::engine::Resource].\n\nRelated symbols:\n\n* [`video_stream_playback`][crate::engine::video_stream_playback]: sidecar module with related enum/flag types\n* [`IVideoStreamPlayback`][crate::engine::IVideoStreamPlayback]: virtual methods\n\n\nSee also [Godot docs for `VideoStreamPlayback`](https://docs.godotengine.org/en/stable/classes/class_videostreamplayback.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct VideoStreamPlayback {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`VideoStreamPlayback`][crate::engine::VideoStreamPlayback].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `VideoStreamPlayback` methods](https://docs.godotengine.org/en/stable/classes/class_videostreamplayback.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IVideoStreamPlayback: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
        fn stop(&mut self,) {
            unimplemented !()
        }
        fn play(&mut self,) {
            unimplemented !()
        }
        fn is_playing(&self,) -> bool {
            unimplemented !()
        }
        fn set_paused(&mut self, paused: bool,) {
            unimplemented !()
        }
        fn is_paused(&self,) -> bool {
            unimplemented !()
        }
        fn get_length(&self,) -> f64 {
            unimplemented !()
        }
        fn get_playback_position(&self,) -> f64 {
            unimplemented !()
        }
        fn seek(&mut self, time: f64,) {
            unimplemented !()
        }
        fn set_audio_track(&mut self, idx: i32,) {
            unimplemented !()
        }
        fn get_texture(&self,) -> Option < Gd < crate::engine::Texture2D > > {
            unimplemented !()
        }
        fn update(&mut self, delta: f64,) {
            unimplemented !()
        }
        fn get_channels(&self,) -> i32 {
            unimplemented !()
        }
        fn get_mix_rate(&self,) -> i32 {
            unimplemented !()
        }
        fn setup_local_to_scene(&mut self,) {
            unimplemented !()
        }
    }
    impl VideoStreamPlayback {
        #[deprecated = "Replaced with `new_gd` in extension trait `NewGd`."]
        pub fn new() -> Gd < Self > {
            crate::obj::Gd::default()
        }
        pub(crate) fn mix_audio_full(&mut self, num_frames: i32, buffer: PackedFloat32Array, offset: i32,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, i32, PackedFloat32Array, i32);
            let args = (num_frames, buffer, offset,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9260usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "mix_audio", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn mix_audio(&mut self, num_frames: i32,) -> i32 {
            self.mix_audio_ex(num_frames,) . done()
        }
        #[inline]
        pub fn mix_audio_ex(&mut self, num_frames: i32,) -> ExMixAudio < '_ > {
            ExMixAudio::new(self, num_frames,)
        }
        fn __checked_id(&self) -> Option < crate::obj::InstanceId > {
            let rtti = unsafe {
                self.rtti.as_ref() . unwrap_unchecked()
            };
            let instance_id = rtti.check_type::< Self > ();
            Some(instance_id)
        }
    }
    impl crate::obj::GodotClass for VideoStreamPlayback {
        type Base = crate::engine::Resource;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"VideoStreamPlayback\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for VideoStreamPlayback {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for VideoStreamPlayback {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::Resource > for VideoStreamPlayback {
        
    }
    impl crate::obj::Inherits < crate::engine::RefCounted > for VideoStreamPlayback {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for VideoStreamPlayback {
        
    }
    impl crate::obj::ExportableObject for VideoStreamPlayback {
        
    }
    impl crate::obj::cap::GodotDefault for VideoStreamPlayback {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for VideoStreamPlayback {
        type Target = crate::engine::Resource;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for VideoStreamPlayback {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_VideoStreamPlayback {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::VideoStreamPlayback > for $Class {
                
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
#[doc = "Default-param extender for [`VideoStreamPlayback::mix_audio_ex`][super::VideoStreamPlayback::mix_audio_ex]."]
#[must_use]
pub struct ExMixAudio < 'a > {
    surround_object: &'a mut re_export::VideoStreamPlayback, num_frames: i32, buffer: PackedFloat32Array, offset: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExMixAudio < 'a > {
    fn new(surround_object: &'a mut re_export::VideoStreamPlayback, num_frames: i32,) -> Self {
        Self {
            surround_object, num_frames, buffer: PackedFloat32Array::new(), offset: 0i32,
        }
    }
    #[inline]
    pub fn buffer(self, value: PackedFloat32Array) -> Self {
        Self {
            buffer: value, .. self
        }
    }
    #[inline]
    pub fn offset(self, value: i32) -> Self {
        Self {
            offset: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        re_export::VideoStreamPlayback::mix_audio_full(self.surround_object, self.num_frames, self.buffer, self.offset,)
    }
}