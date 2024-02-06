#![doc = "Sidecar module for class [`XrInterface`][crate::engine::XrInterface].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `XRInterface` enums](https://docs.godotengine.org/en/stable/classes/class_xrinterface.html#enumerations).\n\n"]
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
    #[doc = "Godot class `XRInterface.`\n\nInherits [`RefCounted`][crate::engine::RefCounted].\n\nRelated symbols:\n\n* [`xr_interface`][crate::engine::xr_interface]: sidecar module with related enum/flag types\n* [`IXrInterface`][crate::engine::IXrInterface]: virtual methods\n\n\nSee also [Godot docs for `XRInterface`](https://docs.godotengine.org/en/stable/classes/class_xrinterface.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct XrInterface {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`XrInterface`][crate::engine::XrInterface].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `XRInterface` methods](https://docs.godotengine.org/en/stable/classes/class_xrinterface.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IXrInterface: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
    impl XrInterface {
        pub fn get_name(&self,) -> StringName {
            type RetMarshal = PtrcallReturnT < StringName >;
            type CallSig = (StringName,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9974usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_name", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_capabilities(&self,) -> u32 {
            type RetMarshal = PtrcallReturnT < u32 >;
            type CallSig = (u32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9975usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_capabilities", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_primary(&mut self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9976usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_primary", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_primary(&mut self, primary: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (primary,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9977usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_primary", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_initialized(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9978usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_initialized", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn initialize(&mut self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9979usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "initialize", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn uninitialize(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9980usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "uninitialize", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_system_info(&mut self,) -> Dictionary {
            type RetMarshal = PtrcallReturnT < Dictionary >;
            type CallSig = (Dictionary,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9981usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_system_info", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_tracking_status(&self,) -> crate::engine::xr_interface::TrackingStatus {
            type RetMarshal = PtrcallReturnT < crate::engine::xr_interface::TrackingStatus >;
            type CallSig = (crate::engine::xr_interface::TrackingStatus,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9982usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_tracking_status", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_render_target_size(&mut self,) -> Vector2 {
            type RetMarshal = PtrcallReturnT < Vector2 >;
            type CallSig = (Vector2,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9983usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_render_target_size", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_view_count(&mut self,) -> u32 {
            type RetMarshal = PtrcallReturnT < u32 >;
            type CallSig = (u32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9984usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_view_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn trigger_haptic_pulse(&mut self, action_name: GString, tracker_name: StringName, frequency: f64, amplitude: f64, duration_sec: f64, delay_sec: f64,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString, StringName, f64, f64, f64, f64);
            let args = (action_name, tracker_name, frequency, amplitude, duration_sec, delay_sec,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9985usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "trigger_haptic_pulse", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn supports_play_area_mode(&mut self, mode: crate::engine::xr_interface::PlayAreaMode,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, crate::engine::xr_interface::PlayAreaMode);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9986usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "supports_play_area_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_play_area_mode(&self,) -> crate::engine::xr_interface::PlayAreaMode {
            type RetMarshal = PtrcallReturnT < crate::engine::xr_interface::PlayAreaMode >;
            type CallSig = (crate::engine::xr_interface::PlayAreaMode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9987usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_play_area_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_play_area_mode(&mut self, mode: crate::engine::xr_interface::PlayAreaMode,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, crate::engine::xr_interface::PlayAreaMode);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9988usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_play_area_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_play_area(&self,) -> PackedVector3Array {
            type RetMarshal = PtrcallReturnT < PackedVector3Array >;
            type CallSig = (PackedVector3Array,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9989usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_play_area", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_anchor_detection_is_enabled(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9990usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_anchor_detection_is_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_anchor_detection_is_enabled(&mut self, enable: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9991usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_anchor_detection_is_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_camera_feed_id(&mut self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9992usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_camera_feed_id", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_passthrough_supported(&mut self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9993usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_passthrough_supported", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_passthrough_enabled(&mut self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9994usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_passthrough_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn start_passthrough(&mut self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9995usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "start_passthrough", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn stop_passthrough(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9996usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "stop_passthrough", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_transform_for_view(&mut self, view: u32, cam_transform: Transform3D,) -> Transform3D {
            type RetMarshal = PtrcallReturnT < Transform3D >;
            type CallSig = (Transform3D, u32, Transform3D);
            let args = (view, cam_transform,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9997usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_transform_for_view", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_projection_for_view(&mut self, view: u32, aspect: f64, near: f64, far: f64,) -> Projection {
            type RetMarshal = PtrcallReturnT < Projection >;
            type CallSig = (Projection, u32, f64, f64, f64);
            let args = (view, aspect, near, far,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9998usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_projection_for_view", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_supported_environment_blend_modes(&mut self,) -> VariantArray {
            type RetMarshal = PtrcallReturnT < VariantArray >;
            type CallSig = (VariantArray,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9999usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_supported_environment_blend_modes", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_environment_blend_mode(&mut self, mode: crate::engine::xr_interface::EnvironmentBlendMode,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, crate::engine::xr_interface::EnvironmentBlendMode);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10000usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_environment_blend_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_environment_blend_mode(&self,) -> crate::engine::xr_interface::EnvironmentBlendMode {
            type RetMarshal = PtrcallReturnT < crate::engine::xr_interface::EnvironmentBlendMode >;
            type CallSig = (crate::engine::xr_interface::EnvironmentBlendMode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10001usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_environment_blend_mode", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for XrInterface {
        type Base = crate::engine::RefCounted;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"XRInterface\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for XrInterface {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for XrInterface {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::RefCounted > for XrInterface {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for XrInterface {
        
    }
    impl std::ops::Deref for XrInterface {
        type Target = crate::engine::RefCounted;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for XrInterface {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_XrInterface {
        ($Class: ident) => {
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
pub struct Capabilities {
    ord: i32
}
impl Capabilities {
    pub const XR_NONE: Self = Self {
        ord: 0i32
    };
    pub const XR_MONO: Self = Self {
        ord: 1i32
    };
    pub const XR_STEREO: Self = Self {
        ord: 2i32
    };
    pub const XR_QUAD: Self = Self {
        ord: 4i32
    };
    pub const XR_VR: Self = Self {
        ord: 8i32
    };
    pub const XR_AR: Self = Self {
        ord: 16i32
    };
    pub const XR_EXTERNAL: Self = Self {
        ord: 32i32
    };
    
}
impl crate::obj::EngineEnum for Capabilities {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 4i32 | ord @ 8i32 | ord @ 16i32 | ord @ 32i32 => Some(Self {
                ord
            }), _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
impl crate::builtin::meta::GodotConvert for Capabilities {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for Capabilities {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for Capabilities {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct TrackingStatus {
    ord: i32
}
impl TrackingStatus {
    pub const XR_NORMAL_TRACKING: Self = Self {
        ord: 0i32
    };
    pub const XR_EXCESSIVE_MOTION: Self = Self {
        ord: 1i32
    };
    pub const XR_INSUFFICIENT_FEATURES: Self = Self {
        ord: 2i32
    };
    pub const XR_UNKNOWN_TRACKING: Self = Self {
        ord: 3i32
    };
    pub const XR_NOT_TRACKING: Self = Self {
        ord: 4i32
    };
    
}
impl crate::obj::EngineEnum for TrackingStatus {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 => Some(Self {
                ord
            }), _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
impl crate::builtin::meta::GodotConvert for TrackingStatus {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for TrackingStatus {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for TrackingStatus {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct PlayAreaMode {
    ord: i32
}
impl PlayAreaMode {
    pub const XR_PLAY_AREA_UNKNOWN: Self = Self {
        ord: 0i32
    };
    pub const XR_PLAY_AREA_3DOF: Self = Self {
        ord: 1i32
    };
    pub const XR_PLAY_AREA_SITTING: Self = Self {
        ord: 2i32
    };
    pub const XR_PLAY_AREA_ROOMSCALE: Self = Self {
        ord: 3i32
    };
    pub const XR_PLAY_AREA_STAGE: Self = Self {
        ord: 4i32
    };
    
}
impl crate::obj::EngineEnum for PlayAreaMode {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 => Some(Self {
                ord
            }), _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
impl crate::builtin::meta::GodotConvert for PlayAreaMode {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for PlayAreaMode {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for PlayAreaMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct EnvironmentBlendMode {
    ord: i32
}
impl EnvironmentBlendMode {
    pub const XR_ENV_BLEND_MODE_OPAQUE: Self = Self {
        ord: 0i32
    };
    pub const XR_ENV_BLEND_MODE_ADDITIVE: Self = Self {
        ord: 1i32
    };
    pub const XR_ENV_BLEND_MODE_ALPHA_BLEND: Self = Self {
        ord: 2i32
    };
    
}
impl crate::obj::EngineEnum for EnvironmentBlendMode {
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
impl crate::builtin::meta::GodotConvert for EnvironmentBlendMode {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for EnvironmentBlendMode {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for EnvironmentBlendMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}