#![doc = "Sidecar module for class [`AnimationNodeStateMachinePlayback`][crate::engine::AnimationNodeStateMachinePlayback].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `AnimationNodeStateMachinePlayback` enums](https://docs.godotengine.org/en/stable/classes/class_animationnodestatemachineplayback.html#enumerations).\n\n"]
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
    #[doc = "Godot class `AnimationNodeStateMachinePlayback.`\n\nInherits [`Resource`][crate::engine::Resource].\n\nRelated symbols:\n\n* [`animation_node_state_machine_playback`][crate::engine::animation_node_state_machine_playback]: sidecar module with related enum/flag types\n* [`IAnimationNodeStateMachinePlayback`][crate::engine::IAnimationNodeStateMachinePlayback]: virtual methods\n\n\nSee also [Godot docs for `AnimationNodeStateMachinePlayback`](https://docs.godotengine.org/en/stable/classes/class_animationnodestatemachineplayback.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct AnimationNodeStateMachinePlayback {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`AnimationNodeStateMachinePlayback`][crate::engine::AnimationNodeStateMachinePlayback].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `AnimationNodeStateMachinePlayback` methods](https://docs.godotengine.org/en/stable/classes/class_animationnodestatemachineplayback.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IAnimationNodeStateMachinePlayback: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
        fn setup_local_to_scene(&mut self,) {
            unimplemented !()
        }
    }
    impl AnimationNodeStateMachinePlayback {
        #[deprecated = "Replaced with `new_gd` in extension trait `NewGd`."]
        pub fn new() -> Gd < Self > {
            crate::obj::Gd::default()
        }
        pub(crate) fn travel_full(&mut self, to_node: StringName, reset_on_teleport: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), StringName, bool);
            let args = (to_node, reset_on_teleport,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(391usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "travel", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn travel(&mut self, to_node: StringName,) {
            self.travel_ex(to_node,) . done()
        }
        #[inline]
        pub fn travel_ex(&mut self, to_node: StringName,) -> ExTravel < '_ > {
            ExTravel::new(self, to_node,)
        }
        pub(crate) fn start_full(&mut self, node: StringName, reset: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), StringName, bool);
            let args = (node, reset,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(392usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "start", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn start(&mut self, node: StringName,) {
            self.start_ex(node,) . done()
        }
        #[inline]
        pub fn start_ex(&mut self, node: StringName,) -> ExStart < '_ > {
            ExStart::new(self, node,)
        }
        pub fn next(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(393usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "next", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn stop(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(394usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "stop", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_playing(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(395usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_playing", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_current_node(&self,) -> StringName {
            type RetMarshal = PtrcallReturnT < StringName >;
            type CallSig = (StringName,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(396usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_current_node", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_current_play_position(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(397usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_current_play_position", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_current_length(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(398usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_current_length", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_fading_from_node(&self,) -> StringName {
            type RetMarshal = PtrcallReturnT < StringName >;
            type CallSig = (StringName,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(399usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_fading_from_node", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_travel_path(&self,) -> Array < StringName > {
            type RetMarshal = PtrcallReturnT < Array < StringName > >;
            type CallSig = (Array < StringName >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(400usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_travel_path", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for AnimationNodeStateMachinePlayback {
        type Base = crate::engine::Resource;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"AnimationNodeStateMachinePlayback\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for AnimationNodeStateMachinePlayback {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for AnimationNodeStateMachinePlayback {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::Resource > for AnimationNodeStateMachinePlayback {
        
    }
    impl crate::obj::Inherits < crate::engine::RefCounted > for AnimationNodeStateMachinePlayback {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for AnimationNodeStateMachinePlayback {
        
    }
    impl crate::obj::ExportableObject for AnimationNodeStateMachinePlayback {
        
    }
    impl crate::obj::cap::GodotDefault for AnimationNodeStateMachinePlayback {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for AnimationNodeStateMachinePlayback {
        type Target = crate::engine::Resource;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for AnimationNodeStateMachinePlayback {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_AnimationNodeStateMachinePlayback {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::AnimationNodeStateMachinePlayback > for $Class {
                
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
#[doc = "Default-param extender for [`AnimationNodeStateMachinePlayback::travel_ex`][super::AnimationNodeStateMachinePlayback::travel_ex]."]
#[must_use]
pub struct ExTravel < 'a > {
    surround_object: &'a mut re_export::AnimationNodeStateMachinePlayback, to_node: StringName, reset_on_teleport: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExTravel < 'a > {
    fn new(surround_object: &'a mut re_export::AnimationNodeStateMachinePlayback, to_node: StringName,) -> Self {
        Self {
            surround_object, to_node, reset_on_teleport: true,
        }
    }
    #[inline]
    pub fn reset_on_teleport(self, value: bool) -> Self {
        Self {
            reset_on_teleport: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::AnimationNodeStateMachinePlayback::travel_full(self.surround_object, self.to_node, self.reset_on_teleport,)
    }
}
#[doc = "Default-param extender for [`AnimationNodeStateMachinePlayback::start_ex`][super::AnimationNodeStateMachinePlayback::start_ex]."]
#[must_use]
pub struct ExStart < 'a > {
    surround_object: &'a mut re_export::AnimationNodeStateMachinePlayback, node: StringName, reset: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExStart < 'a > {
    fn new(surround_object: &'a mut re_export::AnimationNodeStateMachinePlayback, node: StringName,) -> Self {
        Self {
            surround_object, node, reset: true,
        }
    }
    #[inline]
    pub fn reset(self, value: bool) -> Self {
        Self {
            reset: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::AnimationNodeStateMachinePlayback::start_full(self.surround_object, self.node, self.reset,)
    }
}