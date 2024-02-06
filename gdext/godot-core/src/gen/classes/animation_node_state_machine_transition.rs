#![doc = "Sidecar module for class [`AnimationNodeStateMachineTransition`][crate::engine::AnimationNodeStateMachineTransition].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `AnimationNodeStateMachineTransition` enums](https://docs.godotengine.org/en/stable/classes/class_animationnodestatemachinetransition.html#enumerations).\n\n"]
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
    #[doc = "Godot class `AnimationNodeStateMachineTransition.`\n\nInherits [`Resource`][crate::engine::Resource].\n\nRelated symbols:\n\n* [`animation_node_state_machine_transition`][crate::engine::animation_node_state_machine_transition]: sidecar module with related enum/flag types\n* [`IAnimationNodeStateMachineTransition`][crate::engine::IAnimationNodeStateMachineTransition]: virtual methods\n\n\nSee also [Godot docs for `AnimationNodeStateMachineTransition`](https://docs.godotengine.org/en/stable/classes/class_animationnodestatemachinetransition.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct AnimationNodeStateMachineTransition {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`AnimationNodeStateMachineTransition`][crate::engine::AnimationNodeStateMachineTransition].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `AnimationNodeStateMachineTransition` methods](https://docs.godotengine.org/en/stable/classes/class_animationnodestatemachinetransition.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IAnimationNodeStateMachineTransition: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
    impl AnimationNodeStateMachineTransition {
        #[deprecated = "Replaced with `new_gd` in extension trait `NewGd`."]
        pub fn new() -> Gd < Self > {
            crate::obj::Gd::default()
        }
        pub fn set_switch_mode(&mut self, mode: crate::engine::animation_node_state_machine_transition::SwitchMode,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::animation_node_state_machine_transition::SwitchMode);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(401usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_switch_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_switch_mode(&self,) -> crate::engine::animation_node_state_machine_transition::SwitchMode {
            type RetMarshal = PtrcallReturnT < crate::engine::animation_node_state_machine_transition::SwitchMode >;
            type CallSig = (crate::engine::animation_node_state_machine_transition::SwitchMode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(402usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_switch_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_advance_mode(&mut self, mode: crate::engine::animation_node_state_machine_transition::AdvanceMode,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::animation_node_state_machine_transition::AdvanceMode);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(403usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_advance_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_advance_mode(&self,) -> crate::engine::animation_node_state_machine_transition::AdvanceMode {
            type RetMarshal = PtrcallReturnT < crate::engine::animation_node_state_machine_transition::AdvanceMode >;
            type CallSig = (crate::engine::animation_node_state_machine_transition::AdvanceMode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(404usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_advance_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_advance_condition(&mut self, name: StringName,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), StringName);
            let args = (name,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(405usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_advance_condition", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_advance_condition(&self,) -> StringName {
            type RetMarshal = PtrcallReturnT < StringName >;
            type CallSig = (StringName,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(406usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_advance_condition", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_xfade_time(&mut self, secs: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (secs,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(407usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_xfade_time", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_xfade_time(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(408usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_xfade_time", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_xfade_curve(&mut self, curve: Gd < crate::engine::Curve >,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), Gd < crate::engine::Curve >);
            let args = (curve,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(409usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_xfade_curve", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_xfade_curve(&self,) -> Option < Gd < crate::engine::Curve > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::Curve > >;
            type CallSig = (Option < Gd < crate::engine::Curve > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(410usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_xfade_curve", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_reset(&mut self, reset: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (reset,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(411usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_reset", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_reset(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(412usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_reset", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_priority(&mut self, priority: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (priority,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(413usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_priority", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_priority(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(414usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_priority", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_advance_expression(&mut self, text: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString);
            let args = (text,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(415usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_advance_expression", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_advance_expression(&self,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(416usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_advance_expression", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for AnimationNodeStateMachineTransition {
        type Base = crate::engine::Resource;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"AnimationNodeStateMachineTransition\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for AnimationNodeStateMachineTransition {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for AnimationNodeStateMachineTransition {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::Resource > for AnimationNodeStateMachineTransition {
        
    }
    impl crate::obj::Inherits < crate::engine::RefCounted > for AnimationNodeStateMachineTransition {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for AnimationNodeStateMachineTransition {
        
    }
    impl crate::obj::ExportableObject for AnimationNodeStateMachineTransition {
        
    }
    impl crate::obj::cap::GodotDefault for AnimationNodeStateMachineTransition {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for AnimationNodeStateMachineTransition {
        type Target = crate::engine::Resource;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for AnimationNodeStateMachineTransition {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_AnimationNodeStateMachineTransition {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::AnimationNodeStateMachineTransition > for $Class {
                
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
pub struct SwitchMode {
    ord: i32
}
impl SwitchMode {
    pub const SWITCH_MODE_IMMEDIATE: Self = Self {
        ord: 0i32
    };
    pub const SWITCH_MODE_SYNC: Self = Self {
        ord: 1i32
    };
    pub const SWITCH_MODE_AT_END: Self = Self {
        ord: 2i32
    };
    
}
impl crate::obj::EngineEnum for SwitchMode {
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
impl crate::builtin::meta::GodotConvert for SwitchMode {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for SwitchMode {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for SwitchMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct AdvanceMode {
    ord: i32
}
impl AdvanceMode {
    pub const ADVANCE_MODE_DISABLED: Self = Self {
        ord: 0i32
    };
    pub const ADVANCE_MODE_ENABLED: Self = Self {
        ord: 1i32
    };
    pub const ADVANCE_MODE_AUTO: Self = Self {
        ord: 2i32
    };
    
}
impl crate::obj::EngineEnum for AdvanceMode {
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
impl crate::builtin::meta::GodotConvert for AdvanceMode {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for AdvanceMode {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for AdvanceMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}