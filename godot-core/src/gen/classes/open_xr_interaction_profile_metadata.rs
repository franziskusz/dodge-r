#![doc = "Sidecar module for class [`OpenXrInteractionProfileMetadata`][crate::engine::OpenXrInteractionProfileMetadata].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `OpenXRInteractionProfileMetadata` enums](https://docs.godotengine.org/en/stable/classes/class_openxrinteractionprofilemetadata.html#enumerations).\n\n"]
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
    #[doc = "Godot class `OpenXRInteractionProfileMetadata.`\n\nInherits [`Object`][crate::engine::Object].\n\nRelated symbols:\n\n* [`IOpenXrInteractionProfileMetadata`][crate::engine::IOpenXrInteractionProfileMetadata]: virtual methods\n\n\nSee also [Godot docs for `OpenXRInteractionProfileMetadata`](https://docs.godotengine.org/en/stable/classes/class_openxrinteractionprofilemetadata.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct OpenXrInteractionProfileMetadata {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`OpenXrInteractionProfileMetadata`][crate::engine::OpenXrInteractionProfileMetadata].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `OpenXRInteractionProfileMetadata` methods](https://docs.godotengine.org/en/stable/classes/class_openxrinteractionprofilemetadata.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IOpenXrInteractionProfileMetadata: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
    impl OpenXrInteractionProfileMetadata {
        pub fn register_profile_rename(&mut self, old_name: GString, new_name: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString, GString);
            let args = (old_name, new_name,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5339usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "register_profile_rename", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn register_top_level_path(&mut self, display_name: GString, openxr_path: GString, openxr_extension_name: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString, GString, GString);
            let args = (display_name, openxr_path, openxr_extension_name,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5340usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "register_top_level_path", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn register_interaction_profile(&mut self, display_name: GString, openxr_path: GString, openxr_extension_name: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString, GString, GString);
            let args = (display_name, openxr_path, openxr_extension_name,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5341usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "register_interaction_profile", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn register_io_path(&mut self, interaction_profile: GString, display_name: GString, toplevel_path: GString, openxr_path: GString, openxr_extension_name: GString, action_type: crate::engine::open_xr_action::ActionType,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString, GString, GString, GString, GString, crate::engine::open_xr_action::ActionType);
            let args = (interaction_profile, display_name, toplevel_path, openxr_path, openxr_extension_name, action_type,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5342usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "register_io_path", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for OpenXrInteractionProfileMetadata {
        type Base = crate::engine::Object;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"OpenXRInteractionProfileMetadata\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for OpenXrInteractionProfileMetadata {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for OpenXrInteractionProfileMetadata {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::Object > for OpenXrInteractionProfileMetadata {
        
    }
    impl crate::obj::cap::GodotDefault for OpenXrInteractionProfileMetadata {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for OpenXrInteractionProfileMetadata {
        type Target = crate::engine::Object;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for OpenXrInteractionProfileMetadata {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_OpenXrInteractionProfileMetadata {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::OpenXrInteractionProfileMetadata > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::Object > for $Class {
                
            }
        }
    }
}