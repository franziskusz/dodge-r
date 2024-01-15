#![doc = "Sidecar module for class [`OpenXrExtensionWrapperExtension`][crate::engine::OpenXrExtensionWrapperExtension].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `OpenXRExtensionWrapperExtension` enums](https://docs.godotengine.org/en/stable/classes/class_openxrextensionwrapperextension.html#enumerations).\n\n"]
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
    #[doc = "Godot class `OpenXRExtensionWrapperExtension.`\n\nInherits [`Object`][crate::engine::Object].\n\nRelated symbols:\n\n* [`IOpenXrExtensionWrapperExtension`][crate::engine::IOpenXrExtensionWrapperExtension]: virtual methods\n\n\nSee also [Godot docs for `OpenXRExtensionWrapperExtension`](https://docs.godotengine.org/en/stable/classes/class_openxrextensionwrapperextension.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct OpenXrExtensionWrapperExtension {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`OpenXrExtensionWrapperExtension`][crate::engine::OpenXrExtensionWrapperExtension].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `OpenXRExtensionWrapperExtension` methods](https://docs.godotengine.org/en/stable/classes/class_openxrextensionwrapperextension.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IOpenXrExtensionWrapperExtension: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
        fn get_requested_extensions(&mut self,) -> Dictionary {
            unimplemented !()
        }
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" Godot currently does not document safety requirements on this method. Make sure you understand the underlying semantics."]
        unsafe fn set_system_properties_and_get_next_pointer(&mut self, next_pointer: * mut c_void,) -> u64 {
            unimplemented !()
        }
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" Godot currently does not document safety requirements on this method. Make sure you understand the underlying semantics."]
        unsafe fn set_instance_create_info_and_get_next_pointer(&mut self, next_pointer: * mut c_void,) -> u64 {
            unimplemented !()
        }
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" Godot currently does not document safety requirements on this method. Make sure you understand the underlying semantics."]
        unsafe fn set_session_create_and_get_next_pointer(&mut self, next_pointer: * mut c_void,) -> u64 {
            unimplemented !()
        }
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" Godot currently does not document safety requirements on this method. Make sure you understand the underlying semantics."]
        unsafe fn set_swapchain_create_info_and_get_next_pointer(&mut self, next_pointer: * mut c_void,) -> u64 {
            unimplemented !()
        }
        fn on_register_metadata(&mut self,) {
            unimplemented !()
        }
        fn on_before_instance_created(&mut self,) {
            unimplemented !()
        }
        fn on_instance_created(&mut self, instance: u64,) {
            unimplemented !()
        }
        fn on_instance_destroyed(&mut self,) {
            unimplemented !()
        }
        fn on_session_created(&mut self, session: u64,) {
            unimplemented !()
        }
        fn on_process(&mut self,) {
            unimplemented !()
        }
        fn on_pre_render(&mut self,) {
            unimplemented !()
        }
        fn on_session_destroyed(&mut self,) {
            unimplemented !()
        }
        fn on_state_idle(&mut self,) {
            unimplemented !()
        }
        fn on_state_ready(&mut self,) {
            unimplemented !()
        }
        fn on_state_synchronized(&mut self,) {
            unimplemented !()
        }
        fn on_state_visible(&mut self,) {
            unimplemented !()
        }
        fn on_state_focused(&mut self,) {
            unimplemented !()
        }
        fn on_state_stopping(&mut self,) {
            unimplemented !()
        }
        fn on_state_loss_pending(&mut self,) {
            unimplemented !()
        }
        fn on_state_exiting(&mut self,) {
            unimplemented !()
        }
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" Godot currently does not document safety requirements on this method. Make sure you understand the underlying semantics."]
        unsafe fn on_event_polled(&mut self, event: * const c_void,) -> bool {
            unimplemented !()
        }
    }
    impl OpenXrExtensionWrapperExtension {
        pub fn get_openxr_api(&mut self,) -> Option < Gd < crate::engine::OpenXrApiExtension > > {
            type RetMarshal = PtrcallReturnOptionGdT < Gd < crate::engine::OpenXrApiExtension > >;
            type CallSig = (Option < Gd < crate::engine::OpenXrApiExtension > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5317usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_openxr_api", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn register_extension_wrapper(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5318usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "register_extension_wrapper", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for OpenXrExtensionWrapperExtension {
        type Base = crate::engine::Object;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"OpenXRExtensionWrapperExtension\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for OpenXrExtensionWrapperExtension {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for OpenXrExtensionWrapperExtension {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::Object > for OpenXrExtensionWrapperExtension {
        
    }
    impl crate::obj::cap::GodotDefault for OpenXrExtensionWrapperExtension {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for OpenXrExtensionWrapperExtension {
        type Target = crate::engine::Object;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for OpenXrExtensionWrapperExtension {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_OpenXrExtensionWrapperExtension {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::OpenXrExtensionWrapperExtension > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::Object > for $Class {
                
            }
        }
    }
}