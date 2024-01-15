#![doc = "Sidecar module for class [`WebXrInterface`][crate::engine::WebXrInterface].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `WebXRInterface` enums](https://docs.godotengine.org/en/stable/classes/class_webxrinterface.html#enumerations).\n\n"]
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
    #[doc = "Godot class `WebXRInterface.`\n\nInherits [`XrInterface`][crate::engine::XrInterface].\n\nRelated symbols:\n\n* [`web_xr_interface`][crate::engine::web_xr_interface]: sidecar module with related enum/flag types\n* [`IWebXrInterface`][crate::engine::IWebXrInterface]: virtual methods\n\n\nSee also [Godot docs for `WebXRInterface`](https://docs.godotengine.org/en/stable/classes/class_webxrinterface.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct WebXrInterface {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`WebXrInterface`][crate::engine::WebXrInterface].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `WebXRInterface` methods](https://docs.godotengine.org/en/stable/classes/class_webxrinterface.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IWebXrInterface: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
    impl WebXrInterface {
        pub fn is_session_supported(&mut self, session_mode: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString);
            let args = (session_mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9783usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_session_supported", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_session_mode(&mut self, session_mode: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString);
            let args = (session_mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9784usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_session_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_session_mode(&self,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9785usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_session_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_required_features(&mut self, required_features: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString);
            let args = (required_features,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9786usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_required_features", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_required_features(&self,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9787usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_required_features", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_optional_features(&mut self, optional_features: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString);
            let args = (optional_features,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9788usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_optional_features", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_optional_features(&self,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9789usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_optional_features", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_reference_space_type(&self,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9790usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_reference_space_type", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_requested_reference_space_types(&mut self, requested_reference_space_types: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString);
            let args = (requested_reference_space_types,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9791usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_requested_reference_space_types", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_requested_reference_space_types(&self,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9792usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_requested_reference_space_types", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_input_source_active(&self, input_source_id: i32,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, i32);
            let args = (input_source_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9793usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_input_source_active", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_input_source_tracker(&self, input_source_id: i32,) -> Option < Gd < crate::engine::XrPositionalTracker > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::XrPositionalTracker > >;
            type CallSig = (Option < Gd < crate::engine::XrPositionalTracker > >, i32);
            let args = (input_source_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9794usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_input_source_tracker", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_input_source_target_ray_mode(&self, input_source_id: i32,) -> crate::engine::web_xr_interface::TargetRayMode {
            type RetMarshal = PtrcallReturnT < crate::engine::web_xr_interface::TargetRayMode >;
            type CallSig = (crate::engine::web_xr_interface::TargetRayMode, i32);
            let args = (input_source_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9795usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_input_source_target_ray_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_visibility_state(&self,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9796usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_visibility_state", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_display_refresh_rate(&self,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9797usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_display_refresh_rate", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_display_refresh_rate(&mut self, refresh_rate: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), f32);
            let args = (refresh_rate,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9798usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_display_refresh_rate", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_available_display_refresh_rates(&self,) -> VariantArray {
            type RetMarshal = PtrcallReturnT < VariantArray >;
            type CallSig = (VariantArray,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9799usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_available_display_refresh_rates", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for WebXrInterface {
        type Base = crate::engine::XrInterface;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"WebXRInterface\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for WebXrInterface {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for WebXrInterface {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::XrInterface > for WebXrInterface {
        
    }
    impl crate::obj::Inherits < crate::engine::RefCounted > for WebXrInterface {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for WebXrInterface {
        
    }
    impl std::ops::Deref for WebXrInterface {
        type Target = crate::engine::XrInterface;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for WebXrInterface {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_WebXrInterface {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::WebXrInterface > for $Class {
                
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
pub struct TargetRayMode {
    ord: i32
}
impl TargetRayMode {
    pub const TARGET_RAY_MODE_UNKNOWN: Self = Self {
        ord: 0i32
    };
    pub const TARGET_RAY_MODE_GAZE: Self = Self {
        ord: 1i32
    };
    pub const TARGET_RAY_MODE_TRACKED_POINTER: Self = Self {
        ord: 2i32
    };
    pub const TARGET_RAY_MODE_SCREEN: Self = Self {
        ord: 3i32
    };
    
}
impl crate::obj::EngineEnum for TargetRayMode {
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
impl crate::builtin::meta::GodotConvert for TargetRayMode {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for TargetRayMode {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for TargetRayMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}