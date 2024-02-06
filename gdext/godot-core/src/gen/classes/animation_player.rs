#![doc = "Sidecar module for class [`AnimationPlayer`][crate::engine::AnimationPlayer].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `AnimationPlayer` enums](https://docs.godotengine.org/en/stable/classes/class_animationplayer.html#enumerations).\n\n"]
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
    #[doc = "Godot class `AnimationPlayer.`\n\nInherits [`AnimationMixer`][crate::engine::AnimationMixer].\n\nRelated symbols:\n\n* [`animation_player`][crate::engine::animation_player]: sidecar module with related enum/flag types\n* [`IAnimationPlayer`][crate::engine::IAnimationPlayer]: virtual methods\n\n\nSee also [Godot docs for `AnimationPlayer`](https://docs.godotengine.org/en/stable/classes/class_animationplayer.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct AnimationPlayer {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`AnimationPlayer`][crate::engine::AnimationPlayer].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `AnimationPlayer` methods](https://docs.godotengine.org/en/stable/classes/class_animationplayer.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IAnimationPlayer: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
        fn on_notification(&mut self, what: NodeNotification) {
            unimplemented !()
        }
        fn post_process_key_value(&self, animation: Gd < crate::engine::Animation >, track: i32, value: Variant, object: Gd < crate::engine::Object >, object_idx: i32,) -> Variant {
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
    impl AnimationPlayer {
        pub fn animation_set_next(&mut self, animation_from: StringName, animation_to: StringName,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), StringName, StringName);
            let args = (animation_from, animation_to,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(430usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "animation_set_next", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn animation_get_next(&self, animation_from: StringName,) -> StringName {
            type RetMarshal = PtrcallReturnT < StringName >;
            type CallSig = (StringName, StringName);
            let args = (animation_from,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(431usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "animation_get_next", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_blend_time(&mut self, animation_from: StringName, animation_to: StringName, sec: f64,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), StringName, StringName, f64);
            let args = (animation_from, animation_to, sec,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(432usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_blend_time", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_blend_time(&self, animation_from: StringName, animation_to: StringName,) -> f64 {
            type RetMarshal = PtrcallReturnT < f64 >;
            type CallSig = (f64, StringName, StringName);
            let args = (animation_from, animation_to,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(433usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_blend_time", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_default_blend_time(&mut self, sec: f64,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f64);
            let args = (sec,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(434usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_default_blend_time", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_default_blend_time(&self,) -> f64 {
            type RetMarshal = PtrcallReturnT < f64 >;
            type CallSig = (f64,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(435usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_default_blend_time", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn play_full(&mut self, name: StringName, custom_blend: f64, custom_speed: f32, from_end: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), StringName, f64, f32, bool);
            let args = (name, custom_blend, custom_speed, from_end,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(436usize);
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
        pub(crate) fn play_backwards_full(&mut self, name: StringName, custom_blend: f64,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), StringName, f64);
            let args = (name, custom_blend,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(437usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "play_backwards", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn play_backwards(&mut self,) {
            self.play_backwards_ex() . done()
        }
        #[inline]
        pub fn play_backwards_ex(&mut self,) -> ExPlayBackwards < '_ > {
            ExPlayBackwards::new(self,)
        }
        pub fn pause(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(438usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "pause", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn stop_full(&mut self, keep_state: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (keep_state,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(439usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "stop", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn stop(&mut self,) {
            self.stop_ex() . done()
        }
        #[inline]
        pub fn stop_ex(&mut self,) -> ExStop < '_ > {
            ExStop::new(self,)
        }
        pub fn is_playing(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(440usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_playing", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_current_animation(&mut self, animation: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString);
            let args = (animation,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(441usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_current_animation", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_current_animation(&self,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(442usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_current_animation", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_assigned_animation(&mut self, animation: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString);
            let args = (animation,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(443usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_assigned_animation", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_assigned_animation(&self,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(444usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_assigned_animation", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn queue(&mut self, name: StringName,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), StringName);
            let args = (name,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(445usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "queue", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_queue(&mut self,) -> PackedStringArray {
            type RetMarshal = PtrcallReturnT < PackedStringArray >;
            type CallSig = (PackedStringArray,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(446usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_queue", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn clear_queue(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(447usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "clear_queue", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_speed_scale(&mut self, speed: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (speed,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(448usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_speed_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_speed_scale(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(449usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_speed_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_playing_speed(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(450usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_playing_speed", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_autoplay(&mut self, name: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString);
            let args = (name,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(451usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_autoplay", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_autoplay(&self,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(452usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_autoplay", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_movie_quit_on_finish_enabled(&mut self, enabled: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(453usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_movie_quit_on_finish_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_movie_quit_on_finish_enabled(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(454usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_movie_quit_on_finish_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_current_animation_position(&self,) -> f64 {
            type RetMarshal = PtrcallReturnT < f64 >;
            type CallSig = (f64,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(455usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_current_animation_position", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_current_animation_length(&self,) -> f64 {
            type RetMarshal = PtrcallReturnT < f64 >;
            type CallSig = (f64,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(456usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_current_animation_length", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn seek_full(&mut self, seconds: f64, update: bool, update_only: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f64, bool, bool);
            let args = (seconds, update, update_only,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(457usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "seek", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn seek(&mut self, seconds: f64,) {
            self.seek_ex(seconds,) . done()
        }
        #[inline]
        pub fn seek_ex(&mut self, seconds: f64,) -> ExSeek < '_ > {
            ExSeek::new(self, seconds,)
        }
        pub fn set_process_callback(&mut self, mode: crate::engine::animation_player::AnimationProcessCallback,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::animation_player::AnimationProcessCallback);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(458usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_process_callback", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_process_callback(&self,) -> crate::engine::animation_player::AnimationProcessCallback {
            type RetMarshal = PtrcallReturnT < crate::engine::animation_player::AnimationProcessCallback >;
            type CallSig = (crate::engine::animation_player::AnimationProcessCallback,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(459usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_process_callback", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_method_call_mode(&mut self, mode: crate::engine::animation_player::AnimationMethodCallMode,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::animation_player::AnimationMethodCallMode);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(460usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_method_call_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_method_call_mode(&self,) -> crate::engine::animation_player::AnimationMethodCallMode {
            type RetMarshal = PtrcallReturnT < crate::engine::animation_player::AnimationMethodCallMode >;
            type CallSig = (crate::engine::animation_player::AnimationMethodCallMode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(461usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_method_call_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_root(&mut self, path: NodePath,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), NodePath);
            let args = (path,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(462usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_root", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_root(&self,) -> NodePath {
            type RetMarshal = PtrcallReturnT < NodePath >;
            type CallSig = (NodePath,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(463usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_root", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for AnimationPlayer {
        type Base = crate::engine::AnimationMixer;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"AnimationPlayer\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for AnimationPlayer {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for AnimationPlayer {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::AnimationMixer > for AnimationPlayer {
        
    }
    impl crate::obj::Inherits < crate::engine::Node > for AnimationPlayer {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for AnimationPlayer {
        
    }
    impl crate::obj::ExportableObject for AnimationPlayer {
        
    }
    impl crate::obj::cap::GodotDefault for AnimationPlayer {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for AnimationPlayer {
        type Target = crate::engine::AnimationMixer;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for AnimationPlayer {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_AnimationPlayer {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::AnimationPlayer > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::AnimationMixer > for $Class {
                
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
#[doc = "Default-param extender for [`AnimationPlayer::play_ex`][super::AnimationPlayer::play_ex]."]
#[must_use]
pub struct ExPlay < 'a > {
    surround_object: &'a mut re_export::AnimationPlayer, name: StringName, custom_blend: f64, custom_speed: f32, from_end: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExPlay < 'a > {
    fn new(surround_object: &'a mut re_export::AnimationPlayer,) -> Self {
        Self {
            surround_object, name: StringName::from(""), custom_blend: - 1f64, custom_speed: 1f32, from_end: false,
        }
    }
    #[inline]
    pub fn name(self, value: StringName) -> Self {
        Self {
            name: value, .. self
        }
    }
    #[inline]
    pub fn custom_blend(self, value: f64) -> Self {
        Self {
            custom_blend: value, .. self
        }
    }
    #[inline]
    pub fn custom_speed(self, value: f32) -> Self {
        Self {
            custom_speed: value, .. self
        }
    }
    #[inline]
    pub fn from_end(self, value: bool) -> Self {
        Self {
            from_end: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::AnimationPlayer::play_full(self.surround_object, self.name, self.custom_blend, self.custom_speed, self.from_end,)
    }
}
#[doc = "Default-param extender for [`AnimationPlayer::play_backwards_ex`][super::AnimationPlayer::play_backwards_ex]."]
#[must_use]
pub struct ExPlayBackwards < 'a > {
    surround_object: &'a mut re_export::AnimationPlayer, name: StringName, custom_blend: f64,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExPlayBackwards < 'a > {
    fn new(surround_object: &'a mut re_export::AnimationPlayer,) -> Self {
        Self {
            surround_object, name: StringName::from(""), custom_blend: - 1f64,
        }
    }
    #[inline]
    pub fn name(self, value: StringName) -> Self {
        Self {
            name: value, .. self
        }
    }
    #[inline]
    pub fn custom_blend(self, value: f64) -> Self {
        Self {
            custom_blend: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::AnimationPlayer::play_backwards_full(self.surround_object, self.name, self.custom_blend,)
    }
}
#[doc = "Default-param extender for [`AnimationPlayer::stop_ex`][super::AnimationPlayer::stop_ex]."]
#[must_use]
pub struct ExStop < 'a > {
    surround_object: &'a mut re_export::AnimationPlayer, keep_state: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExStop < 'a > {
    fn new(surround_object: &'a mut re_export::AnimationPlayer,) -> Self {
        Self {
            surround_object, keep_state: false,
        }
    }
    #[inline]
    pub fn keep_state(self, value: bool) -> Self {
        Self {
            keep_state: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::AnimationPlayer::stop_full(self.surround_object, self.keep_state,)
    }
}
#[doc = "Default-param extender for [`AnimationPlayer::seek_ex`][super::AnimationPlayer::seek_ex]."]
#[must_use]
pub struct ExSeek < 'a > {
    surround_object: &'a mut re_export::AnimationPlayer, seconds: f64, update: bool, update_only: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExSeek < 'a > {
    fn new(surround_object: &'a mut re_export::AnimationPlayer, seconds: f64,) -> Self {
        Self {
            surround_object, seconds, update: false, update_only: false,
        }
    }
    #[inline]
    pub fn update(self, value: bool) -> Self {
        Self {
            update: value, .. self
        }
    }
    #[inline]
    pub fn update_only(self, value: bool) -> Self {
        Self {
            update_only: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::AnimationPlayer::seek_full(self.surround_object, self.seconds, self.update, self.update_only,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct AnimationProcessCallback {
    ord: i32
}
impl AnimationProcessCallback {
    pub const ANIMATION_PROCESS_PHYSICS: Self = Self {
        ord: 0i32
    };
    pub const ANIMATION_PROCESS_IDLE: Self = Self {
        ord: 1i32
    };
    pub const ANIMATION_PROCESS_MANUAL: Self = Self {
        ord: 2i32
    };
    
}
impl crate::obj::EngineEnum for AnimationProcessCallback {
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
impl crate::builtin::meta::GodotConvert for AnimationProcessCallback {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for AnimationProcessCallback {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for AnimationProcessCallback {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct AnimationMethodCallMode {
    ord: i32
}
impl AnimationMethodCallMode {
    pub const ANIMATION_METHOD_CALL_DEFERRED: Self = Self {
        ord: 0i32
    };
    pub const ANIMATION_METHOD_CALL_IMMEDIATE: Self = Self {
        ord: 1i32
    };
    
}
impl crate::obj::EngineEnum for AnimationMethodCallMode {
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
impl crate::builtin::meta::GodotConvert for AnimationMethodCallMode {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for AnimationMethodCallMode {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for AnimationMethodCallMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}