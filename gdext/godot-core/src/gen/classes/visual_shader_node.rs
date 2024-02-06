#![doc = "Sidecar module for class [`VisualShaderNode`][crate::engine::VisualShaderNode].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `VisualShaderNode` enums](https://docs.godotengine.org/en/stable/classes/class_visualshadernode.html#enumerations).\n\n"]
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
    #[doc = "Godot class `VisualShaderNode.`\n\nInherits [`Resource`][crate::engine::Resource].\n\nRelated symbols:\n\n* [`visual_shader_node`][crate::engine::visual_shader_node]: sidecar module with related enum/flag types\n* [`IVisualShaderNode`][crate::engine::IVisualShaderNode]: virtual methods\n\n\nSee also [Godot docs for `VisualShaderNode`](https://docs.godotengine.org/en/stable/classes/class_visualshadernode.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct VisualShaderNode {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`VisualShaderNode`][crate::engine::VisualShaderNode].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `VisualShaderNode` methods](https://docs.godotengine.org/en/stable/classes/class_visualshadernode.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IVisualShaderNode: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
    impl VisualShaderNode {
        pub fn get_default_input_port(&self, type_: crate::engine::visual_shader_node::PortType,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32, crate::engine::visual_shader_node::PortType);
            let args = (type_,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9444usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_default_input_port", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_output_port_for_preview(&mut self, port: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (port,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9445usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_output_port_for_preview", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_output_port_for_preview(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9446usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_output_port_for_preview", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn set_input_port_default_value_full(&mut self, port: i32, value: Variant, prev_value: Variant,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32, Variant, Variant);
            let args = (port, value, prev_value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9447usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_input_port_default_value", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[inline]
        pub fn set_input_port_default_value(&mut self, port: i32, value: Variant,) {
            self.set_input_port_default_value_ex(port, value,) . done()
        }
        #[inline]
        pub fn set_input_port_default_value_ex(&mut self, port: i32, value: Variant,) -> ExSetInputPortDefaultValue < '_ > {
            ExSetInputPortDefaultValue::new(self, port, value,)
        }
        pub fn get_input_port_default_value(&self, port: i32,) -> Variant {
            type RetMarshal = PtrcallReturnT < Variant >;
            type CallSig = (Variant, i32);
            let args = (port,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9448usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_input_port_default_value", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn remove_input_port_default_value(&mut self, port: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (port,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9449usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "remove_input_port_default_value", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn clear_default_input_values(&mut self,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9450usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "clear_default_input_values", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_default_input_values(&mut self, values: VariantArray,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), VariantArray);
            let args = (values,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9451usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_default_input_values", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_default_input_values(&self,) -> VariantArray {
            type RetMarshal = PtrcallReturnT < VariantArray >;
            type CallSig = (VariantArray,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9452usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_default_input_values", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for VisualShaderNode {
        type Base = crate::engine::Resource;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"VisualShaderNode\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for VisualShaderNode {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for VisualShaderNode {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::Resource > for VisualShaderNode {
        
    }
    impl crate::obj::Inherits < crate::engine::RefCounted > for VisualShaderNode {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for VisualShaderNode {
        
    }
    impl crate::obj::ExportableObject for VisualShaderNode {
        
    }
    impl std::ops::Deref for VisualShaderNode {
        type Target = crate::engine::Resource;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for VisualShaderNode {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_VisualShaderNode {
        ($Class: ident) => {
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
#[doc = "Default-param extender for [`VisualShaderNode::set_input_port_default_value_ex`][super::VisualShaderNode::set_input_port_default_value_ex]."]
#[must_use]
pub struct ExSetInputPortDefaultValue < 'a > {
    surround_object: &'a mut re_export::VisualShaderNode, port: i32, value: Variant, prev_value: Variant,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExSetInputPortDefaultValue < 'a > {
    fn new(surround_object: &'a mut re_export::VisualShaderNode, port: i32, value: Variant,) -> Self {
        Self {
            surround_object, port, value, prev_value: Variant::nil(),
        }
    }
    #[inline]
    pub fn prev_value(self, value: Variant) -> Self {
        Self {
            prev_value: value, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        re_export::VisualShaderNode::set_input_port_default_value_full(self.surround_object, self.port, self.value, self.prev_value,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct PortType {
    ord: i32
}
impl PortType {
    pub const PORT_TYPE_SCALAR: Self = Self {
        ord: 0i32
    };
    pub const PORT_TYPE_SCALAR_INT: Self = Self {
        ord: 1i32
    };
    pub const PORT_TYPE_SCALAR_UINT: Self = Self {
        ord: 2i32
    };
    pub const PORT_TYPE_VECTOR_2D: Self = Self {
        ord: 3i32
    };
    pub const PORT_TYPE_VECTOR_3D: Self = Self {
        ord: 4i32
    };
    pub const PORT_TYPE_VECTOR_4D: Self = Self {
        ord: 5i32
    };
    pub const PORT_TYPE_BOOLEAN: Self = Self {
        ord: 6i32
    };
    pub const PORT_TYPE_TRANSFORM: Self = Self {
        ord: 7i32
    };
    pub const PORT_TYPE_SAMPLER: Self = Self {
        ord: 8i32
    };
    pub const PORT_TYPE_MAX: Self = Self {
        ord: 9i32
    };
    
}
impl crate::obj::EngineEnum for PortType {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 | ord @ 9i32 => Some(Self {
                ord
            }), _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
impl crate::obj::IndexEnum for PortType {
    const ENUMERATOR_COUNT: usize = 9usize;
    
}
impl crate::builtin::meta::GodotConvert for PortType {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for PortType {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for PortType {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}