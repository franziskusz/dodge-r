#![doc = "Sidecar module for class [`AudioStreamPlayer3D`][crate::engine::AudioStreamPlayer3D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `AudioStreamPlayer3D` enums](https://docs.godotengine.org/en/stable/classes/class_audiostreamplayer3d.html#enumerations).\n\n"]
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
    #[doc = "Godot class `AudioStreamPlayer3D.`\n\nInherits [`Node3D`][crate::engine::Node3D].\n\nRelated symbols:\n\n* [`audio_stream_player_3d`][crate::engine::audio_stream_player_3d]: sidecar module with related enum/flag types\n* [`IAudioStreamPlayer3D`][crate::engine::IAudioStreamPlayer3D]: virtual methods\n\n\nSee also [Godot docs for `AudioStreamPlayer3D`](https://docs.godotengine.org/en/stable/classes/class_audiostreamplayer3d.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct AudioStreamPlayer3D {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`AudioStreamPlayer3D`][crate::engine::AudioStreamPlayer3D].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `AudioStreamPlayer3D` methods](https://docs.godotengine.org/en/stable/classes/class_audiostreamplayer3d.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IAudioStreamPlayer3D: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
        fn on_notification(&mut self, what: Node3DNotification) {
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
    impl AudioStreamPlayer3D {
        pub fn set_stream(&mut self, stream: Gd < crate::engine::AudioStream >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::AudioStream >);
            let args = (stream,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(856usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_stream", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_stream(&self,) -> Option < Gd < crate::engine::AudioStream > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::AudioStream > >;
            type CallSig = (Option < Gd < crate::engine::AudioStream > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(857usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_stream", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_volume_db(&mut self, volume_db: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (volume_db,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(858usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_volume_db", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_volume_db(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(859usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_volume_db", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_unit_size(&mut self, unit_size: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (unit_size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(860usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_unit_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_unit_size(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(861usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_unit_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_max_db(&mut self, max_db: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (max_db,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(862usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_max_db", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_max_db(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(863usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_max_db", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_pitch_scale(&mut self, pitch_scale: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (pitch_scale,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(864usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_pitch_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_pitch_scale(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(865usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_pitch_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn play_full(&mut self, from_position: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (from_position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(866usize);
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
                let method_bind = sys::class_scene_api() . fptr_by_index(867usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "seek", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn stop(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(868usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "stop", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_playing(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(869usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_playing", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_playback_position(&mut self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(870usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_playback_position", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_bus(&mut self, bus: StringName,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), StringName);
            let args = (bus,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(871usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_bus", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_bus(&self,) -> StringName {
            type RetMarshal = PtrcallReturnT < StringName >;
            type CallSig = (StringName,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(872usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_bus", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_autoplay(&mut self, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(873usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_autoplay", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_autoplay_enabled(&mut self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(874usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_autoplay_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_max_distance(&mut self, meters: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (meters,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(875usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_max_distance", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_max_distance(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(876usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_max_distance", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_area_mask(&mut self, mask: u32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), u32);
            let args = (mask,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(877usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_area_mask", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_area_mask(&self,) -> u32 {
            type RetMarshal = PtrcallReturnT < u32 >;
            type CallSig = (u32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(878usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_area_mask", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_emission_angle(&mut self, degrees: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (degrees,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(879usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_emission_angle", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_emission_angle(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(880usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_emission_angle", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_emission_angle_enabled(&mut self, enabled: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(881usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_emission_angle_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_emission_angle_enabled(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(882usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_emission_angle_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_emission_angle_filter_attenuation_db(&mut self, db: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (db,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(883usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_emission_angle_filter_attenuation_db", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_emission_angle_filter_attenuation_db(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(884usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_emission_angle_filter_attenuation_db", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_attenuation_filter_cutoff_hz(&mut self, degrees: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (degrees,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(885usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_attenuation_filter_cutoff_hz", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_attenuation_filter_cutoff_hz(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(886usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_attenuation_filter_cutoff_hz", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_attenuation_filter_db(&mut self, db: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (db,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(887usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_attenuation_filter_db", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_attenuation_filter_db(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(888usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_attenuation_filter_db", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_attenuation_model(&mut self, model: crate::engine::audio_stream_player_3d::AttenuationModel,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::audio_stream_player_3d::AttenuationModel);
            let args = (model,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(889usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_attenuation_model", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_attenuation_model(&self,) -> crate::engine::audio_stream_player_3d::AttenuationModel {
            type RetMarshal = PtrcallReturnT < crate::engine::audio_stream_player_3d::AttenuationModel >;
            type CallSig = (crate::engine::audio_stream_player_3d::AttenuationModel,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(890usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_attenuation_model", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_doppler_tracking(&mut self, mode: crate::engine::audio_stream_player_3d::DopplerTracking,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::audio_stream_player_3d::DopplerTracking);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(891usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_doppler_tracking", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_doppler_tracking(&self,) -> crate::engine::audio_stream_player_3d::DopplerTracking {
            type RetMarshal = PtrcallReturnT < crate::engine::audio_stream_player_3d::DopplerTracking >;
            type CallSig = (crate::engine::audio_stream_player_3d::DopplerTracking,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(892usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_doppler_tracking", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_stream_paused(&mut self, pause: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (pause,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(893usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_stream_paused", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_stream_paused(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(894usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_stream_paused", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_max_polyphony(&mut self, max_polyphony: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (max_polyphony,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(895usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_max_polyphony", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_max_polyphony(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(896usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_max_polyphony", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_panning_strength(&mut self, panning_strength: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (panning_strength,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(897usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_panning_strength", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_panning_strength(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(898usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_panning_strength", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn has_stream_playback(&mut self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(899usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "has_stream_playback", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_stream_playback(&mut self,) -> Option < Gd < crate::engine::AudioStreamPlayback > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::AudioStreamPlayback > >;
            type CallSig = (Option < Gd < crate::engine::AudioStreamPlayback > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(900usize);
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
    impl crate::obj::GodotClass for AudioStreamPlayer3D {
        type Base = crate::engine::Node3D;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"AudioStreamPlayer3D\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for AudioStreamPlayer3D {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for AudioStreamPlayer3D {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::Node3D > for AudioStreamPlayer3D {
        
    }
    impl crate::obj::Inherits < crate::engine::Node > for AudioStreamPlayer3D {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for AudioStreamPlayer3D {
        
    }
    impl crate::obj::ExportableObject for AudioStreamPlayer3D {
        
    }
    impl crate::obj::cap::GodotDefault for AudioStreamPlayer3D {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for AudioStreamPlayer3D {
        type Target = crate::engine::Node3D;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for AudioStreamPlayer3D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_AudioStreamPlayer3D {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::AudioStreamPlayer3D > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::Node3D > for $Class {
                
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
#[doc = "Default-param extender for [`AudioStreamPlayer3D::play_ex`][super::AudioStreamPlayer3D::play_ex]."]
#[must_use]
pub struct ExPlay < 'a > {
    surround_object: &'a mut re_export::AudioStreamPlayer3D, from_position: f32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExPlay < 'a > {
    fn new(surround_object: &'a mut re_export::AudioStreamPlayer3D,) -> Self {
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
        re_export::AudioStreamPlayer3D::play_full(self.surround_object, self.from_position,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct AttenuationModel {
    ord: i32
}
impl AttenuationModel {
    pub const ATTENUATION_INVERSE_DISTANCE: Self = Self {
        ord: 0i32
    };
    pub const ATTENUATION_INVERSE_SQUARE_DISTANCE: Self = Self {
        ord: 1i32
    };
    pub const ATTENUATION_LOGARITHMIC: Self = Self {
        ord: 2i32
    };
    pub const ATTENUATION_DISABLED: Self = Self {
        ord: 3i32
    };
    
}
impl crate::obj::EngineEnum for AttenuationModel {
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
impl crate::builtin::meta::GodotConvert for AttenuationModel {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for AttenuationModel {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for AttenuationModel {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct DopplerTracking {
    ord: i32
}
impl DopplerTracking {
    pub const DOPPLER_TRACKING_DISABLED: Self = Self {
        ord: 0i32
    };
    pub const DOPPLER_TRACKING_IDLE_STEP: Self = Self {
        ord: 1i32
    };
    pub const DOPPLER_TRACKING_PHYSICS_STEP: Self = Self {
        ord: 2i32
    };
    
}
impl crate::obj::EngineEnum for DopplerTracking {
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
impl crate::builtin::meta::GodotConvert for DopplerTracking {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for DopplerTracking {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for DopplerTracking {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}