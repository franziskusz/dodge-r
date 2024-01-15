#![doc = "Sidecar module for class [`AudioServer`][crate::engine::AudioServer].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `AudioServer` enums](https://docs.godotengine.org/en/stable/classes/class_audioserver.html#enumerations).\n\n"]
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
    #[doc = "Godot class `AudioServer.`\n\nInherits [`Object`][crate::engine::Object].\n\nRelated symbols:\n\n* [`audio_server`][crate::engine::audio_server]: sidecar module with related enum/flag types\n* [`IAudioServer`][crate::engine::IAudioServer]: virtual methods\n\n\nSee also [Godot docs for `AudioServer`](https://docs.godotengine.org/en/stable/classes/class_audioserver.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct AudioServer {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`AudioServer`][crate::engine::AudioServer].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `AudioServer` methods](https://docs.godotengine.org/en/stable/classes/class_audioserver.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IAudioServer: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
    }
    impl AudioServer {
        pub fn singleton() -> Gd < Self > {
            unsafe {
                let __class_name = StringName::from_latin1_with_nul(b"AudioServer\0");
                let __object_ptr = sys::interface_fn !(global_get_singleton) (__class_name.string_sys());
                Gd::from_obj_sys(__object_ptr)
            }
        }
        pub fn set_bus_count(&mut self, amount: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (amount,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(0usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_bus_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_bus_count(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_bus_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn remove_bus(&mut self, index: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(2usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "remove_bus", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn add_bus_full(&mut self, at_position: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (at_position,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(3usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_bus", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn add_bus(&mut self,) {
            self.add_bus_ex() . done()
        }
        #[inline]
        pub fn add_bus_ex(&mut self,) -> ExAddBus < '_ > {
            ExAddBus::new(self,)
        }
        pub fn move_bus(&mut self, index: i32, to_index: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, i32);
            let args = (index, to_index,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(4usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "move_bus", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_bus_name(&mut self, bus_idx: i32, name: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, GString);
            let args = (bus_idx, name,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(5usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_bus_name", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_bus_name(&self, bus_idx: i32,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString, i32);
            let args = (bus_idx,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(6usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_bus_name", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_bus_index(&self, bus_name: StringName,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, StringName);
            let args = (bus_name,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(7usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_bus_index", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_bus_channels(&self, bus_idx: i32,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, i32);
            let args = (bus_idx,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(8usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_bus_channels", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_bus_volume_db(&mut self, bus_idx: i32, volume_db: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, f32);
            let args = (bus_idx, volume_db,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(9usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_bus_volume_db", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_bus_volume_db(&self, bus_idx: i32,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32, i32);
            let args = (bus_idx,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(10usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_bus_volume_db", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_bus_send(&mut self, bus_idx: i32, send: StringName,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, StringName);
            let args = (bus_idx, send,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(11usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_bus_send", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_bus_send(&self, bus_idx: i32,) -> StringName {
            type RetMarshal = PtrcallReturnT < StringName >;
            type CallSig = (StringName, i32);
            let args = (bus_idx,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(12usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_bus_send", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_bus_solo(&mut self, bus_idx: i32, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, bool);
            let args = (bus_idx, enable,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(13usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_bus_solo", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_bus_solo(&self, bus_idx: i32,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, i32);
            let args = (bus_idx,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(14usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_bus_solo", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_bus_mute(&mut self, bus_idx: i32, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, bool);
            let args = (bus_idx, enable,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(15usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_bus_mute", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_bus_mute(&self, bus_idx: i32,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, i32);
            let args = (bus_idx,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(16usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_bus_mute", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_bus_bypass_effects(&mut self, bus_idx: i32, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, bool);
            let args = (bus_idx, enable,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(17usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_bus_bypass_effects", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_bus_bypassing_effects(&self, bus_idx: i32,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, i32);
            let args = (bus_idx,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(18usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_bus_bypassing_effects", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn add_bus_effect_full(&mut self, bus_idx: i32, effect: Gd < crate::engine::AudioEffect >, at_position: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, Gd < crate::engine::AudioEffect >, i32);
            let args = (bus_idx, effect, at_position,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(19usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_bus_effect", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn add_bus_effect(&mut self, bus_idx: i32, effect: Gd < crate::engine::AudioEffect >,) {
            self.add_bus_effect_ex(bus_idx, effect,) . done()
        }
        #[inline]
        pub fn add_bus_effect_ex(&mut self, bus_idx: i32, effect: Gd < crate::engine::AudioEffect >,) -> ExAddBusEffect < '_ > {
            ExAddBusEffect::new(self, bus_idx, effect,)
        }
        pub fn remove_bus_effect(&mut self, bus_idx: i32, effect_idx: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, i32);
            let args = (bus_idx, effect_idx,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(20usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "remove_bus_effect", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_bus_effect_count(&mut self, bus_idx: i32,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, i32);
            let args = (bus_idx,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(21usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_bus_effect_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_bus_effect(&mut self, bus_idx: i32, effect_idx: i32,) -> Option < Gd < crate::engine::AudioEffect > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::AudioEffect > >;
            type CallSig = (Option < Gd < crate::engine::AudioEffect > >, i32, i32);
            let args = (bus_idx, effect_idx,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(22usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_bus_effect", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn get_bus_effect_instance_full(&mut self, bus_idx: i32, effect_idx: i32, channel: i32,) -> Option < Gd < crate::engine::AudioEffectInstance > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::AudioEffectInstance > >;
            type CallSig = (Option < Gd < crate::engine::AudioEffectInstance > >, i32, i32, i32);
            let args = (bus_idx, effect_idx, channel,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(23usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_bus_effect_instance", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn get_bus_effect_instance(&mut self, bus_idx: i32, effect_idx: i32,) -> Option < Gd < crate::engine::AudioEffectInstance > > {
            self.get_bus_effect_instance_ex(bus_idx, effect_idx,) . done()
        }
        #[inline]
        pub fn get_bus_effect_instance_ex(&mut self, bus_idx: i32, effect_idx: i32,) -> ExGetBusEffectInstance < '_ > {
            ExGetBusEffectInstance::new(self, bus_idx, effect_idx,)
        }
        pub fn swap_bus_effects(&mut self, bus_idx: i32, effect_idx: i32, by_effect_idx: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, i32, i32);
            let args = (bus_idx, effect_idx, by_effect_idx,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(24usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "swap_bus_effects", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_bus_effect_enabled(&mut self, bus_idx: i32, effect_idx: i32, enabled: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, i32, bool);
            let args = (bus_idx, effect_idx, enabled,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(25usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_bus_effect_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_bus_effect_enabled(&self, bus_idx: i32, effect_idx: i32,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, i32, i32);
            let args = (bus_idx, effect_idx,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(26usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_bus_effect_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_bus_peak_volume_left_db(&self, bus_idx: i32, channel: i32,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32, i32, i32);
            let args = (bus_idx, channel,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(27usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_bus_peak_volume_left_db", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_bus_peak_volume_right_db(&self, bus_idx: i32, channel: i32,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32, i32, i32);
            let args = (bus_idx, channel,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(28usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_bus_peak_volume_right_db", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_playback_speed_scale(&mut self, scale: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (scale,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(29usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_playback_speed_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_playback_speed_scale(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(30usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_playback_speed_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn lock(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(31usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "lock", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn unlock(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(32usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "unlock", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_speaker_mode(&self,) -> crate::engine::audio_server::SpeakerMode {
            type RetMarshal = PtrcallReturnT < crate::engine::audio_server::SpeakerMode >;
            type CallSig = (crate::engine::audio_server::SpeakerMode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(33usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_speaker_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_mix_rate(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(34usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_mix_rate", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_output_device_list(&mut self,) -> PackedStringArray {
            type RetMarshal = PtrcallReturnT < PackedStringArray >;
            type CallSig = (PackedStringArray,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(35usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_output_device_list", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_output_device(&mut self,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(36usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_output_device", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_output_device(&mut self, name: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString);
            let args = (name,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(37usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_output_device", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_time_to_next_mix(&self,) -> f64 {
            type RetMarshal = PtrcallReturnT < f64 >;
            type CallSig = (f64,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(38usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_time_to_next_mix", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_time_since_last_mix(&self,) -> f64 {
            type RetMarshal = PtrcallReturnT < f64 >;
            type CallSig = (f64,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(39usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_time_since_last_mix", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_output_latency(&self,) -> f64 {
            type RetMarshal = PtrcallReturnT < f64 >;
            type CallSig = (f64,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(40usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_output_latency", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_input_device_list(&mut self,) -> PackedStringArray {
            type RetMarshal = PtrcallReturnT < PackedStringArray >;
            type CallSig = (PackedStringArray,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(41usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_input_device_list", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_input_device(&mut self,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(42usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_input_device", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_input_device(&mut self, name: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString);
            let args = (name,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(43usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_input_device", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_bus_layout(&mut self, bus_layout: Gd < crate::engine::AudioBusLayout >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::AudioBusLayout >);
            let args = (bus_layout,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(44usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_bus_layout", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn generate_bus_layout(&self,) -> Option < Gd < crate::engine::AudioBusLayout > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::AudioBusLayout > >;
            type CallSig = (Option < Gd < crate::engine::AudioBusLayout > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(45usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "generate_bus_layout", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_enable_tagging_used_audio_streams(&mut self, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(46usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_enable_tagging_used_audio_streams", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for AudioServer {
        type Base = crate::engine::Object;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"AudioServer\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Servers;
        
    }
    unsafe impl crate::obj::Bounds for AudioServer {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for AudioServer {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::Object > for AudioServer {
        
    }
    impl std::ops::Deref for AudioServer {
        type Target = crate::engine::Object;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for AudioServer {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_AudioServer {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::AudioServer > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`AudioServer::add_bus_ex`][super::AudioServer::add_bus_ex]."]
#[must_use]
pub struct ExAddBus < 'a > {
    surround_object: &'a mut re_export::AudioServer, at_position: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddBus < 'a > {
    fn new(surround_object: &'a mut re_export::AudioServer,) -> Self {
        Self {
            surround_object, at_position: - 1i32,
        }
    }
    #[inline]
    pub fn at_position(self, value: i32) -> Self {
        Self {
            at_position: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::AudioServer::add_bus_full(self.surround_object, self.at_position,)
    }
}
#[doc = "Default-param extender for [`AudioServer::add_bus_effect_ex`][super::AudioServer::add_bus_effect_ex]."]
#[must_use]
pub struct ExAddBusEffect < 'a > {
    surround_object: &'a mut re_export::AudioServer, bus_idx: i32, effect: Gd < crate::engine::AudioEffect >, at_position: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddBusEffect < 'a > {
    fn new(surround_object: &'a mut re_export::AudioServer, bus_idx: i32, effect: Gd < crate::engine::AudioEffect >,) -> Self {
        Self {
            surround_object, bus_idx, effect, at_position: - 1i32,
        }
    }
    #[inline]
    pub fn at_position(self, value: i32) -> Self {
        Self {
            at_position: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::AudioServer::add_bus_effect_full(self.surround_object, self.bus_idx, self.effect, self.at_position,)
    }
}
#[doc = "Default-param extender for [`AudioServer::get_bus_effect_instance_ex`][super::AudioServer::get_bus_effect_instance_ex]."]
#[must_use]
pub struct ExGetBusEffectInstance < 'a > {
    surround_object: &'a mut re_export::AudioServer, bus_idx: i32, effect_idx: i32, channel: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetBusEffectInstance < 'a > {
    fn new(surround_object: &'a mut re_export::AudioServer, bus_idx: i32, effect_idx: i32,) -> Self {
        Self {
            surround_object, bus_idx, effect_idx, channel: 0i32,
        }
    }
    #[inline]
    pub fn channel(self, value: i32) -> Self {
        Self {
            channel: value, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::engine::AudioEffectInstance > > {
        re_export::AudioServer::get_bus_effect_instance_full(self.surround_object, self.bus_idx, self.effect_idx, self.channel,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct SpeakerMode {
    ord: i32
}
impl SpeakerMode {
    pub const SPEAKER_MODE_STEREO: Self = Self {
        ord: 0i32
    };
    pub const SPEAKER_SURROUND_31: Self = Self {
        ord: 1i32
    };
    pub const SPEAKER_SURROUND_51: Self = Self {
        ord: 2i32
    };
    pub const SPEAKER_SURROUND_71: Self = Self {
        ord: 3i32
    };
    
}
impl crate::obj::EngineEnum for SpeakerMode {
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
impl crate::builtin::meta::GodotConvert for SpeakerMode {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for SpeakerMode {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for SpeakerMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}