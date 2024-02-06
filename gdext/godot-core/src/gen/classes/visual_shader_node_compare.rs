#![doc = "Sidecar module for class [`VisualShaderNodeCompare`][crate::engine::VisualShaderNodeCompare].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `VisualShaderNodeCompare` enums](https://docs.godotengine.org/en/stable/classes/class_visualshadernodecompare.html#enumerations).\n\n"]
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
    #[doc = "Godot class `VisualShaderNodeCompare.`\n\nInherits [`VisualShaderNode`][crate::engine::VisualShaderNode].\n\nRelated symbols:\n\n* [`visual_shader_node_compare`][crate::engine::visual_shader_node_compare]: sidecar module with related enum/flag types\n* [`IVisualShaderNodeCompare`][crate::engine::IVisualShaderNodeCompare]: virtual methods\n\n\nSee also [Godot docs for `VisualShaderNodeCompare`](https://docs.godotengine.org/en/stable/classes/class_visualshadernodecompare.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct VisualShaderNodeCompare {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`VisualShaderNodeCompare`][crate::engine::VisualShaderNodeCompare].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `VisualShaderNodeCompare` methods](https://docs.godotengine.org/en/stable/classes/class_visualshadernodecompare.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IVisualShaderNodeCompare: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
    impl VisualShaderNodeCompare {
        #[deprecated = "Replaced with `new_gd` in extension trait `NewGd`."]
        pub fn new() -> Gd < Self > {
            crate::obj::Gd::default()
        }
        pub fn set_comparison_type(&mut self, type_: crate::engine::visual_shader_node_compare::ComparisonType,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::visual_shader_node_compare::ComparisonType);
            let args = (type_,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9479usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_comparison_type", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_comparison_type(&self,) -> crate::engine::visual_shader_node_compare::ComparisonType {
            type RetMarshal = PtrcallReturnT < crate::engine::visual_shader_node_compare::ComparisonType >;
            type CallSig = (crate::engine::visual_shader_node_compare::ComparisonType,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9480usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_comparison_type", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_function(&mut self, func: crate::engine::visual_shader_node_compare::Function,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::visual_shader_node_compare::Function);
            let args = (func,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9481usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_function", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_function(&self,) -> crate::engine::visual_shader_node_compare::Function {
            type RetMarshal = PtrcallReturnT < crate::engine::visual_shader_node_compare::Function >;
            type CallSig = (crate::engine::visual_shader_node_compare::Function,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9482usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_function", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_condition(&mut self, condition: crate::engine::visual_shader_node_compare::Condition,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::visual_shader_node_compare::Condition);
            let args = (condition,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9483usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_condition", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_condition(&self,) -> crate::engine::visual_shader_node_compare::Condition {
            type RetMarshal = PtrcallReturnT < crate::engine::visual_shader_node_compare::Condition >;
            type CallSig = (crate::engine::visual_shader_node_compare::Condition,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9484usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_condition", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for VisualShaderNodeCompare {
        type Base = crate::engine::VisualShaderNode;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"VisualShaderNodeCompare\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for VisualShaderNodeCompare {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for VisualShaderNodeCompare {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::VisualShaderNode > for VisualShaderNodeCompare {
        
    }
    impl crate::obj::Inherits < crate::engine::Resource > for VisualShaderNodeCompare {
        
    }
    impl crate::obj::Inherits < crate::engine::RefCounted > for VisualShaderNodeCompare {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for VisualShaderNodeCompare {
        
    }
    impl crate::obj::ExportableObject for VisualShaderNodeCompare {
        
    }
    impl crate::obj::cap::GodotDefault for VisualShaderNodeCompare {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for VisualShaderNodeCompare {
        type Target = crate::engine::VisualShaderNode;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for VisualShaderNodeCompare {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_VisualShaderNodeCompare {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::VisualShaderNodeCompare > for $Class {
                
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
pub struct ComparisonType {
    ord: i32
}
impl ComparisonType {
    pub const CTYPE_SCALAR: Self = Self {
        ord: 0i32
    };
    pub const CTYPE_SCALAR_INT: Self = Self {
        ord: 1i32
    };
    pub const CTYPE_SCALAR_UINT: Self = Self {
        ord: 2i32
    };
    pub const CTYPE_VECTOR_2D: Self = Self {
        ord: 3i32
    };
    pub const CTYPE_VECTOR_3D: Self = Self {
        ord: 4i32
    };
    pub const CTYPE_VECTOR_4D: Self = Self {
        ord: 5i32
    };
    pub const CTYPE_BOOLEAN: Self = Self {
        ord: 6i32
    };
    pub const CTYPE_TRANSFORM: Self = Self {
        ord: 7i32
    };
    pub const CTYPE_MAX: Self = Self {
        ord: 8i32
    };
    
}
impl crate::obj::EngineEnum for ComparisonType {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 => Some(Self {
                ord
            }), _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
impl crate::obj::IndexEnum for ComparisonType {
    const ENUMERATOR_COUNT: usize = 8usize;
    
}
impl crate::builtin::meta::GodotConvert for ComparisonType {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for ComparisonType {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for ComparisonType {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Function {
    ord: i32
}
impl Function {
    pub const FUNC_EQUAL: Self = Self {
        ord: 0i32
    };
    pub const FUNC_NOT_EQUAL: Self = Self {
        ord: 1i32
    };
    pub const FUNC_GREATER_THAN: Self = Self {
        ord: 2i32
    };
    pub const FUNC_GREATER_THAN_EQUAL: Self = Self {
        ord: 3i32
    };
    pub const FUNC_LESS_THAN: Self = Self {
        ord: 4i32
    };
    pub const FUNC_LESS_THAN_EQUAL: Self = Self {
        ord: 5i32
    };
    pub const FUNC_MAX: Self = Self {
        ord: 6i32
    };
    
}
impl crate::obj::EngineEnum for Function {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 => Some(Self {
                ord
            }), _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
impl crate::obj::IndexEnum for Function {
    const ENUMERATOR_COUNT: usize = 6usize;
    
}
impl crate::builtin::meta::GodotConvert for Function {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for Function {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for Function {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Condition {
    ord: i32
}
impl Condition {
    pub const COND_ALL: Self = Self {
        ord: 0i32
    };
    pub const COND_ANY: Self = Self {
        ord: 1i32
    };
    pub const COND_MAX: Self = Self {
        ord: 2i32
    };
    
}
impl crate::obj::EngineEnum for Condition {
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
impl crate::obj::IndexEnum for Condition {
    const ENUMERATOR_COUNT: usize = 2usize;
    
}
impl crate::builtin::meta::GodotConvert for Condition {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for Condition {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for Condition {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}