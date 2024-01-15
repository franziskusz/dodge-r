#![doc = "Sidecar module for class [`VisualShaderNodeVectorFunc`][crate::engine::VisualShaderNodeVectorFunc].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `VisualShaderNodeVectorFunc` enums](https://docs.godotengine.org/en/stable/classes/class_visualshadernodevectorfunc.html#enumerations).\n\n"]
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
    #[doc = "Godot class `VisualShaderNodeVectorFunc.`\n\nInherits [`VisualShaderNodeVectorBase`][crate::engine::VisualShaderNodeVectorBase].\n\nRelated symbols:\n\n* [`visual_shader_node_vector_func`][crate::engine::visual_shader_node_vector_func]: sidecar module with related enum/flag types\n* [`IVisualShaderNodeVectorFunc`][crate::engine::IVisualShaderNodeVectorFunc]: virtual methods\n\n\nSee also [Godot docs for `VisualShaderNodeVectorFunc`](https://docs.godotengine.org/en/stable/classes/class_visualshadernodevectorfunc.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct VisualShaderNodeVectorFunc {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`VisualShaderNodeVectorFunc`][crate::engine::VisualShaderNodeVectorFunc].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `VisualShaderNodeVectorFunc` methods](https://docs.godotengine.org/en/stable/classes/class_visualshadernodevectorfunc.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IVisualShaderNodeVectorFunc: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
    impl VisualShaderNodeVectorFunc {
        #[deprecated = "Replaced with `new_gd` in extension trait `NewGd`."]
        pub fn new() -> Gd < Self > {
            crate::obj::Gd::default()
        }
        pub fn set_function(&mut self, func: crate::engine::visual_shader_node_vector_func::Function,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::visual_shader_node_vector_func::Function);
            let args = (func,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9670usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_function", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_function(&self,) -> crate::engine::visual_shader_node_vector_func::Function {
            type RetMarshal = PtrcallReturnT < crate::engine::visual_shader_node_vector_func::Function >;
            type CallSig = (crate::engine::visual_shader_node_vector_func::Function,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9671usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_function", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for VisualShaderNodeVectorFunc {
        type Base = crate::engine::VisualShaderNodeVectorBase;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"VisualShaderNodeVectorFunc\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for VisualShaderNodeVectorFunc {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for VisualShaderNodeVectorFunc {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::VisualShaderNodeVectorBase > for VisualShaderNodeVectorFunc {
        
    }
    impl crate::obj::Inherits < crate::engine::VisualShaderNode > for VisualShaderNodeVectorFunc {
        
    }
    impl crate::obj::Inherits < crate::engine::Resource > for VisualShaderNodeVectorFunc {
        
    }
    impl crate::obj::Inherits < crate::engine::RefCounted > for VisualShaderNodeVectorFunc {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for VisualShaderNodeVectorFunc {
        
    }
    impl crate::obj::ExportableObject for VisualShaderNodeVectorFunc {
        
    }
    impl crate::obj::cap::GodotDefault for VisualShaderNodeVectorFunc {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for VisualShaderNodeVectorFunc {
        type Target = crate::engine::VisualShaderNodeVectorBase;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for VisualShaderNodeVectorFunc {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_VisualShaderNodeVectorFunc {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::VisualShaderNodeVectorFunc > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::VisualShaderNodeVectorBase > for $Class {
                
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
pub struct Function {
    ord: i32
}
impl Function {
    pub const FUNC_NORMALIZE: Self = Self {
        ord: 0i32
    };
    pub const FUNC_SATURATE: Self = Self {
        ord: 1i32
    };
    pub const FUNC_NEGATE: Self = Self {
        ord: 2i32
    };
    pub const FUNC_RECIPROCAL: Self = Self {
        ord: 3i32
    };
    pub const FUNC_ABS: Self = Self {
        ord: 4i32
    };
    pub const FUNC_ACOS: Self = Self {
        ord: 5i32
    };
    pub const FUNC_ACOSH: Self = Self {
        ord: 6i32
    };
    pub const FUNC_ASIN: Self = Self {
        ord: 7i32
    };
    pub const FUNC_ASINH: Self = Self {
        ord: 8i32
    };
    pub const FUNC_ATAN: Self = Self {
        ord: 9i32
    };
    pub const FUNC_ATANH: Self = Self {
        ord: 10i32
    };
    pub const FUNC_CEIL: Self = Self {
        ord: 11i32
    };
    pub const FUNC_COS: Self = Self {
        ord: 12i32
    };
    pub const FUNC_COSH: Self = Self {
        ord: 13i32
    };
    pub const FUNC_DEGREES: Self = Self {
        ord: 14i32
    };
    pub const FUNC_EXP: Self = Self {
        ord: 15i32
    };
    pub const FUNC_EXP2: Self = Self {
        ord: 16i32
    };
    pub const FUNC_FLOOR: Self = Self {
        ord: 17i32
    };
    pub const FUNC_FRACT: Self = Self {
        ord: 18i32
    };
    pub const FUNC_INVERSE_SQRT: Self = Self {
        ord: 19i32
    };
    pub const FUNC_LOG: Self = Self {
        ord: 20i32
    };
    pub const FUNC_LOG2: Self = Self {
        ord: 21i32
    };
    pub const FUNC_RADIANS: Self = Self {
        ord: 22i32
    };
    pub const FUNC_ROUND: Self = Self {
        ord: 23i32
    };
    pub const FUNC_ROUNDEVEN: Self = Self {
        ord: 24i32
    };
    pub const FUNC_SIGN: Self = Self {
        ord: 25i32
    };
    pub const FUNC_SIN: Self = Self {
        ord: 26i32
    };
    pub const FUNC_SINH: Self = Self {
        ord: 27i32
    };
    pub const FUNC_SQRT: Self = Self {
        ord: 28i32
    };
    pub const FUNC_TAN: Self = Self {
        ord: 29i32
    };
    pub const FUNC_TANH: Self = Self {
        ord: 30i32
    };
    pub const FUNC_TRUNC: Self = Self {
        ord: 31i32
    };
    pub const FUNC_ONEMINUS: Self = Self {
        ord: 32i32
    };
    pub const FUNC_MAX: Self = Self {
        ord: 33i32
    };
    
}
impl crate::obj::EngineEnum for Function {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 | ord @ 9i32 | ord @ 10i32 | ord @ 11i32 | ord @ 12i32 | ord @ 13i32 | ord @ 14i32 | ord @ 15i32 | ord @ 16i32 | ord @ 17i32 | ord @ 18i32 | ord @ 19i32 | ord @ 20i32 | ord @ 21i32 | ord @ 22i32 | ord @ 23i32 | ord @ 24i32 | ord @ 25i32 | ord @ 26i32 | ord @ 27i32 | ord @ 28i32 | ord @ 29i32 | ord @ 30i32 | ord @ 31i32 | ord @ 32i32 | ord @ 33i32 => Some(Self {
                ord
            }), _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
impl crate::obj::IndexEnum for Function {
    const ENUMERATOR_COUNT: usize = 33usize;
    
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