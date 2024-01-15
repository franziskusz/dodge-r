#![doc = "Sidecar module for class [`SliderJoint3D`][crate::engine::SliderJoint3D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `SliderJoint3D` enums](https://docs.godotengine.org/en/stable/classes/class_sliderjoint3d.html#enumerations).\n\n"]
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
    #[doc = "Godot class `SliderJoint3D.`\n\nInherits [`Joint3D`][crate::engine::Joint3D].\n\nRelated symbols:\n\n* [`slider_joint_3d`][crate::engine::slider_joint_3d]: sidecar module with related enum/flag types\n* [`ISliderJoint3D`][crate::engine::ISliderJoint3D]: virtual methods\n\n\nSee also [Godot docs for `SliderJoint3D`](https://docs.godotengine.org/en/stable/classes/class_sliderjoint3d.html).\n\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct SliderJoint3D {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`SliderJoint3D`][crate::engine::SliderJoint3D].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `SliderJoint3D` methods](https://docs.godotengine.org/en/stable/classes/class_sliderjoint3d.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ISliderJoint3D: crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api {
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
        fn on_notification(&mut self, what: Node3DNotification) {
            unimplemented !()
        }
        fn process(&mut self, delta: f64,) {
            unimplemented !()
        }
        fn physics_process(&mut self, delta: f64,) {
            unimplemented !()
        }
        fn enter_tree(&mut self,) {
            unimplemented !()
        }
        fn exit_tree(&mut self,) {
            unimplemented !()
        }
        fn ready(&mut self,) {
            unimplemented !()
        }
        fn get_configuration_warnings(&self,) -> PackedStringArray {
            unimplemented !()
        }
        fn input(&mut self, event: Gd < crate::engine::InputEvent >,) {
            unimplemented !()
        }
        fn shortcut_input(&mut self, event: Gd < crate::engine::InputEvent >,) {
            unimplemented !()
        }
        fn unhandled_input(&mut self, event: Gd < crate::engine::InputEvent >,) {
            unimplemented !()
        }
        fn unhandled_key_input(&mut self, event: Gd < crate::engine::InputEvent >,) {
            unimplemented !()
        }
    }
    impl SliderJoint3D {
        pub fn set_param(&mut self, param: crate::engine::slider_joint_3d::Param, value: f32,) {
            type RetMarshal = PtrcallReturnUnit;
            type CallSig = ((), crate::engine::slider_joint_3d::Param, f32);
            let args = (param, value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7649usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "set_param", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_param(&self, param: crate::engine::slider_joint_3d::Param,) -> f32 {
            type RetMarshal = PtrcallReturnT < f32 >;
            type CallSig = (f32, crate::engine::slider_joint_3d::Param);
            let args = (param,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7650usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall::< RetMarshal > (method_bind, "get_param", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for SliderJoint3D {
        type Base = crate::engine::Joint3D;
        fn class_name() -> ClassName {
            ClassName::from_ascii_cstr(b"SliderJoint3D\0")
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for SliderJoint3D {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        
    }
    impl crate::obj::EngineClass for SliderJoint3D {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of !(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits < crate::engine::Joint3D > for SliderJoint3D {
        
    }
    impl crate::obj::Inherits < crate::engine::Node3D > for SliderJoint3D {
        
    }
    impl crate::obj::Inherits < crate::engine::Node > for SliderJoint3D {
        
    }
    impl crate::obj::Inherits < crate::engine::Object > for SliderJoint3D {
        
    }
    impl crate::obj::ExportableObject for SliderJoint3D {
        
    }
    impl crate::obj::cap::GodotDefault for SliderJoint3D {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::engine::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for SliderJoint3D {
        type Target = crate::engine::Joint3D;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for SliderJoint3D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherits_transitive_SliderJoint3D {
        ($Class: ident) => {
            impl::godot::obj::Inherits < ::godot::engine::SliderJoint3D > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::Joint3D > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::Node3D > for $Class {
                
            }
            impl::godot::obj::Inherits < ::godot::engine::Node > for $Class {
                
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
pub struct Param {
    ord: i32
}
impl Param {
    pub const PARAM_LINEAR_LIMIT_UPPER: Self = Self {
        ord: 0i32
    };
    pub const PARAM_LINEAR_LIMIT_LOWER: Self = Self {
        ord: 1i32
    };
    pub const PARAM_LINEAR_LIMIT_SOFTNESS: Self = Self {
        ord: 2i32
    };
    pub const PARAM_LINEAR_LIMIT_RESTITUTION: Self = Self {
        ord: 3i32
    };
    pub const PARAM_LINEAR_LIMIT_DAMPING: Self = Self {
        ord: 4i32
    };
    pub const PARAM_LINEAR_MOTION_SOFTNESS: Self = Self {
        ord: 5i32
    };
    pub const PARAM_LINEAR_MOTION_RESTITUTION: Self = Self {
        ord: 6i32
    };
    pub const PARAM_LINEAR_MOTION_DAMPING: Self = Self {
        ord: 7i32
    };
    pub const PARAM_LINEAR_ORTHOGONAL_SOFTNESS: Self = Self {
        ord: 8i32
    };
    pub const PARAM_LINEAR_ORTHOGONAL_RESTITUTION: Self = Self {
        ord: 9i32
    };
    pub const PARAM_LINEAR_ORTHOGONAL_DAMPING: Self = Self {
        ord: 10i32
    };
    pub const PARAM_ANGULAR_LIMIT_UPPER: Self = Self {
        ord: 11i32
    };
    pub const PARAM_ANGULAR_LIMIT_LOWER: Self = Self {
        ord: 12i32
    };
    pub const PARAM_ANGULAR_LIMIT_SOFTNESS: Self = Self {
        ord: 13i32
    };
    pub const PARAM_ANGULAR_LIMIT_RESTITUTION: Self = Self {
        ord: 14i32
    };
    pub const PARAM_ANGULAR_LIMIT_DAMPING: Self = Self {
        ord: 15i32
    };
    pub const PARAM_ANGULAR_MOTION_SOFTNESS: Self = Self {
        ord: 16i32
    };
    pub const PARAM_ANGULAR_MOTION_RESTITUTION: Self = Self {
        ord: 17i32
    };
    pub const PARAM_ANGULAR_MOTION_DAMPING: Self = Self {
        ord: 18i32
    };
    pub const PARAM_ANGULAR_ORTHOGONAL_SOFTNESS: Self = Self {
        ord: 19i32
    };
    pub const PARAM_ANGULAR_ORTHOGONAL_RESTITUTION: Self = Self {
        ord: 20i32
    };
    pub const PARAM_ANGULAR_ORTHOGONAL_DAMPING: Self = Self {
        ord: 21i32
    };
    pub const PARAM_MAX: Self = Self {
        ord: 22i32
    };
    
}
impl crate::obj::EngineEnum for Param {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 | ord @ 9i32 | ord @ 10i32 | ord @ 11i32 | ord @ 12i32 | ord @ 13i32 | ord @ 14i32 | ord @ 15i32 | ord @ 16i32 | ord @ 17i32 | ord @ 18i32 | ord @ 19i32 | ord @ 20i32 | ord @ 21i32 | ord @ 22i32 => Some(Self {
                ord
            }), _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
impl crate::obj::IndexEnum for Param {
    const ENUMERATOR_COUNT: usize = 22usize;
    
}
impl crate::builtin::meta::GodotConvert for Param {
    type Via = i32;
    
}
impl crate::builtin::meta::ToGodot for Param {
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::builtin::meta::FromGodot for Param {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::builtin::meta::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::builtin::meta::FromGodotError::InvalidEnum.into_error(via))
    }
}