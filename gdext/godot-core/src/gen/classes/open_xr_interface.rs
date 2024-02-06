#![doc = "Sidecar module for class [`OpenXrInterface`][crate::engine::OpenXrInterface].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `OpenXRInterface` enums](https://docs.godotengine.org/en/stable/classes/class_openxrinterface.html#enumerations).\n\n"]
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
    #[doc = "Godot class `OpenXRInterface.`\n\nInherits [`XrInterface`][crate::engine::XrInterface].\n\nRelated symbols:\n\n* [`open_xr_interface`][crate::engine::open_xr_interface]: sidecar module with related enum/flag types\n* [`IOpenXrInterface`][crate::engine::IOpenXrInterface]: virtual methods\n\n\nSee also [Godot docs for `OpenXRInterface`](https://docs.godotengine.org/en/stable/classes/class_openxrinterface.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct OpenXrInterface {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`OpenXrInterface`][crate::engine::OpenXrInterface].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `OpenXRInterface` methods](https://docs.godotengine.org/en/stable/classes/class_openxrinterface.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IOpenXrInterface: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
    impl OpenXrInterface {
        #[deprecated = "Replaced with `new_gd` in extension trait `NewGd`."]
        pub fn new() -> Gd < Self > {
            crate::obj::Gd::default()
        }
        pub fn get_display_refresh_rate(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5343usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_display_refresh_rate", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_display_refresh_rate(&mut self, refresh_rate: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (refresh_rate,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5344usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_display_refresh_rate", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_render_target_size_multiplier(&self,) -> f64 {
            type RetMarshal = PtrcallReturnT < f64 >;
            type CallSig = (f64,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5345usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_render_target_size_multiplier", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_render_target_size_multiplier(&mut self, multiplier: f64,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f64);
            let args = (multiplier,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5346usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_render_target_size_multiplier", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_foveation_supported(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5347usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_foveation_supported", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_foveation_level(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5348usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_foveation_level", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_foveation_level(&mut self, foveation_level: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (foveation_level,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5349usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_foveation_level", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_foveation_dynamic(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5350usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_foveation_dynamic", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_foveation_dynamic(&mut self, foveation_dynamic: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (foveation_dynamic,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5351usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_foveation_dynamic", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_action_set_active(&self, name: GString,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, GString);
            let args = (name,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5352usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_action_set_active", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_action_set_active(&mut self, name: GString, active: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString, bool);
            let args = (name, active,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5353usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_action_set_active", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_action_sets(&self,) -> VariantArray {
            type RetMarshal = PtrcallReturnT < VariantArray >;
            type CallSig = (VariantArray,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5354usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_action_sets", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_available_display_refresh_rates(&self,) -> VariantArray {
            type RetMarshal = PtrcallReturnT < VariantArray >;
            type CallSig = (VariantArray,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5355usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_available_display_refresh_rates", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_motion_range(&mut self, hand: crate::engine::open_xr_interface::Hand, motion_range: crate::engine::open_xr_interface::HandMotionRange,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::open_xr_interface::Hand, crate::engine::open_xr_interface::HandMotionRange);
            let args = (hand, motion_range,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5356usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_motion_range", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_motion_range(&self, hand: crate::engine::open_xr_interface::Hand,) -> crate::engine::open_xr_interface::HandMotionRange {
            type RetMarshal = PtrcallReturnT < crate::engine::open_xr_interface::HandMotionRange >;
            type CallSig = (crate::engine::open_xr_interface::HandMotionRange, crate::engine::open_xr_interface::Hand);
            let args = (hand,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5357usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_motion_range", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_hand_joint_flags(&self, hand: crate::engine::open_xr_interface::Hand, joint: crate::engine::open_xr_interface::HandJoints,) -> crate::engine::open_xr_interface::HandJointFlags {
            type RetMarshal = PtrcallReturnT < crate::engine::open_xr_interface::HandJointFlags >;
            type CallSig = (crate::engine::open_xr_interface::HandJointFlags, crate::engine::open_xr_interface::Hand, crate::engine::open_xr_interface::HandJoints);
            let args = (hand, joint,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5358usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_hand_joint_flags", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_hand_joint_rotation(&self, hand: crate::engine::open_xr_interface::Hand, joint: crate::engine::open_xr_interface::HandJoints,) -> Quaternion {
            type RetMarshal = PtrcallReturnT < Quaternion >;
            type CallSig = (Quaternion, crate::engine::open_xr_interface::Hand, crate::engine::open_xr_interface::HandJoints);
            let args = (hand, joint,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5359usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_hand_joint_rotation", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_hand_joint_position(&self, hand: crate::engine::open_xr_interface::Hand, joint: crate::engine::open_xr_interface::HandJoints,) -> Vector3 {
            type RetMarshal = PtrcallReturnT < Vector3 >;
            type CallSig = (Vector3, crate::engine::open_xr_interface::Hand, crate::engine::open_xr_interface::HandJoints);
            let args = (hand, joint,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5360usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_hand_joint_position", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_hand_joint_radius(&self, hand: crate::engine::open_xr_interface::Hand, joint: crate::engine::open_xr_interface::HandJoints,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32, crate::engine::open_xr_interface::Hand, crate::engine::open_xr_interface::HandJoints);
            let args = (hand, joint,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5361usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_hand_joint_radius", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_hand_joint_linear_velocity(&self, hand: crate::engine::open_xr_interface::Hand, joint: crate::engine::open_xr_interface::HandJoints,) -> Vector3 {
            type RetMarshal = PtrcallReturnT < Vector3 >;
            type CallSig = (Vector3, crate::engine::open_xr_interface::Hand, crate::engine::open_xr_interface::HandJoints);
            let args = (hand, joint,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5362usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_hand_joint_linear_velocity", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_hand_joint_angular_velocity(&self, hand: crate::engine::open_xr_interface::Hand, joint: crate::engine::open_xr_interface::HandJoints,) -> Vector3 {
            type RetMarshal = PtrcallReturnT < Vector3 >;
            type CallSig = (Vector3, crate::engine::open_xr_interface::Hand, crate::engine::open_xr_interface::HandJoints);
            let args = (hand, joint,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5363usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_hand_joint_angular_velocity", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_hand_tracking_supported(&mut self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5364usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_hand_tracking_supported", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_eye_gaze_interaction_supported(&mut self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5365usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_eye_gaze_interaction_supported", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for OpenXrInterface {
        type Base = crate::engine::XrInterface;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"OpenXRInterface\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for OpenXrInterface {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for OpenXrInterface {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::XrInterface > for OpenXrInterface {
        
    }
    impl crate::obj::Inherits < crate::engine::RefCounted > for OpenXrInterface {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for OpenXrInterface {
        
    }
    impl crate::obj::cap::GodotDefault for OpenXrInterface {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for OpenXrInterface {
        type Target = crate::engine::XrInterface;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for OpenXrInterface {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_OpenXrInterface {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::OpenXrInterface > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::XrInterface > for $Class {
                
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
pub struct Hand {
    ord: i32
}
impl Hand {
    pub const HAND_LEFT: Self = Self {
        ord: 0i32
    };
    pub const HAND_RIGHT: Self = Self {
        ord: 1i32
    };
    pub const HAND_MAX: Self = Self {
        ord: 2i32
    };
    
}
impl crate::obj::EngineEnum for Hand {
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
impl crate::obj::IndexEnum for Hand {
    const ENUMERATOR_COUNT: usize = 2usize;
    
}
impl crate::builtin::meta::GodotConvert for Hand {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for Hand {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for Hand {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct HandMotionRange {
    ord: i32
}
impl HandMotionRange {
    pub const HAND_MOTION_RANGE_UNOBSTRUCTED: Self = Self {
        ord: 0i32
    };
    pub const HAND_MOTION_RANGE_CONFORM_TO_CONTROLLER: Self = Self {
        ord: 1i32
    };
    pub const HAND_MOTION_RANGE_MAX: Self = Self {
        ord: 2i32
    };
    
}
impl crate::obj::EngineEnum for HandMotionRange {
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
impl crate::obj::IndexEnum for HandMotionRange {
    const ENUMERATOR_COUNT: usize = 2usize;
    
}
impl crate::builtin::meta::GodotConvert for HandMotionRange {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for HandMotionRange {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for HandMotionRange {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct HandJoints {
    ord: i32
}
impl HandJoints {
    pub const HAND_JOINT_PALM: Self = Self {
        ord: 0i32
    };
    pub const HAND_JOINT_WRIST: Self = Self {
        ord: 1i32
    };
    pub const HAND_JOINT_THUMB_METACARPAL: Self = Self {
        ord: 2i32
    };
    pub const HAND_JOINT_THUMB_PROXIMAL: Self = Self {
        ord: 3i32
    };
    pub const HAND_JOINT_THUMB_DISTAL: Self = Self {
        ord: 4i32
    };
    pub const HAND_JOINT_THUMB_TIP: Self = Self {
        ord: 5i32
    };
    pub const HAND_JOINT_INDEX_METACARPAL: Self = Self {
        ord: 6i32
    };
    pub const HAND_JOINT_INDEX_PROXIMAL: Self = Self {
        ord: 7i32
    };
    pub const HAND_JOINT_INDEX_INTERMEDIATE: Self = Self {
        ord: 8i32
    };
    pub const HAND_JOINT_INDEX_DISTAL: Self = Self {
        ord: 9i32
    };
    pub const HAND_JOINT_INDEX_TIP: Self = Self {
        ord: 10i32
    };
    pub const HAND_JOINT_MIDDLE_METACARPAL: Self = Self {
        ord: 11i32
    };
    pub const HAND_JOINT_MIDDLE_PROXIMAL: Self = Self {
        ord: 12i32
    };
    pub const HAND_JOINT_MIDDLE_INTERMEDIATE: Self = Self {
        ord: 13i32
    };
    pub const HAND_JOINT_MIDDLE_DISTAL: Self = Self {
        ord: 14i32
    };
    pub const HAND_JOINT_MIDDLE_TIP: Self = Self {
        ord: 15i32
    };
    pub const HAND_JOINT_RING_METACARPAL: Self = Self {
        ord: 16i32
    };
    pub const HAND_JOINT_RING_PROXIMAL: Self = Self {
        ord: 17i32
    };
    pub const HAND_JOINT_RING_INTERMEDIATE: Self = Self {
        ord: 18i32
    };
    pub const HAND_JOINT_RING_DISTAL: Self = Self {
        ord: 19i32
    };
    pub const HAND_JOINT_RING_TIP: Self = Self {
        ord: 20i32
    };
    pub const HAND_JOINT_LITTLE_METACARPAL: Self = Self {
        ord: 21i32
    };
    pub const HAND_JOINT_LITTLE_PROXIMAL: Self = Self {
        ord: 22i32
    };
    pub const HAND_JOINT_LITTLE_INTERMEDIATE: Self = Self {
        ord: 23i32
    };
    pub const HAND_JOINT_LITTLE_DISTAL: Self = Self {
        ord: 24i32
    };
    pub const HAND_JOINT_LITTLE_TIP: Self = Self {
        ord: 25i32
    };
    pub const HAND_JOINT_MAX: Self = Self {
        ord: 26i32
    };
    
}
impl crate::obj::EngineEnum for HandJoints {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 | ord @ 9i32 | ord @ 10i32 | ord @ 11i32 | ord @ 12i32 | ord @ 13i32 | ord @ 14i32 | ord @ 15i32 | ord @ 16i32 | ord @ 17i32 | ord @ 18i32 | ord @ 19i32 | ord @ 20i32 | ord @ 21i32 | ord @ 22i32 | ord @ 23i32 | ord @ 24i32 | ord @ 25i32 | ord @ 26i32 => Some(Self {
                ord
            }), _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
impl crate::obj::IndexEnum for HandJoints {
    const ENUMERATOR_COUNT: usize = 26usize;
    
}
impl crate::builtin::meta::GodotConvert for HandJoints {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for HandJoints {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for HandJoints {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub struct HandJointFlags {
    ord: u64
}
impl HandJointFlags {
    pub const HAND_JOINT_NONE: Self = Self {
        ord: 0u64
    };
    pub const HAND_JOINT_ORIENTATION_VALID: Self = Self {
        ord: 1u64
    };
    pub const HAND_JOINT_ORIENTATION_TRACKED: Self = Self {
        ord: 2u64
    };
    pub const HAND_JOINT_POSITION_VALID: Self = Self {
        ord: 4u64
    };
    pub const HAND_JOINT_POSITION_TRACKED: Self = Self {
        ord: 8u64
    };
    pub const HAND_JOINT_LINEAR_VELOCITY_VALID: Self = Self {
        ord: 16u64
    };
    pub const HAND_JOINT_ANGULAR_VELOCITY_VALID: Self = Self {
        ord: 32u64
    };
    
}
impl crate::obj::EngineBitfield for HandJointFlags {
    fn try_from_ord(ord: u64) -> Option < Self > {
        Some(Self {
            ord
        })
    }
    fn ord(self) -> u64 {
        self.ord
    }
}
impl std::ops::BitOr for HandJointFlags {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            ord: self.ord | rhs.ord
        }
    }
}
impl crate::builtin::meta::GodotConvert for HandJointFlags {
    type Via = u64;
    
}
impl crate::builtin::meta::ToGodot for HandJointFlags {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineBitfield > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for HandJointFlags {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineBitfield > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}