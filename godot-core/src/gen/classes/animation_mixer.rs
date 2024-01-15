#![doc = "Sidecar module for class [`AnimationMixer`][crate::engine::AnimationMixer].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `AnimationMixer` enums](https://docs.godotengine.org/en/stable/classes/class_animationmixer.html#enumerations).\n\n"]
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
    #[doc = "Godot class `AnimationMixer.`\n\nInherits [`Node`][crate::engine::Node].\n\nRelated symbols:\n\n* [`animation_mixer`][crate::engine::animation_mixer]: sidecar module with related enum/flag types\n* [`IAnimationMixer`][crate::engine::IAnimationMixer]: virtual methods\n\n\nSee also [Godot docs for `AnimationMixer`](https://docs.godotengine.org/en/stable/classes/class_animationmixer.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct AnimationMixer {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`AnimationMixer`][crate::engine::AnimationMixer].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `AnimationMixer` methods](https://docs.godotengine.org/en/stable/classes/class_animationmixer.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IAnimationMixer: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
    impl AnimationMixer {
        pub fn add_animation_library(&mut self, name: StringName, library: Gd < crate::engine::AnimationLibrary >,) -> crate::engine::global::Error {
            type RetMarshal = PtrcallReturnT < crate::engine::global::Error >;
            type CallSig = (crate::engine::global::Error, StringName, Gd < crate::engine::AnimationLibrary >);
            let args = (name, library,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(239usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_animation_library", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn remove_animation_library(&mut self, name: StringName,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), StringName);
            let args = (name,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(240usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "remove_animation_library", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn rename_animation_library(&mut self, name: StringName, newname: StringName,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), StringName, StringName);
            let args = (name, newname,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(241usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "rename_animation_library", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn has_animation_library(&self, name: StringName,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, StringName);
            let args = (name,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(242usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "has_animation_library", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_animation_library(&self, name: StringName,) -> Option < Gd < crate::engine::AnimationLibrary > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::AnimationLibrary > >;
            type CallSig = (Option < Gd < crate::engine::AnimationLibrary > >, StringName);
            let args = (name,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(243usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_animation_library", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_animation_library_list(&self,) -> Array < StringName > {
            type RetMarshal = PtrcallReturnT < Array < StringName > >;
            type CallSig = (Array < StringName >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(244usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_animation_library_list", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn has_animation(&self, name: StringName,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, StringName);
            let args = (name,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(245usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "has_animation", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_animation(&self, name: StringName,) -> Option < Gd < crate::engine::Animation > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::Animation > >;
            type CallSig = (Option < Gd < crate::engine::Animation > >, StringName);
            let args = (name,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(246usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_animation", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_animation_list(&self,) -> PackedStringArray {
            type RetMarshal = PtrcallReturnT < PackedStringArray >;
            type CallSig = (PackedStringArray,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(247usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_animation_list", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_active(&mut self, active: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (active,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(248usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_active", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_active(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(249usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_active", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_deterministic(&mut self, deterministic: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (deterministic,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(250usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_deterministic", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_deterministic(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(251usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_deterministic", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_root_node(&mut self, path: NodePath,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), NodePath);
            let args = (path,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(252usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_root_node", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_root_node(&self,) -> NodePath {
            type RetMarshal = PtrcallReturnT < NodePath >;
            type CallSig = (NodePath,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(253usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_root_node", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_callback_mode_process(&mut self, mode: crate::engine::animation_mixer::AnimationCallbackModeProcess,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::animation_mixer::AnimationCallbackModeProcess);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(254usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_callback_mode_process", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_callback_mode_process(&self,) -> crate::engine::animation_mixer::AnimationCallbackModeProcess {
            type RetMarshal = PtrcallReturnT < crate::engine::animation_mixer::AnimationCallbackModeProcess >;
            type CallSig = (crate::engine::animation_mixer::AnimationCallbackModeProcess,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(255usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_callback_mode_process", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_callback_mode_method(&mut self, mode: crate::engine::animation_mixer::AnimationCallbackModeMethod,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::animation_mixer::AnimationCallbackModeMethod);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(256usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_callback_mode_method", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_callback_mode_method(&self,) -> crate::engine::animation_mixer::AnimationCallbackModeMethod {
            type RetMarshal = PtrcallReturnT < crate::engine::animation_mixer::AnimationCallbackModeMethod >;
            type CallSig = (crate::engine::animation_mixer::AnimationCallbackModeMethod,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(257usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_callback_mode_method", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_audio_max_polyphony(&mut self, max_polyphony: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (max_polyphony,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(258usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_audio_max_polyphony", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_audio_max_polyphony(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(259usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_audio_max_polyphony", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_root_motion_track(&mut self, path: NodePath,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), NodePath);
            let args = (path,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(260usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_root_motion_track", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_root_motion_track(&self,) -> NodePath {
            type RetMarshal = PtrcallReturnT < NodePath >;
            type CallSig = (NodePath,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(261usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_root_motion_track", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_root_motion_position(&self,) -> Vector3 {
            type RetMarshal = PtrcallReturnT < Vector3 >;
            type CallSig = (Vector3,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(262usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_root_motion_position", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_root_motion_rotation(&self,) -> Quaternion {
            type RetMarshal = PtrcallReturnT < Quaternion >;
            type CallSig = (Quaternion,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(263usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_root_motion_rotation", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_root_motion_scale(&self,) -> Vector3 {
            type RetMarshal = PtrcallReturnT < Vector3 >;
            type CallSig = (Vector3,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(264usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_root_motion_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_root_motion_position_accumulator(&self,) -> Vector3 {
            type RetMarshal = PtrcallReturnT < Vector3 >;
            type CallSig = (Vector3,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(265usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_root_motion_position_accumulator", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_root_motion_rotation_accumulator(&self,) -> Quaternion {
            type RetMarshal = PtrcallReturnT < Quaternion >;
            type CallSig = (Quaternion,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(266usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_root_motion_rotation_accumulator", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_root_motion_scale_accumulator(&self,) -> Vector3 {
            type RetMarshal = PtrcallReturnT < Vector3 >;
            type CallSig = (Vector3,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(267usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_root_motion_scale_accumulator", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn clear_caches(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(268usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "clear_caches", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn advance(&mut self, delta: f64,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f64);
            let args = (delta,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(269usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "advance", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_reset_on_save_enabled(&mut self, enabled: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(270usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_reset_on_save_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_reset_on_save_enabled(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(271usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_reset_on_save_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn find_animation(&self, animation: Gd < crate::engine::Animation >,) -> StringName {
            type RetMarshal = PtrcallReturnT < StringName >;
            type CallSig = (StringName, Gd < crate::engine::Animation >);
            let args = (animation,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(272usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "find_animation", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn find_animation_library(&self, animation: Gd < crate::engine::Animation >,) -> StringName {
            type RetMarshal = PtrcallReturnT < StringName >;
            type CallSig = (StringName, Gd < crate::engine::Animation >);
            let args = (animation,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(273usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "find_animation_library", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for AnimationMixer {
        type Base = crate::engine::Node;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"AnimationMixer\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for AnimationMixer {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for AnimationMixer {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::Node > for AnimationMixer {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for AnimationMixer {
        
    }
    impl crate::obj::ExportableObject for AnimationMixer {
        
    }
    impl std::ops::Deref for AnimationMixer {
        type Target = crate::engine::Node;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for AnimationMixer {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_AnimationMixer {
        ($Class: ident) => {
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
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct AnimationCallbackModeProcess {
    ord: i32
}
impl AnimationCallbackModeProcess {
    pub const ANIMATION_CALLBACK_MODE_PROCESS_PHYSICS: Self = Self {
        ord: 0i32
    };
    pub const ANIMATION_CALLBACK_MODE_PROCESS_IDLE: Self = Self {
        ord: 1i32
    };
    pub const ANIMATION_CALLBACK_MODE_PROCESS_MANUAL: Self = Self {
        ord: 2i32
    };
    
}
impl crate::obj::EngineEnum for AnimationCallbackModeProcess {
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
impl crate::builtin::meta::GodotConvert for AnimationCallbackModeProcess {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for AnimationCallbackModeProcess {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for AnimationCallbackModeProcess {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct AnimationCallbackModeMethod {
    ord: i32
}
impl AnimationCallbackModeMethod {
    pub const ANIMATION_CALLBACK_MODE_METHOD_DEFERRED: Self = Self {
        ord: 0i32
    };
    pub const ANIMATION_CALLBACK_MODE_METHOD_IMMEDIATE: Self = Self {
        ord: 1i32
    };
    
}
impl crate::obj::EngineEnum for AnimationCallbackModeMethod {
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
impl crate::builtin::meta::GodotConvert for AnimationCallbackModeMethod {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for AnimationCallbackModeMethod {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for AnimationCallbackModeMethod {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}