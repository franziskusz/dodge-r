#![doc = "Sidecar module for class [`AudioStreamPlayer2D`][crate::engine::AudioStreamPlayer2D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `AudioStreamPlayer2D` enums](https://docs.godotengine.org/en/stable/classes/class_audiostreamplayer2d.html#enumerations).\n\n"]
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
    #[doc = "Godot class `AudioStreamPlayer2D.`\n\nInherits [`Node2D`][crate::engine::Node2D].\n\nRelated symbols:\n\n* [`audio_stream_player_2d`][crate::engine::audio_stream_player_2d]: sidecar module with related enum/flag types\n* [`IAudioStreamPlayer2D`][crate::engine::IAudioStreamPlayer2D]: virtual methods\n\n\nSee also [Godot docs for `AudioStreamPlayer2D`](https://docs.godotengine.org/en/stable/classes/class_audiostreamplayer2d.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct AudioStreamPlayer2D {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`AudioStreamPlayer2D`][crate::engine::AudioStreamPlayer2D].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `AudioStreamPlayer2D` methods](https://docs.godotengine.org/en/stable/classes/class_audiostreamplayer2d.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IAudioStreamPlayer2D: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
        fn on_notification(&mut self, what: CanvasItemNotification) {
            unimplemented !()
        }
        fn draw(&mut self,) {
            unimplemented !()
        }
        fn process(&mut self, delta: f64,) {
            unimplemented !()
        }
        fn physics_process(&mut self, delta: f64,) {
            unimplemented !()
        }
        fn enter_tree(&mut self,) {
            unimplemented !()
        }
        fn exit_tree(&mut self,) {
            unimplemented !()
        }
        fn ready(&mut self,) {
            unimplemented !()
        }
        fn get_configuration_warnings(&self,) -> PackedStringArray {
            unimplemented !()
        }
        fn input(&mut self, event: Gd < crate::engine::InputEvent >,) {
            unimplemented !()
        }
        fn shortcut_input(&mut self, event: Gd < crate::engine::InputEvent >,) {
            unimplemented !()
        }
        fn unhandled_input(&mut self, event: Gd < crate::engine::InputEvent >,) {
            unimplemented !()
        }
        fn unhandled_key_input(&mut self, event: Gd < crate::engine::InputEvent >,) {
            unimplemented !()
        }
    }
    impl AudioStreamPlayer2D {
        pub fn set_stream(&mut self, stream: Gd < crate::engine::AudioStream >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::AudioStream >);
            let args = (stream,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(827usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_stream", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_stream(&self,) -> Option < Gd < crate::engine::AudioStream > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::AudioStream > >;
            type CallSig = (Option < Gd < crate::engine::AudioStream > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(828usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_stream", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_volume_db(&mut self, volume_db: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (volume_db,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(829usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_volume_db", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_volume_db(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(830usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_volume_db", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_pitch_scale(&mut self, pitch_scale: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (pitch_scale,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(831usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_pitch_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_pitch_scale(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(832usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_pitch_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn play_full(&mut self, from_position: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (from_position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(833usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "play", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn play(&mut self,) {
            self.play_ex() . done()
        }
        #[inline]
        pub fn play_ex(&mut self,) -> ExPlay < '_ > {
            ExPlay::new(self,)
        }
        pub fn seek(&mut self, to_position: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (to_position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(834usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "seek", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn stop(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(835usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "stop", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_playing(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(836usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_playing", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_playback_position(&mut self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(837usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_playback_position", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_bus(&mut self, bus: StringName,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), StringName);
            let args = (bus,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(838usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_bus", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_bus(&self,) -> StringName {
            type RetMarshal = PtrcallReturnT < StringName >;
            type CallSig = (StringName,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(839usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_bus", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_autoplay(&mut self, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(840usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_autoplay", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_autoplay_enabled(&mut self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(841usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_autoplay_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_max_distance(&mut self, pixels: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (pixels,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(842usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_max_distance", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_max_distance(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(843usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_max_distance", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_attenuation(&mut self, curve: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (curve,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(844usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_attenuation", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_attenuation(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(845usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_attenuation", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_area_mask(&mut self, mask: u32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), u32);
            let args = (mask,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(846usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_area_mask", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_area_mask(&self,) -> u32 {
            type RetMarshal = PtrcallReturnT < u32 >;
            type CallSig = (u32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(847usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_area_mask", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_stream_paused(&mut self, pause: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (pause,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(848usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_stream_paused", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_stream_paused(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(849usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_stream_paused", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_max_polyphony(&mut self, max_polyphony: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (max_polyphony,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(850usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_max_polyphony", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_max_polyphony(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(851usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_max_polyphony", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_panning_strength(&mut self, panning_strength: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (panning_strength,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(852usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_panning_strength", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_panning_strength(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(853usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_panning_strength", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn has_stream_playback(&mut self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(854usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "has_stream_playback", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_stream_playback(&mut self,) -> Option < Gd < crate::engine::AudioStreamPlayback > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::AudioStreamPlayback > >;
            type CallSig = (Option < Gd < crate::engine::AudioStreamPlayback > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(855usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_stream_playback", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for AudioStreamPlayer2D {
        type Base = crate::engine::Node2D;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"AudioStreamPlayer2D\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for AudioStreamPlayer2D {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for AudioStreamPlayer2D {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::Node2D > for AudioStreamPlayer2D {
        
    }
    impl crate::obj::Inherits < crate::engine::CanvasItem > for AudioStreamPlayer2D {
        
    }
    impl crate::obj::Inherits < crate::engine::Node > for AudioStreamPlayer2D {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for AudioStreamPlayer2D {
        
    }
    impl crate::obj::ExportableObject for AudioStreamPlayer2D {
        
    }
    impl crate::obj::cap::GodotDefault for AudioStreamPlayer2D {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for AudioStreamPlayer2D {
        type Target = crate::engine::Node2D;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for AudioStreamPlayer2D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_AudioStreamPlayer2D {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::AudioStreamPlayer2D > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::Node2D > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::CanvasItem > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::Node > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::Object > for $Class {
                
            }
            impl::godot::obj::ExportableObject for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`AudioStreamPlayer2D::play_ex`][super::AudioStreamPlayer2D::play_ex]."]
#[must_use]
pub struct ExPlay < 'a > {
    surround_object: &'a mut re_export::AudioStreamPlayer2D, from_position: f32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExPlay < 'a > {
    fn new(surround_object: &'a mut re_export::AudioStreamPlayer2D,) -> Self {
        Self {
            surround_object, from_position: 0f32,
        }
    }
    #[inline]
    pub fn from_position(self, value: f32) -> Self {
        Self {
            from_position: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::AudioStreamPlayer2D::play_full(self.surround_object, self.from_position,)
    }
}