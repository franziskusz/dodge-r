#![doc = "Sidecar module for class [`ScriptExtension`][crate::engine::ScriptExtension].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `ScriptExtension` enums](https://docs.godotengine.org/en/stable/classes/class_scriptextension.html#enumerations).\n\n"]
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
    #[doc = "Godot class `ScriptExtension.`\n\nInherits [`Script`][crate::engine::Script].\n\nRelated symbols:\n\n* [`IScriptExtension`][crate::engine::IScriptExtension]: virtual methods\n\n\nSee also [Godot docs for `ScriptExtension`](https://docs.godotengine.org/en/stable/classes/class_scriptextension.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct ScriptExtension {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`ScriptExtension`][crate::engine::ScriptExtension].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `ScriptExtension` methods](https://docs.godotengine.org/en/stable/classes/class_scriptextension.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IScriptExtension: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
        fn editor_can_reload_from_file(&mut self,) -> bool {
            unimplemented !()
        }
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" Godot currently does not document safety requirements on this method. Make sure you understand the underlying semantics."]
        unsafe fn placeholder_erased(&mut self, placeholder: * mut c_void,) {
            unimplemented !()
        }
        fn can_instantiate(&self,) -> bool {
            unimplemented !()
        }
        fn get_base_script(&self,) -> Option < Gd < crate::engine::Script > > {
            unimplemented !()
        }
        fn get_global_name(&self,) -> StringName {
            unimplemented !()
        }
        fn inherits_script(&self, script: Gd < crate::engine::Script >,) -> bool {
            unimplemented !()
        }
        fn get_instance_base_type(&self,) -> StringName {
            unimplemented !()
        }
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" Godot currently does not document safety requirements on this method. Make sure you understand the underlying semantics."]
        unsafe fn instance_create(&self, for_object: Gd < crate::engine::Object >,) -> * mut c_void {
            unimplemented !()
        }
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" Godot currently does not document safety requirements on this method. Make sure you understand the underlying semantics."]
        unsafe fn placeholder_instance_create(&self, for_object: Gd < crate::engine::Object >,) -> * mut c_void {
            unimplemented !()
        }
        fn instance_has(&self, object: Gd < crate::engine::Object >,) -> bool {
            unimplemented !()
        }
        fn has_source_code(&self,) -> bool {
            unimplemented !()
        }
        fn get_source_code(&self,) -> GString {
            unimplemented !()
        }
        fn set_source_code(&mut self, code: GString,) {
            unimplemented !()
        }
        fn reload(&mut self, keep_state: bool,) -> crate::engine::global::Error {
            unimplemented !()
        }
        fn get_documentation(&self,) -> Array < Dictionary > {
            unimplemented !()
        }
        fn get_class_icon_path(&self,) -> GString {
            unimplemented !()
        }
        fn has_method(&self, method: StringName,) -> bool {
            unimplemented !()
        }
        fn has_static_method(&self, method: StringName,) -> bool {
            unimplemented !()
        }
        fn get_method_info(&self, method: StringName,) -> Dictionary {
            unimplemented !()
        }
        fn is_tool(&self,) -> bool {
            unimplemented !()
        }
        fn is_valid(&self,) -> bool {
            unimplemented !()
        }
        fn is_abstract(&self,) -> bool {
            unimplemented !()
        }
        fn get_language(&self,) -> Option < Gd < crate::engine::ScriptLanguage > > {
            unimplemented !()
        }
        fn has_script_signal(&self, signal: StringName,) -> bool {
            unimplemented !()
        }
        fn get_script_signal_list(&self,) -> Array < Dictionary > {
            unimplemented !()
        }
        fn has_property_default_value(&self, property: StringName,) -> bool {
            unimplemented !()
        }
        fn get_property_default_value(&self, property: StringName,) -> Variant {
            unimplemented !()
        }
        fn update_exports(&mut self,) {
            unimplemented !()
        }
        fn get_script_method_list(&self,) -> Array < Dictionary > {
            unimplemented !()
        }
        fn get_script_property_list(&self,) -> Array < Dictionary > {
            unimplemented !()
        }
        fn get_member_line(&self, member: StringName,) -> i32 {
            unimplemented !()
        }
        fn get_constants(&self,) -> Dictionary {
            unimplemented !()
        }
        fn get_members(&self,) -> Array < StringName > {
            unimplemented !()
        }
        fn is_placeholder_fallback_enabled(&self,) -> bool {
            unimplemented !()
        }
        fn get_rpc_config(&self,) -> Variant {
            unimplemented !()
        }
        fn setup_local_to_scene(&mut self,) {
            unimplemented !()
        }
    }
    impl ScriptExtension {
        #[deprecated = "Replaced with `new_gd` in extension trait `NewGd`."]
        pub fn new() -> Gd < Self > {
            crate::obj::Gd::default()
        }
        fn __checked_id(&self) -> Option < crate::obj::InstanceId > {
            let rtti = unsafe {
                self.rtti.as_ref() . unwrap_unchecked()
            };
            let instance_id = rtti.check_type::< Self > ();
            Some(instance_id)
        }
    }
    impl crate::obj::GodotClass for ScriptExtension {
        type Base = crate::engine::Script;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"ScriptExtension\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for ScriptExtension {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for ScriptExtension {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::Script > for ScriptExtension {
        
    }
    impl crate::obj::Inherits < crate::engine::Resource > for ScriptExtension {
        
    }
    impl crate::obj::Inherits < crate::engine::RefCounted > for ScriptExtension {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for ScriptExtension {
        
    }
    impl crate::obj::ExportableObject for ScriptExtension {
        
    }
    impl crate::obj::cap::GodotDefault for ScriptExtension {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for ScriptExtension {
        type Target = crate::engine::Script;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for ScriptExtension {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_ScriptExtension {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::ScriptExtension > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::Script > for $Class {
                
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