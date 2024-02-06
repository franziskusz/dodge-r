#![doc = "Sidecar module for class [`VisualShaderNodeCustom`][crate::engine::VisualShaderNodeCustom].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `VisualShaderNodeCustom` enums](https://docs.godotengine.org/en/stable/classes/class_visualshadernodecustom.html#enumerations).\n\n"]
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
    #[doc = "Godot class `VisualShaderNodeCustom.`\n\nInherits [`VisualShaderNode`][crate::engine::VisualShaderNode].\n\nRelated symbols:\n\n* [`IVisualShaderNodeCustom`][crate::engine::IVisualShaderNodeCustom]: virtual methods\n\n\nSee also [Godot docs for `VisualShaderNodeCustom`](https://docs.godotengine.org/en/stable/classes/class_visualshadernodecustom.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct VisualShaderNodeCustom {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`VisualShaderNodeCustom`][crate::engine::VisualShaderNodeCustom].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `VisualShaderNodeCustom` methods](https://docs.godotengine.org/en/stable/classes/class_visualshadernodecustom.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IVisualShaderNodeCustom: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
        fn get_name(&self,) -> GString {
            unimplemented !()
        }
        fn get_description(&self,) -> GString {
            unimplemented !()
        }
        fn get_category(&self,) -> GString {
            unimplemented !()
        }
        fn get_return_icon_type(&self,) -> crate::engine::visual_shader_node::PortType {
            unimplemented !()
        }
        fn get_input_port_count(&self,) -> i32 {
            unimplemented !()
        }
        fn get_input_port_type(&self, port: i32,) -> crate::engine::visual_shader_node::PortType {
            unimplemented !()
        }
        fn get_input_port_name(&self, port: i32,) -> GString {
            unimplemented !()
        }
        fn get_input_port_default_value(&self, port: i32,) -> Variant {
            unimplemented !()
        }
        fn get_default_input_port(&self, type_: crate::engine::visual_shader_node::PortType,) -> i32 {
            unimplemented !()
        }
        fn get_output_port_count(&self,) -> i32 {
            unimplemented !()
        }
        fn get_output_port_type(&self, port: i32,) -> crate::engine::visual_shader_node::PortType {
            unimplemented !()
        }
        fn get_output_port_name(&self, port: i32,) -> GString {
            unimplemented !()
        }
        fn get_property_count(&self,) -> i32 {
            unimplemented !()
        }
        fn get_property_name(&self, index: i32,) -> GString {
            unimplemented !()
        }
        fn get_property_default_index(&self, index: i32,) -> i32 {
            unimplemented !()
        }
        fn get_property_options(&self, index: i32,) -> PackedStringArray {
            unimplemented !()
        }
        fn get_code(&self, input_vars: Array < GString >, output_vars: Array < GString >, mode: crate::engine::shader::Mode, type_: crate::engine::visual_shader::Type,) -> GString {
            unimplemented !()
        }
        fn get_func_code(&self, mode: crate::engine::shader::Mode, type_: crate::engine::visual_shader::Type,) -> GString {
            unimplemented !()
        }
        fn get_global_code(&self, mode: crate::engine::shader::Mode,) -> GString {
            unimplemented !()
        }
        fn is_highend(&self,) -> bool {
            unimplemented !()
        }
        fn is_available(&self, mode: crate::engine::shader::Mode, type_: crate::engine::visual_shader::Type,) -> bool {
            unimplemented !()
        }
        fn setup_local_to_scene(&mut self,) {
            unimplemented !()
        }
    }
    impl VisualShaderNodeCustom {
        #[deprecated = "Replaced with `new_gd` in extension trait `NewGd`."]
        pub fn new() -> Gd < Self > {
            crate::obj::Gd::default()
        }
        pub fn get_option_index(&self, option: i32,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, i32);
            let args = (option,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9495usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_option_index", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for VisualShaderNodeCustom {
        type Base = crate::engine::VisualShaderNode;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"VisualShaderNodeCustom\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for VisualShaderNodeCustom {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for VisualShaderNodeCustom {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::VisualShaderNode > for VisualShaderNodeCustom {
        
    }
    impl crate::obj::Inherits < crate::engine::Resource > for VisualShaderNodeCustom {
        
    }
    impl crate::obj::Inherits < crate::engine::RefCounted > for VisualShaderNodeCustom {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for VisualShaderNodeCustom {
        
    }
    impl crate::obj::ExportableObject for VisualShaderNodeCustom {
        
    }
    impl crate::obj::cap::GodotDefault for VisualShaderNodeCustom {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for VisualShaderNodeCustom {
        type Target = crate::engine::VisualShaderNode;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for VisualShaderNodeCustom {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_VisualShaderNodeCustom {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::VisualShaderNodeCustom > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::VisualShaderNode > for $Class {
                
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