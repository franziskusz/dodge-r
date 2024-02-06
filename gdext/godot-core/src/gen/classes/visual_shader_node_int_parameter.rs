#![doc = "Sidecar module for class [`VisualShaderNodeIntParameter`][crate::engine::VisualShaderNodeIntParameter].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `VisualShaderNodeIntParameter` enums](https://docs.godotengine.org/en/stable/classes/class_visualshadernodeintparameter.html#enumerations).\n\n"]
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
    #[doc = "Godot class `VisualShaderNodeIntParameter.`\n\nInherits [`VisualShaderNodeParameter`][crate::engine::VisualShaderNodeParameter].\n\nRelated symbols:\n\n* [`visual_shader_node_int_parameter`][crate::engine::visual_shader_node_int_parameter]: sidecar module with related enum/flag types\n* [`IVisualShaderNodeIntParameter`][crate::engine::IVisualShaderNodeIntParameter]: virtual methods\n\n\nSee also [Godot docs for `VisualShaderNodeIntParameter`](https://docs.godotengine.org/en/stable/classes/class_visualshadernodeintparameter.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct VisualShaderNodeIntParameter {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`VisualShaderNodeIntParameter`][crate::engine::VisualShaderNodeIntParameter].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `VisualShaderNodeIntParameter` methods](https://docs.godotengine.org/en/stable/classes/class_visualshadernodeintparameter.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IVisualShaderNodeIntParameter: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
    impl VisualShaderNodeIntParameter {
        #[deprecated = "Replaced with `new_gd` in extension trait `NewGd`."]
        pub fn new() -> Gd < Self > {
            crate::obj::Gd::default()
        }
        pub fn set_hint(&mut self, hint: crate::engine::visual_shader_node_int_parameter::Hint,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::visual_shader_node_int_parameter::Hint);
            let args = (hint,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9552usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_hint", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_hint(&self,) -> crate::engine::visual_shader_node_int_parameter::Hint {
            type RetMarshal = PtrcallReturnT < crate::engine::visual_shader_node_int_parameter::Hint >;
            type CallSig = (crate::engine::visual_shader_node_int_parameter::Hint,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9553usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_hint", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_min(&mut self, value: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9554usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_min", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_min(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9555usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_min", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_max(&mut self, value: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9556usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_max", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_max(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9557usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_max", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_step(&mut self, value: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9558usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_step", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_step(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9559usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_step", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_default_value_enabled(&mut self, enabled: bool,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), bool);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9560usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_default_value_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_default_value_enabled(&self,) -> bool {
            type RetMarshal = PtrcallReturnT < bool >;
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9561usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "is_default_value_enabled", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_default_value(&mut self, value: i32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), i32);
            let args = (value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9562usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_default_value", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_default_value(&self,) -> i32 {
            type RetMarshal = PtrcallReturnT < i32 >;
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9563usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_default_value", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for VisualShaderNodeIntParameter {
        type Base = crate::engine::VisualShaderNodeParameter;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"VisualShaderNodeIntParameter\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for VisualShaderNodeIntParameter {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for VisualShaderNodeIntParameter {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::VisualShaderNodeParameter > for VisualShaderNodeIntParameter {
        
    }
    impl crate::obj::Inherits < crate::engine::VisualShaderNode > for VisualShaderNodeIntParameter {
        
    }
    impl crate::obj::Inherits < crate::engine::Resource > for VisualShaderNodeIntParameter {
        
    }
    impl crate::obj::Inherits < crate::engine::RefCounted > for VisualShaderNodeIntParameter {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for VisualShaderNodeIntParameter {
        
    }
    impl crate::obj::ExportableObject for VisualShaderNodeIntParameter {
        
    }
    impl crate::obj::cap::GodotDefault for VisualShaderNodeIntParameter {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for VisualShaderNodeIntParameter {
        type Target = crate::engine::VisualShaderNodeParameter;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for VisualShaderNodeIntParameter {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_VisualShaderNodeIntParameter {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::VisualShaderNodeIntParameter > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::VisualShaderNodeParameter > for $Class {
                
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
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Hint {
    ord: i32
}
impl Hint {
    pub const HINT_NONE: Self = Self {
        ord: 0i32
    };
    pub const HINT_RANGE: Self = Self {
        ord: 1i32
    };
    pub const HINT_RANGE_STEP: Self = Self {
        ord: 2i32
    };
    pub const HINT_MAX: Self = Self {
        ord: 3i32
    };
    
}
impl crate::obj::EngineEnum for Hint {
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
impl crate::obj::IndexEnum for Hint {
    const ENUMERATOR_COUNT: usize = 3usize;
    
}
impl crate::builtin::meta::GodotConvert for Hint {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for Hint {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for Hint {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}