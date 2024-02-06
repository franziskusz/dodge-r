#![doc = "Sidecar module for class [`VisualShaderNodeGroupBase`][crate::engine::VisualShaderNodeGroupBase].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `VisualShaderNodeGroupBase` enums](https://docs.godotengine.org/en/stable/classes/class_visualshadernodegroupbase.html#enumerations).\n\n"]
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
    #[doc = "Godot class `VisualShaderNodeGroupBase.`\n\nInherits [`VisualShaderNodeResizableBase`][crate::engine::VisualShaderNodeResizableBase].\n\nRelated symbols:\n\n* [`IVisualShaderNodeGroupBase`][crate::engine::IVisualShaderNodeGroupBase]: virtual methods\n\n\nSee also [Godot docs for `VisualShaderNodeGroupBase`](https://docs.godotengine.org/en/stable/classes/class_visualshadernodegroupbase.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct VisualShaderNodeGroupBase {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`VisualShaderNodeGroupBase`][crate::engine::VisualShaderNodeGroupBase].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `VisualShaderNodeGroupBase` methods](https://docs.godotengine.org/en/stable/classes/class_visualshadernodegroupbase.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IVisualShaderNodeGroupBase: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
    impl VisualShaderNodeGroupBase {
        pub fn set_inputs(&mut self, inputs: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString);
            let args = (inputs,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9522usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_inputs", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_inputs(&self,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9523usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_inputs", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_outputs(&mut self, outputs: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), GString);
            let args = (outputs,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9524usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_outputs", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_outputs(&self,) -> GString {
            type RetMarshal = PtrcallReturnT < GString >;
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9525usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_outputs", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_valid_port_name(&self, name: GString,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, GString);
            let args = (name,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9526usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_valid_port_name", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn add_input_port(&mut self, id: i32, type_: i32, name: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, i32, GString);
            let args = (id, type_, name,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9527usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_input_port", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn remove_input_port(&mut self, id: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9528usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "remove_input_port", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_input_port_count(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9529usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_input_port_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn has_input_port(&self, id: i32,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, i32);
            let args = (id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9530usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "has_input_port", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn clear_input_ports(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9531usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "clear_input_ports", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn add_output_port(&mut self, id: i32, type_: i32, name: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, i32, GString);
            let args = (id, type_, name,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9532usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "add_output_port", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn remove_output_port(&mut self, id: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9533usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "remove_output_port", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_output_port_count(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9534usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_output_port_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn has_output_port(&self, id: i32,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool, i32);
            let args = (id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9535usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "has_output_port", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn clear_output_ports(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9536usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "clear_output_ports", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_input_port_name(&mut self, id: i32, name: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, GString);
            let args = (id, name,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9537usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_input_port_name", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_input_port_type(&mut self, id: i32, type_: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, i32);
            let args = (id, type_,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9538usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_input_port_type", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_output_port_name(&mut self, id: i32, name: GString,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, GString);
            let args = (id, name,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9539usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_output_port_name", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_output_port_type(&mut self, id: i32, type_: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, i32);
            let args = (id, type_,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9540usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_output_port_type", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_free_input_port_id(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9541usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_free_input_port_id", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_free_output_port_id(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9542usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_free_output_port_id", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for VisualShaderNodeGroupBase {
        type Base = crate::engine::VisualShaderNodeResizableBase;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"VisualShaderNodeGroupBase\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for VisualShaderNodeGroupBase {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for VisualShaderNodeGroupBase {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::VisualShaderNodeResizableBase > for VisualShaderNodeGroupBase {
        
    }
    impl crate::obj::Inherits < crate::engine::VisualShaderNode > for VisualShaderNodeGroupBase {
        
    }
    impl crate::obj::Inherits < crate::engine::Resource > for VisualShaderNodeGroupBase {
        
    }
    impl crate::obj::Inherits < crate::engine::RefCounted > for VisualShaderNodeGroupBase {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for VisualShaderNodeGroupBase {
        
    }
    impl crate::obj::ExportableObject for VisualShaderNodeGroupBase {
        
    }
    impl std::ops::Deref for VisualShaderNodeGroupBase {
        type Target = crate::engine::VisualShaderNodeResizableBase;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for VisualShaderNodeGroupBase {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_VisualShaderNodeGroupBase {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::VisualShaderNodeGroupBase > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::VisualShaderNodeResizableBase > for $Class {
                
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