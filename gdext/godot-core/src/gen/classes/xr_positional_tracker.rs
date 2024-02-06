#![doc = "Sidecar module for class [`XrPositionalTracker`][crate::engine::XrPositionalTracker].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `XRPositionalTracker` enums](https://docs.godotengine.org/en/stable/classes/class_xrpositionaltracker.html#enumerations).\n\n"]
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
    #[doc = "Godot class `XRPositionalTracker.`\n\nInherits [`RefCounted`][crate::engine::RefCounted].\n\nRelated symbols:\n\n* [`xr_positional_tracker`][crate::engine::xr_positional_tracker]: sidecar module with related enum/flag types\n* [`IXrPositionalTracker`][crate::engine::IXrPositionalTracker]: virtual methods\n\n\nSee also [Godot docs for `XRPositionalTracker`](https://docs.godotengine.org/en/stable/classes/class_xrpositionaltracker.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct XrPositionalTracker {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`XrPositionalTracker`][crate::engine::XrPositionalTracker].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `XRPositionalTracker` methods](https://docs.godotengine.org/en/stable/classes/class_xrpositionaltracker.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IXrPositionalTracker: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
    impl XrPositionalTracker {
        #[deprecated = "Replaced with `new_gd` in extension trait `NewGd`."]
        pub fn new() -> Gd < Self > {
            crate::obj::Gd::default()
        }
        pub fn get_tracker_type(&self,) -> crate::engine::xr_server::TrackerType {
            type RetMarshal = PtrcallReturnT < crate::engine::xr_server::TrackerType >;
            type CallSig = (crate::engine::xr_server::TrackerType,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10032usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_tracker_type", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_tracker_type(&mut self, type_: crate::engine::xr_server::TrackerType,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::xr_server::TrackerType);
            let args = (type_,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10033usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_tracker_type", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_tracker_name(&self,) -> StringName {
            type RetMarshal = PtrcallReturnT < StringName >;
            type CallSig = (StringName,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10034usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_tracker_name", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_tracker_name(&mut self, name: StringName,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), StringName);
            let args = (name,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10035usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_tracker_name", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_tracker_desc(&self,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10036usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_tracker_desc", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_tracker_desc(&mut self, description: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString);
            let args = (description,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10037usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_tracker_desc", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_tracker_profile(&self,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10038usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_tracker_profile", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_tracker_profile(&mut self, profile: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString);
            let args = (profile,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10039usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_tracker_profile", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_tracker_hand(&self,) -> crate::engine::xr_positional_tracker::TrackerHand {
            type RetMarshal = PtrcallReturnT < crate::engine::xr_positional_tracker::TrackerHand >;
            type CallSig = (crate::engine::xr_positional_tracker::TrackerHand,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10040usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_tracker_hand", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_tracker_hand(&mut self, hand: crate::engine::xr_positional_tracker::TrackerHand,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::xr_positional_tracker::TrackerHand);
            let args = (hand,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10041usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_tracker_hand", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn has_pose(&self, name: StringName,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, StringName);
            let args = (name,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10042usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "has_pose", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_pose(&self, name: StringName,) -> Option < Gd < crate::engine::XrPose > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::XrPose > >;
            type CallSig = (Option < Gd < crate::engine::XrPose > >, StringName);
            let args = (name,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10043usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_pose", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn invalidate_pose(&mut self, name: StringName,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), StringName);
            let args = (name,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10044usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "invalidate_pose", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_pose(&mut self, name: StringName, transform: Transform3D, linear_velocity: Vector3, angular_velocity: Vector3, tracking_confidence: crate::engine::xr_pose::TrackingConfidence,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), StringName, Transform3D, Vector3, Vector3, crate::engine::xr_pose::TrackingConfidence);
            let args = (name, transform, linear_velocity, angular_velocity, tracking_confidence,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10045usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_pose", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_input(&self, name: StringName,) -> Variant {
            type RetMarshal = PtrcallReturnT < Variant >;
            type CallSig = (Variant, StringName);
            let args = (name,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10046usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_input", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_input(&mut self, name: StringName, value: Variant,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), StringName, Variant);
            let args = (name, value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10047usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_input", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for XrPositionalTracker {
        type Base = crate::engine::RefCounted;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"XRPositionalTracker\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for XrPositionalTracker {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for XrPositionalTracker {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::RefCounted > for XrPositionalTracker {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for XrPositionalTracker {
        
    }
    impl crate::obj::cap::GodotDefault for XrPositionalTracker {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for XrPositionalTracker {
        type Target = crate::engine::RefCounted;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for XrPositionalTracker {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_XrPositionalTracker {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::XrPositionalTracker > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::RefCounted > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::Object > for $Class {
                
            }
        }
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct TrackerHand {
    ord: i32
}
impl TrackerHand {
    pub const TRACKER_HAND_UNKNOWN: Self = Self {
        ord: 0i32
    };
    pub const TRACKER_HAND_LEFT: Self = Self {
        ord: 1i32
    };
    pub const TRACKER_HAND_RIGHT: Self = Self {
        ord: 2i32
    };
    
}
impl crate::obj::EngineEnum for TrackerHand {
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
impl crate::builtin::meta::GodotConvert for TrackerHand {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for TrackerHand {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for TrackerHand {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}